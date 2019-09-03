use crate::parser::Token;
use crate::interpreter::expandable_vec::ExpandableVec;
use std::char;
use std::io::{self, Bytes, Stdin};
use std::io::prelude::*;

#[derive(Debug)]
pub struct Interpreter {
  pointer: i64,
  tape: ExpandableVec<i64>
}

impl Interpreter {
  pub fn new() -> Interpreter {
    Interpreter {
      pointer: 0,
      tape: ExpandableVec::new()
    }
  }

  fn curr_val(&mut self) -> &mut i64 {
    self.tape.at(self.pointer)
  }

  fn curr_val_im(&self) -> i64 {
    self.tape.at_im(self.pointer)
  }

  fn print_val(&self, writer: &mut impl Write) {
    let val = self.curr_val_im();
    if val >= 0 && val < std::u32::MAX as i64 {
      match char::from_u32(val as u32) {
        Some(c) => write!(writer, "{}", c).unwrap(),
        None => (),
      };
    };
  } 

  fn read_val(&mut self, reader: &mut impl Read) {
    let mut buf = [0; 1];
    match reader.read(&mut buf) {
      Ok(ok) => match ok {
        1 => *self.curr_val() = buf[0] as i64,
        _ => *self.curr_val() = -1,
      },
      Err(e) => panic!(e),
    }
  }

  pub fn execute(&mut self, instructions: Vec<Token>, reader: &mut impl Read, writer: &mut impl Write) {
    let mut curr_inst = 0;

    loop {
      if curr_inst >= instructions.len() {
        break;
      }

      let token = instructions[curr_inst]; 
      
      match token {
        Token::IncPointer(inc) => self.pointer += inc,
        Token::IncCell(inc) => *self.curr_val() += inc,
        Token::StartLoop => {
          if *self.curr_val() == 0 {
            let mut counter = 0;
            loop {
              curr_inst += 1;
              match instructions[curr_inst] {
                Token::EndLoop => {
                  if counter == 0 {
                    break
                  } else {
                    counter -= 1;
                  }
                },
                Token::StartLoop => counter += 1,
                _ => (),
              }
            }
          }
        },
        Token::EndLoop => {
          if *self.curr_val() != 0 {
            let mut counter = 0;
            loop {
              curr_inst -= 1;
              match instructions[curr_inst] {
                Token::StartLoop => {
                  if counter == 0 {
                    break
                  } else {
                    counter -= 1;
                  }
                },
                Token::EndLoop => counter += 1,
                _ => (),
              }
            }
          }
        },
        Token::Print => self.print_val(writer),
        Token::ReadInput => self.read_val(reader),
      }

      curr_inst += 1;
    }
  }
}