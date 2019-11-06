use super::board::{ Board , BOARD_SIZE };
use super::cell::Cell;
use super::cell::Position;

/*
at a high level, solving is applying a set of strategies, repeatedly
in a round-robin-ish way until, either, puzzle is solved or out of
options

we'll try to provide a hind based solving, at each step,
the global logic will select a solving strategy and ask it about a hint
to fill in one cell
*/


/// Information about where from to add/change/remove what.
#[allow(dead_code)]
pub enum BoardAction {
  /// At `pos` add `value`.
  ValueAdd { pos: Position, value: u8 },
  /// From `pos`, remove `value`.
  /// Value may be redundant but it allows transformation
  /// for both back and forth.
  ValueRemove { pos: Position, value: u8 },
  ///
  HintAdd { pos: Position, value: u8 },
  HintRemove { pos: Position, value: u8 },
}

#[allow(dead_code)]
pub struct Hint {
  cell: Cell,
  pos: Position,
}

/// Implementors look at the board and try to apply
/// their internal logic into finding one more empty
/// cell to fill in
///
#[allow(dead_code)]
pub trait SolveStrategy {
  fn get_hint(&self, board: &Board ) -> Option< Hint >;
  fn get_name(&self) -> &String;
}

//
// solving strategy NakedSingle
//  - https://www.sadmansoftware.com/sudoku/nakedsingle.php
//

#[allow(dead_code)]
pub struct SolveStrategyNakedSingle {
  name: String,
  // here there be state for this solving strategy
}

#[allow(dead_code)]
impl SolveStrategyNakedSingle {
  // helper functions
  #[allow(unused_variables)]
  fn is_naked_single(&self, board: &Board, lin: usize, col: usize) -> bool {
    return false;
  }
}

#[allow(dead_code)]
impl SolveStrategy for SolveStrategyNakedSingle {
  #[allow(unused_variables)]
  fn get_hint(&self, board: &Board ) -> Option< Hint > {
    for lin in 0..BOARD_SIZE as usize {
      for col in 0..BOARD_SIZE as usize {
        let is = SolveStrategyNakedSingle::is_naked_single(self, board, lin, col);
      }
    }
    None
  }

  fn get_name(&self) -> &String {
    &self.name
  }
}