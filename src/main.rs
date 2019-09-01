
use std::io::Read;

enum Token {
    IncCell(i64),
    IncPointer(i64),

}

fn parse_tokens(reader: &mut Read) -> Vec<Token> {
    let tokens = vec![];
    let mut contents = String::new();
    
    reader.read_to_string(&mut contents)
        .unwrap();

    contents.chars().filter(|c| true);
    
    tokens

}

fn main() {
    println!("Hello, world!");
}


