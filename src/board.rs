use super::cell::*;

pub const GROUP_SIZE: usize = 3;
pub const BOARD_SIZE: usize = GROUP_SIZE * GROUP_SIZE;
pub const BOARD_VEC_SIZE: usize = BOARD_SIZE * BOARD_SIZE;

/*
Cell
\C| 0 1 2   3 4 5   6 7 8
L\+----------------------
0 | . . 1 | 7 . . | 5 . 9
1 | 5 7 3 | . 2 4 | 1 . 6
2 | 8 . . | 5 . 1 | . . 2
  | ------+-------+------
3 | 7 . . | 2 9 5 | . 1 8
4 | . . 9 | 4 . . | 3 . 5
5 | 6 5 2 | 8 . . | . . 7
  | ------+-------+------
6 | 4 6 5 | . 8 . | . 7 1
7 | . . . | 1 5 9 | . . 4
8 | 9 . 8 | . . 7 | . 5 3

Group
\C|   0       1       2  
L\+----------------------
  | . . 1 | 7 . . | 5 . 9
0 | 5 7 3 | . 2 4 | 1 . 6
  | 8 . . | 5 . 1 | . . 2
  | ------+-------+------
  | 7 . . | 2 9 5 | . 1 8
1 | . . 9 | 4 . . | 3 . 5
  | 6 5 2 | 8 . . | . . 7
  | ------+-------+------
  | 4 6 5 | . 8 . | . 7 1
2 | . . . | 1 5 9 | . . 4
  | 9 . 8 | . . 7 | . 5 3
*/

#[allow(dead_code)]
pub struct Board {
  pub value_array: [Cell; BOARD_VEC_SIZE ], 
    // line by line
    // last cell on the right on the first line:
    //    [ 0, BOARD_SIZE - 1 ] = [ ( 0 * BOARD_SIZE ) + ( BOARD_SIZE - 1 ) ]
    // first cell on the left on the last line:
    //    [ BOARD_SIZE - 1 , 0 ] = [ ( ( BOARD_SIZE - 1 ) * BOARD_SIZE ) + 0 ]
    // cell on lin/col
    //    [ lin , col ] = [ ( lin * BOARD_SIZE ) + ( col )
}

impl Board {
  pub fn empty() -> Self {
    Board { value_array: [ Cell::empty() ; BOARD_VEC_SIZE ] }
  }

  pub fn cell_at(&self, lin: usize, col: usize) -> &Cell {
    &self.value_array[ ( lin * BOARD_SIZE ) + col ]
  }

  pub fn group_for_position(pos: &Position) -> (Position, Position) {
    // upper left
    let lin_up = ( pos.lin / GROUP_SIZE ) * GROUP_SIZE;
    let lin_down = lin_up + GROUP_SIZE;
    // lower right
    let col_left = ( pos.col / GROUP_SIZE ) * GROUP_SIZE;
    let col_right = col_left + GROUP_SIZE;

    ( Position { lin: lin_up , col: col_left }
        , Position { lin: lin_down , col: col_right } )
  }
}

#[test]
fn test_group_for_position() {
  // Group
  // L\C|   0   |   1   |   2
  // ---+-------+-------+------
  //    | . . 1 | 7 . . | 5 . 9
  //  0 | 5 7 3 | . 2 4 | 1 . 6
  //    | 8 . . | 5 . 1 | . . 2
  // ---+ ------+-------+------
  //    | 7 . . | 2 9 5 | . 1 8
  //  1 | . . 9 | 4 . . | 3 . 5
  //    | 6 5 2 | 8 . . | . . 7
  // ---+ ------+-------+------
  //    | 4 6 5 | . 8 . | . 7 1
  //  2 | . . . | 1 5 9 | . . 4
  //    | 9 . 8 | . . 7 | . 5 3

  const UP_LEFT_UPPER_LEFT  : Position  = Position { lin: 0           , col: 0 };
  const UP_LEFT_LOWER_RITE  : Position  = Position { lin: GROUP_SIZE  , col: GROUP_SIZE };
  const UP_MIDL_UPPER_LEFT  : Position  = Position { lin: 0           , col: GROUP_SIZE };
  const UP_MIDL_LOWER_RITE  : Position  = Position { lin: GROUP_SIZE  , col: GROUP_SIZE * 2 };
  const UP_RITE_UPPER_LEFT  : Position  = Position { lin: 0           , col: GROUP_SIZE * 2 };
  const UP_RITE_LOWER_RITE  : Position  = Position { lin: GROUP_SIZE  , col: GROUP_SIZE * 3 };

  assert!( Board::group_for_position(&Position { lin: 0, col: 0 } ) == ( UP_LEFT_UPPER_LEFT , UP_LEFT_LOWER_RITE ) );
  assert!( Board::group_for_position(&Position { lin: 0, col: 2 } ) == ( UP_LEFT_UPPER_LEFT , UP_LEFT_LOWER_RITE ) );
  assert!( Board::group_for_position(&Position { lin: 2, col: 0 } ) == ( UP_LEFT_UPPER_LEFT , UP_LEFT_LOWER_RITE ) );
  assert!( Board::group_for_position(&Position { lin: 2, col: 2 } ) == ( UP_LEFT_UPPER_LEFT , UP_LEFT_LOWER_RITE ) );

  assert!( Board::group_for_position(&Position { lin: 0, col: 3 } ) == ( UP_MIDL_UPPER_LEFT , UP_MIDL_LOWER_RITE ) );
  assert!( Board::group_for_position(&Position { lin: 0, col: 5 } ) == ( UP_MIDL_UPPER_LEFT , UP_MIDL_LOWER_RITE ) );
  assert!( Board::group_for_position(&Position { lin: 2, col: 3 } ) == ( UP_MIDL_UPPER_LEFT , UP_MIDL_LOWER_RITE ) );
  assert!( Board::group_for_position(&Position { lin: 2, col: 5 } ) == ( UP_MIDL_UPPER_LEFT , UP_MIDL_LOWER_RITE ) );

  //assert!( Board::group_for_position(  &Position { lin: 3, col: 5 } )
  //    == ( UP_MIDL_UPPER_LEFT , UP_MIDL_LOWER_RITE ) );
}

