use super::board::{ Board , BOARD_SIZE };
//use std::io::Result as IOResult;

pub trait DisplayBoard {
  //fn new() -> Self;

  fn  display( &self , board: & Board ) -> ();
}

pub struct DisplayBoardToConsoleSimpleStyle {
}

impl DisplayBoard for DisplayBoardToConsoleSimpleStyle {
  //fn new() -> Self {
  //  DisplayBoardToConsoleSimpleStyle {}
  //}

  fn display( &self , board: & Board ) -> () {
    let zero: u8 = '0'.to_digit(10).unwrap() as u8;
    for lin in 0..BOARD_SIZE as usize {
      for col in 0..BOARD_SIZE as usize {
        let cel = board.cell_at( lin , col );
        print!( " {}",
          match cel.value {
            Some(v) => {
              // most outrageous way to convert # to '#'
              let str = format!("{}", zero + v);
              str.chars().next().unwrap()
              // other would have been to use a lookup table
            },
            None =>  ' ',
          });
      }
      println!();
    }
  }
}