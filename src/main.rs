extern crate rbf;

use rbf::testable_main;
use std::io::stdout;
use std::io::stdin;
use std::env;

fn main() {
  testable_main(env::args(), &mut stdin(), &mut stdin(), &mut stdout())
}




