use std::io::Stdin;
use std::io::Read;
use std::io::stdin;
use std::fs::File;

pub trait ReadWrapper {
  fn get_reader(&mut self) -> Box<Read>;
}

pub struct StdinReader {
  stdin: Stdin,
}

impl StdinReader {
  pub fn new() -> StdinReader {
    StdinReader {
      stdin: stdin()
    }
  }
}

impl ReadWrapper for StdinReader {
  fn get_reader(&mut self) -> Box<Read> {
    Box::new(stdin())
  }
}

pub struct FileReader {
  file: File,
}

impl FileReader {
  pub fn new(filepath: String) -> FileReader {
    FileReader {
      file: File::open(filepath).unwrap()
    }
  }
}

impl ReadWrapper for FileReader {
  fn get_reader(&mut self) -> Box<Read> {
    Box::new(self.file.try_clone().unwrap())
  }
} 