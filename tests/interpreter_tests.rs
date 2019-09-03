extern crate rbf;
use rbf::testable_main;

use std::fs::File;
use std::io::prelude::*;
use std::io::{Write, Read, Seek, SeekFrom};
use std::env;
use std::io::stdout;
use std::io::stdin;


// fn split_command_arguments(line: &str) -> impl Iterator<Item = String> {
//   line.split_whitespace()
//       .map(|s| s.to_string())    
// }

fn read_file(path: String) -> String {
  let mut file = File::open(path).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  
  contents
}

fn create_tmp_file(filename: &str) -> (File, String) {
  let mut dir = env::temp_dir();
  dir.push(format!("rbf_tests_{}", filename));
  let path = dir.to_str().unwrap().to_string();

  let tmp_file = File::create(dir).unwrap();
  (tmp_file, path)
}

fn create_source(source_path: &str) -> String {
  let contents = read_file(source_path.to_string());

  let (mut tmp_file, path) = create_tmp_file("source.b");
  write!(tmp_file, "{}", contents).unwrap();

  path
}

fn run_main(test_source: &str) -> String {
  let file = create_source(test_source);
  let args = vec!["rbf".to_string(), file].into_iter();
  let (mut stdout, stdout_path) = create_tmp_file("stdout.txt");

  testable_main(args, &mut stdin(), &mut stdin(), &mut stdout);

  read_file(stdout_path)
}

#[test]
fn test_hello_world() {
  let stdout_contents = run_main("tests/sources/hello.b");
  assert!(stdout_contents == "Hello World!\n");
}