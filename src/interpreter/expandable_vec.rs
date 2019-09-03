// use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct ExpandableVec<T> {
  pos: Vec<T>,
  neg: Vec<T>,
}

fn at_vec<T>(vec: &mut Vec<T>, index: usize) -> &mut T 
where T : std::default::Default + std::clone::Clone {
  let size = index + 1;
  if size as i64 - vec.len() as i64 > 0 {
    vec.resize(size, T::default());
  }

  &mut vec[index]
}
impl<T: std::default::Default + std::clone::Clone> ExpandableVec<T> {

  pub fn new() -> ExpandableVec<T> {
    ExpandableVec {
      pos: vec![],
      neg: vec![],
    }
  }

  pub fn at(&mut self, index: i64) -> &mut T {
    if index >= 0 {
      at_vec(&mut self.pos, index as usize)
    } else {
      at_vec(&mut self.neg, (-index) as usize)
    }
  }
}
