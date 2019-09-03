// use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct ExpandableVec<T> {
  pos: Vec<T>,
  neg: Vec<T>,
}



impl<T: std::default::Default + std::clone::Clone> ExpandableVec<T> {
  pub fn new() -> ExpandableVec<T> {
    ExpandableVec {
      pos: vec![],
      neg: vec![],
    }
  }

  fn at_vec(vec: &mut Vec<T>, index: usize) -> &mut T {
    let size = index + 1;
    if size as i64 - vec.len() as i64 > 0 {
      vec.resize(size, T::default());
    }

    &mut vec[index]
}


  pub fn at(&mut self, index: i64) -> &mut T {
    if index >= 0 {
      Self::at_vec(&mut self.pos, index as usize)
    } else {
      Self::at_vec(&mut self.neg, (-index) as usize)
    }
  }

  fn at_vec_im(vec: &Vec<T>, index: usize) -> T {
    let size = index + 1;
    if size as i64 - vec.len() as i64 > 0 {
      T::default()
    }
    else {
      let val = &vec[index];
      val.clone()
    }
  }

  pub fn at_im(&self, index: i64) -> T {
    if index >= 0 {
      Self::at_vec_im(&self.pos, index as usize)
    } else {
      Self::at_vec_im(&self.neg, (-index) as usize)
    }
  }
}
