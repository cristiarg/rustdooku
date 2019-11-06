use std::fmt;

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq)]
pub struct Position {
  pub lin: usize,
  pub col: usize,
}


#[derive(Clone, Copy)]
pub struct Cell {
  pub value: Option< u8 >,
}

impl fmt::Debug for Cell {
  fn fmt( &self , formatter: &mut fmt::Formatter ) -> fmt::Result {
    //write!( formatter , "[ {} ]", match self.value {
    //    None => String::from("_"),
    //    Some(v) => v.to_string(),
    //  } )
    write!( formatter , "[ {} ]"
        , self.value.map_or_else(
            || String::from("_"), // None
            |v| v.to_string() ) )
  }
}

#[allow(dead_code)]
impl Cell {
  pub fn empty() -> Self {
    Cell { value: None }
  }

  pub fn new(v: u8 ) -> Self {
    debug_assert!( 1 <= v && v <= 9 );
    Cell { value: Some( v ) }
  }

  pub fn has_value(&self) -> bool {
    self.value.is_some()
  }

  pub fn set_value(&mut self, _v: u8 ) -> () {
    debug_assert!( 1 <= _v && _v <= 9 );
    self.value = Some( _v )
  }
}

#[test]
fn test_cell_01() {
  let mut cell_none = Cell::empty();
  assert!( !cell_none.has_value() );

  cell_none.value = match cell_none.value {
      None => Some( 7 ),
      Some(v) => { let new_v = v + 42; Some( new_v ) }, // would not be executed
    };
  assert!( cell_none.has_value() );
  assert!( cell_none.value == Some(7) );

  let mut cell_some = Cell::new( 5 );
  assert!( cell_some.has_value() );

  cell_some.value = match cell_some.has_value() {
      false => Some( 42 ),
      true => { let new_v = cell_some.value.unwrap() + 4; Some( new_v ) },
    };
  assert_eq!( cell_some.value , Some( 9 ) );

  match cell_some.value.as_mut() {
    None => {},
    Some(v) => *v = 3,
  };
  assert_eq!( cell_some.value , Some( 3 ) );
}

