use std::io::{self, Write};

use crate::lexer;
use crate::token::TokenType;

const PROMPT: &str = ">>";

pub fn start() {
    let mut input = String::new();

    loop {
        print!("{PROMPT} ");
        io::stdout().flush().unwrap();
        input.clear();

        io::stdin().read_line(&mut input).unwrap();
        let mut lex = lexer::new(&input);

        let mut tok = lex.next_token();
        while tok.t_type != TokenType::Eof {
            println!("{tok:?}");

            tok = lex.next_token();
        }
    }
}
