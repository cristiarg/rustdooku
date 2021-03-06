/*
- a structure to represent a cell
- a structure to allow representing the board
- a series of solvers that apply different strategies to manipulate and solve the board
*/

extern crate core;
extern crate proc_macro;

mod cell;
mod board;
mod load;
mod display;
mod solve;

//use cell::Cell;
use load::LoadBoard;
use load::LoadBoardOneLinerSimpleFormat;
use display::DisplayBoard;
use display::DisplayBoardToConsoleSimpleStyle;
//use solve::SolveStrategy;

fn main() {
  let full_file_name = String::from( "/home/crs/proj/rustdooku/res/0.txt" );
  println!("full file name: {:?}", &full_file_name);
  let board_loader: LoadBoardOneLinerSimpleFormat = LoadBoard::new( full_file_name.clone() );
  let loader_result = board_loader.load();
  match &loader_result {
    Ok( b ) => {
      println!("board loaded correctly");
      let str = DisplayBoardToConsoleSimpleStyle::new(&b );
      let trt: &DisplayBoard = &str;
      //let board_displayer: DisplayBoardToConsoleSimpleStyle = DisplayBoard::new(&b);
      trt.display(/*&b*/);
    }
    Err( e ) => {
      println!("board failed to load correctly: {}" , e );
    }
  }

  //println!(": {:?}", cl);
  //let cl = Cell::empty();
  //println!("Cell: {:?}", cl);
}
