use std::io::stdin;
mod parser;
mod interpreter;
use interpreter::executor::{Interpreter};

fn main() {
  let tokens = parser::parse_tokens(&mut stdin());

  parser::validate(&tokens);

  let mut interpreter = Interpreter::new();

  interpreter.execute(tokens);
}


