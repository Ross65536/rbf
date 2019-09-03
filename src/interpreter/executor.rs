use crate::parser::Token;
use crate::interpreter::expandable_vec::ExpandableVec;
use std::char;
use std::io;
use std::io::prelude::*;
use std::thread;

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

  pub fn execute(&mut self, instructions: Vec<Token>) {
    let mut curr_inst = 0;
    let mut stdin = io::stdin().bytes();


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
        Token::Print => {
          let val = *self.curr_val();
          if val >= 0 && val < std::u32::MAX as i64 {
            match char::from_u32(val as u32) {
              Some(c) => print!("{}", c),
              None => (),
            }
          }
        },
        Token::ReadInput => {
          match stdin.next() {
            Some(res) => match res {
              Ok(b) => *self.curr_val() = b as i64,
              Err(e) => panic!(e), 
            },
            None => *self.curr_val() = -1,  
          }
        }
      }

      curr_inst += 1;
    }
  }
}