use super::board::{ Board , BOARD_VEC_SIZE };
use std::io::prelude::*;
use std::io::Result as IOResult;
use std::io::Error as IOError;
use std::io::ErrorKind as IOErrorKind;
use std::io::{ BufReader };
use std::result::Result;
use std::fs::File;
use std::path::Path;

pub trait LoadBoard {
  fn new( file_name: String ) -> Self;

  fn load( &self ) -> IOResult<Board>; // Result<Box<Board>,>;
}

pub struct LoadBoardOneLinerSimpleFormat {
  file_name: String,
}

fn load_one_board(line: &String) -> Result< Board, String > {
  let mut board = Board::empty();
  for (idx, ch) in line.chars().into_iter().enumerate() {
    // check that the maximum number of values on a board
    match idx {
      idx if idx < BOARD_VEC_SIZE => {
        match ch {
          // only these chars are expected
          ch if '1' <= ch && ch <= '9' => {
            let digit: u8 = ch.to_digit( 10 ).unwrap() as u8;
            //board.value_array[ idx ].value = Some( digit )
            board.value_array[ idx ].set_value( digit );
          }
          // recognized empty'es
          ch if '.' == ch || '0' == ch => {
            // leave empty cell
          }
          // not recognized
          ch => {
            //panic!( "unrecognized character in input '{}'; only digits and '.' are supported" , ch );
            let mes: String = format!( "unrecognized character in input '{}'; only digits and '.' are supported" , ch );
            return Err( mes )
          }
        }
      },
      idx => {
        let mes = format!( "a maximum of {} elements are expected, found {}" , BOARD_VEC_SIZE , idx + 1 );
        return Err( mes )
      },
    }
  }
  Ok( board )
}

impl LoadBoard for LoadBoardOneLinerSimpleFormat {
  fn new( file_name: String ) -> LoadBoardOneLinerSimpleFormat {
    LoadBoardOneLinerSimpleFormat { file_name: file_name }
  }

  fn load(&self) -> IOResult<Board> {
    let file_path = Path::new( &self.file_name );
    let file = File::open( &file_path )?;
    let buf_read = BufReader::new( &file );

    for (_i, line) in buf_read.lines().into_iter().enumerate() {
      let line_str: &String = &line?;
      let new_board = load_one_board( line_str );
      match new_board {
        Ok( nb ) => {
          return Ok( nb );
        }
        Err( e ) => {
          return Err( IOError::new( IOErrorKind::Other , e ) );
        }
      }
    }

    Err( IOError::new( IOErrorKind::Other , "unspecified error" ) )
  }
}

