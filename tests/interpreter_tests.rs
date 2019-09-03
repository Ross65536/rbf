extern crate rbf;
use rbf::testable_main;

use std::fs::File;
use std::io::{Write, Read};
use std::env;
use std::io::stdin;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::path::PathBuf;

fn read_file(path: String) -> String {
  let mut file = File::open(path).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  
  contents
}

fn create_tmp_file(filename: &str) -> (File, String) {
  let path = format!("./tests/tmp/rbf_tests_{}", filename);
  let path_str = path.to_string();
  let tmp_file = OpenOptions::new()
    .read(true)
    .write(true)
    .truncate(true)
    .create(true)
    .open(path)
    .unwrap();

  (tmp_file, path_str)
}

fn write_tmp_file(filename: &str, contents: String) -> (File, String) {
  let (mut tmp_file, path) = create_tmp_file(filename);
  write!(tmp_file, "{}", contents).unwrap();
  tmp_file.seek(SeekFrom::Start(0)).unwrap();

  (tmp_file, path)
}

fn create_source(source_path: &str) -> String {
  let contents = read_file(source_path.to_string());
  write_tmp_file("sources.b", contents).1
}


fn run_main(test_source: &str) -> String {
  run_main_with_input(test_source, "".to_string())
}

fn run_main_with_input(test_source: &str, stdin_contents: String) -> String {
  let file = create_source(test_source);
  let args = vec!["rbf".to_string(), file].into_iter();
  let (mut stdout, stdout_path) = create_tmp_file("stdout.txt");
  let mut stdin = write_tmp_file("stdin.txt", stdin_contents).0;
  let mut copy = stdin.try_clone().unwrap();

  testable_main(args, &mut stdin, &mut copy, &mut stdout);

  read_file(stdout_path)
}

#[test]
fn test_simple() {
  let stdout_contents = run_main("tests/sources/hello.b");
  assert!(stdout_contents == "Hello World!\n");
}

#[test]
fn test_negative_cells() {
  let stdout_contents = run_main("tests/sources/hello_negative.b");
  assert!(stdout_contents == "Hello World!\n");
}

#[test]
fn test_stdin() {
  let stdin = "axmdeafe\nPo44?:@[\t 32".to_string();
  let stdout_contents = run_main_with_input("tests/sources/input.b", stdin.clone());
  println!("||{}||", stdout_contents);
  assert!(stdout_contents == stdin);
}