/*
- a structure to represent a cell
- a structure to allow representing the board
- a series of solvers that apply different strategies to manipulate and solve the board
*/

mod cell;
mod board;
mod load;

//use cell::Cell;
use load::LoadBoard;
use load::LoadBoardOneLinerSimpleFormat;

fn main() {
  let full_file_name = String::from( "/home/crs/proj/rustdooku/res/0.txt" );
  println!("full file name: {:?}", &full_file_name);
  let file_loader: LoadBoardOneLinerSimpleFormat = LoadBoard::new( full_file_name.clone() );
  let file_loader_result = file_loader.load();
  match file_loader_result {
    Ok( _ ) => {
      println!("board loaded correctly");
    }
    Err( e ) => {
      println!("board failed to load correctly: {}" , e );
    }
  }
  //println!(": {:?}", cl);
  //let cl = Cell::empty();
  //println!("Cell: {:?}", cl);
}
