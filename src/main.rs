mod config;
use std::io::Write;
use std::io::stdout;
use std::io::stdin;
mod parser;
mod interpreter;
use interpreter::executor::{Interpreter};
use config::{parse};
use std::env;
use std::io::Read;
use std::fs::File;
mod injection;

pub fn testable_main(stdin: &mut impl Read, stdout: &mut impl Write) {
  let (commands, _flags) = parse(env::args());
  let mut source : Box<dyn Read> = if commands.len() >= 1 {
    Box::new(File::open(commands[0].clone()).unwrap())
  } else {
    Box::new(stdin)
  };

  let tokens = parser::parse_tokens(&mut source);
  parser::validate(&tokens);
  let mut interpreter = Interpreter::new();
  interpreter.execute(tokens, &mut source, stdout);
}

fn main() {
  testable_main(&mut stdin(), &mut stdout())
}




