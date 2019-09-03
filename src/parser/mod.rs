use std::io::Read;

#[derive(Debug, Copy, Clone)]
pub enum Token {
  IncCell(i64),
  IncPointer(i64),
  ReadInput,
  Print,
  StartLoop,
  EndLoop
}

fn is_token_char(c: char) -> bool {
  match c {
    ',' | '.' | '[' | ']' | '+' | '-' | '>' | '<' => true,
    _ => false,
  }
}

fn char_to_token(c: char) -> Token {
  match c {
    ',' => Token::ReadInput,
    '.' => Token::Print,
    '[' => Token::StartLoop,
    ']' => Token::EndLoop,
    '+' => Token::IncCell(1),
    '-' => Token::IncCell(-1),
    '>' => Token::IncPointer(1),
    '<' => Token::IncPointer(-1),
    _ => panic!("invalid usage")
  }
}

pub fn parse_tokens(reader: &mut impl Read) -> Vec<Token> {
  let mut contents = String::new();
  
  reader.read_to_string(&mut contents)
      .unwrap();

  contents.chars()
    .filter(|c| is_token_char(*c))
    .map(|c| char_to_token(c))
    .collect()
}



pub fn validate(tokens: &Vec<Token>) {
  let mut loop_start_counter = 0;

  for token in tokens {
    loop_start_counter += match token {
      Token::StartLoop => 1,
      Token::EndLoop => -1,
      _ => 0
    };

    if loop_start_counter < 0 {
      panic!("Mismatched brackets");
    }
  }
}