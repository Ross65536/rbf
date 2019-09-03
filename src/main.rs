mod config;
use std::io::stdin;
mod parser;
mod interpreter;
use interpreter::executor::{Interpreter};
use config::{parse};
use std::env;
use std::io::Read;
use std::fs::File;
mod injection;

fn main() {
  let (commands, flags) = parse(env::args());
  let mut file : Box<Read> = if commands.len() >= 1 {
    Box::new(File::open(commands[0].clone()).unwrap())
  } else {
    Box::new(stdin())
  };

  let tokens = parser::parse_tokens(&mut file);

  parser::validate(&tokens);

  let mut interpreter = Interpreter::new();

  interpreter.execute(tokens);
}




