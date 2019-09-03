mod config;
use std::io::Write;
mod parser;
mod interpreter;
use interpreter::executor::{Interpreter};
use config::{parse};
use std::io::Read;
use std::fs::File;
mod injection;

// TODO fix clone hack
pub fn testable_main(args: impl Iterator<Item = String>, stdin: &mut impl Read, stdin_clone: &mut impl Read, stdout: &mut impl Write) {
  let (commands, _flags) = parse(args);
  let (mut source, mut stdin) : (Box<dyn Read>, Box<dyn Read>) = if commands.len() >= 1 {
    (Box::new(File::open(commands[0].clone()).unwrap()), Box::new(stdin))
  } else {
    (Box::new(stdin), Box::new(stdin_clone))
  };

  let tokens = parser::parse_tokens(&mut source);
  parser::validate(&tokens);
  let mut interpreter = Interpreter::new();
  interpreter.execute(tokens, &mut stdin, stdout);
}