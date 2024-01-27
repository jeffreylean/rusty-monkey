use crate::token::{lookup_ident, Token, TokenType};

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    ch: char,
}

// Create new lexer instance.
pub fn new(input: &str) -> Lexer {
    let mut lex = Lexer {
        input,
        position: 0,
        read_position: 0,
        ch: char::default(),
    };
    lex.read_char();
    lex
}

impl<'a> Lexer<'a> {
    // Read the next character and move the pointer forward.
    fn read_char(&mut self) {
        if self.read_position < self.input.len() {
            self.ch = self
                .input
                .chars()
                .nth(self.read_position)
                .expect("Invalid input index");
        } else {
            // EOF
            self.ch = '\x00';
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    // Method to read identifier.
    fn read_identifier(&mut self) -> &str {
        let pos = self.position;
        // Repeatedly read character by character until we
        // reach a space.
        while self.ch.is_alphabetic() || self.ch == '_' {
            self.read_char();
        }

        &self.input[pos..self.position]
    }

    // Method to read number.
    fn read_number(&mut self) -> &str {
        let pos = self.position;
        // Repeatedly read digit by digit until we
        // reach a non-digit, ie, space.
        while self.ch.is_ascii_digit() {
            self.read_char();
        }

        &self.input[pos..self.position]
    }

    // Method to skip the space by moving the pointer forward.
    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char()
        }
    }

    // Method to tokenize the next token.
    pub fn next_token(&mut self) -> Token {
        let tok: Token;

        self.skip_whitespace();

        'outer: {
            match self.ch {
                '=' => {
                    if let Some(c) = self.input.chars().nth(self.read_position) {
                        if c == '=' {
                            tok = Token {
                                t_type: TokenType::Eq,
                                literal: "==".to_string(),
                            };
                            self.read_char();
                            break 'outer;
                        }
                    }
                    tok = Token {
                        t_type: TokenType::Assign,
                        literal: "=".to_string(),
                    }
                }
                ';' => {
                    tok = Token {
                        t_type: TokenType::Semicolon,
                        literal: ";".to_string(),
                    }
                }
                '(' => {
                    tok = Token {
                        t_type: TokenType::Lparen,
                        literal: "(".to_string(),
                    }
                }
                ')' => {
                    tok = Token {
                        t_type: TokenType::Rparen,
                        literal: ")".to_string(),
                    }
                }
                ',' => {
                    tok = Token {
                        t_type: TokenType::Comma,
                        literal: ",".to_string(),
                    }
                }
                '+' => {
                    tok = Token {
                        t_type: TokenType::Plus,
                        literal: "+".to_string(),
                    }
                }
                '-' => {
                    tok = Token {
                        t_type: TokenType::Minus,
                        literal: "-".to_string(),
                    }
                }
                '{' => {
                    tok = Token {
                        t_type: TokenType::Lbrace,
                        literal: "{".to_string(),
                    }
                }
                '}' => {
                    tok = Token {
                        t_type: TokenType::Rbrace,
                        literal: "}".to_string(),
                    }
                }
                '<' => {
                    tok = Token {
                        t_type: TokenType::Lt,
                        literal: "<".to_string(),
                    }
                }
                '>' => {
                    tok = Token {
                        t_type: TokenType::Gt,
                        literal: ">".to_string(),
                    }
                }
                '!' => {
                    if let Some(c) = self.input.chars().nth(self.read_position) {
                        if c == '=' {
                            tok = Token {
                                t_type: TokenType::NotEq,
                                literal: "!=".to_string(),
                            };
                            self.read_char();
                            break 'outer;
                        }
                    }
                    tok = Token {
                        t_type: TokenType::Bang,
                        literal: "!".to_string(),
                    }
                }
                '/' => {
                    tok = Token {
                        t_type: TokenType::Slash,
                        literal: "/".to_string(),
                    }
                }
                '*' => {
                    tok = Token {
                        t_type: TokenType::Asterisk,
                        literal: "*".to_string(),
                    }
                }
                '\x00' => {
                    tok = Token {
                        t_type: TokenType::Eof,
                        literal: "".to_string(),
                    }
                }
                _ => {
                    if self.ch.is_alphabetic() || self.ch == '_' {
                        let literal = self.read_identifier();
                        tok = Token {
                            literal: literal.to_string(),
                            t_type: lookup_ident(literal),
                        };
                        return tok;
                    } else if self.ch.is_ascii_digit() {
                        tok = Token {
                            t_type: TokenType::Int,
                            literal: self.read_number().to_string(),
                        };
                        return tok;
                    } else {
                        tok = Token {
                            t_type: TokenType::Illegal,
                            literal: "".to_string(),
                        }
                    }
                }
            };
        }
        self.read_char();
        tok
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::{Token, TokenType};

    #[test]
    fn test_next_token() {
        let input = r#"
            let five = 5;
            let ten = 10;

            let add = fn(x,y) {
                x+y;
            };

            let result = add(five,ten);
            !-/*5;
            5 < 10 > 5;

            if (5 < 10) {
                return true;
            } else {
                return false;
            }

            10 == 10;
            10 != 9;
            "#;

        let tests: Vec<Token> = vec![
            Token {
                t_type: TokenType::Let,
                literal: "let".to_string(),
            },
            Token {
                t_type: TokenType::Ident,
                literal: "five".to_string(),
            },
            Token {
                t_type: TokenType::Assign,
                literal: "=".to_string(),
            },
            Token {
                t_type: TokenType::Int,
                literal: "5".to_string(),
            },
            Token {
                t_type: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                t_type: TokenType::Let,
                literal: "let".to_string(),
            },
            Token {
                t_type: TokenType::Ident,
                literal: "ten".to_string(),
            },
            Token {
                t_type: TokenType::Assign,
                literal: "=".to_string(),
            },
            Token {
                t_type: TokenType::Int,
                literal: "10".to_string(),
            },
            Token {
                t_type: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                t_type: TokenType::Let,
                literal: "let".to_string(),
            },
            Token {
                t_type: TokenType::Ident,
                literal: "add".to_string(),
            },
            Token {
                t_type: TokenType::Assign,
                literal: "=".to_string(),
            },
            Token {
                t_type: TokenType::Function,
                literal: "fn".to_string(),
            },
            Token {
                t_type: TokenType::Lparen,
                literal: "(".to_string(),
            },
            Token {
                t_type: TokenType::Ident,
                literal: "x".to_string(),
            },
            Token {
                t_type: TokenType::Comma,
                literal: ",".to_string(),
            },
            Token {
                t_type: TokenType::Ident,
                literal: "y".to_string(),
            },
            Token {
                t_type: TokenType::Rparen,
                literal: ")".to_string(),
            },
            Token {
                t_type: TokenType::Lbrace,
                literal: "{".to_string(),
            },
            Token {
                t_type: TokenType::Ident,
                literal: "x".to_string(),
            },
            Token {
                t_type: TokenType::Plus,
                literal: "+".to_string(),
            },
            Token {
                t_type: TokenType::Ident,
                literal: "y".to_string(),
            },
            Token {
                t_type: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                t_type: TokenType::Rbrace,
                literal: "}".to_string(),
            },
            Token {
                t_type: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                t_type: TokenType::Let,
                literal: "let".to_string(),
            },
            Token {
                t_type: TokenType::Ident,
                literal: "result".to_string(),
            },
            Token {
                t_type: TokenType::Assign,
                literal: "=".to_string(),
            },
            Token {
                t_type: TokenType::Ident,
                literal: "add".to_string(),
            },
            Token {
                t_type: TokenType::Lparen,
                literal: "(".to_string(),
            },
            Token {
                t_type: TokenType::Ident,
                literal: "five".to_string(),
            },
            Token {
                t_type: TokenType::Comma,
                literal: ",".to_string(),
            },
            Token {
                t_type: TokenType::Ident,
                literal: "ten".to_string(),
            },
            Token {
                t_type: TokenType::Rparen,
                literal: ")".to_string(),
            },
            Token {
                t_type: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                t_type: TokenType::Bang,
                literal: "!".to_string(),
            },
            Token {
                t_type: TokenType::Minus,
                literal: "-".to_string(),
            },
            Token {
                t_type: TokenType::Slash,
                literal: "/".to_string(),
            },
            Token {
                t_type: TokenType::Asterisk,
                literal: "*".to_string(),
            },
            Token {
                t_type: TokenType::Int,
                literal: "5".to_string(),
            },
            Token {
                t_type: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                t_type: TokenType::Int,
                literal: "5".to_string(),
            },
            Token {
                t_type: TokenType::Lt,
                literal: "<".to_string(),
            },
            Token {
                t_type: TokenType::Int,
                literal: "10".to_string(),
            },
            Token {
                t_type: TokenType::Gt,
                literal: ">".to_string(),
            },
            Token {
                t_type: TokenType::Int,
                literal: "5".to_string(),
            },
            Token {
                t_type: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                t_type: TokenType::If,
                literal: "if".to_string(),
            },
            Token {
                t_type: TokenType::Lparen,
                literal: "(".to_string(),
            },
            Token {
                t_type: TokenType::Int,
                literal: "5".to_string(),
            },
            Token {
                t_type: TokenType::Lt,
                literal: "<".to_string(),
            },
            Token {
                t_type: TokenType::Int,
                literal: "10".to_string(),
            },
            Token {
                t_type: TokenType::Rparen,
                literal: ")".to_string(),
            },
            Token {
                t_type: TokenType::Lbrace,
                literal: "{".to_string(),
            },
            Token {
                t_type: TokenType::Return,
                literal: "return".to_string(),
            },
            Token {
                t_type: TokenType::True,
                literal: "true".to_string(),
            },
            Token {
                t_type: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                t_type: TokenType::Rbrace,
                literal: "}".to_string(),
            },
            Token {
                t_type: TokenType::Else,
                literal: "else".to_string(),
            },
            Token {
                t_type: TokenType::Lbrace,
                literal: "{".to_string(),
            },
            Token {
                t_type: TokenType::Return,
                literal: "return".to_string(),
            },
            Token {
                t_type: TokenType::False,
                literal: "false".to_string(),
            },
            Token {
                t_type: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                t_type: TokenType::Rbrace,
                literal: "}".to_string(),
            },
            Token {
                t_type: TokenType::Int,
                literal: "10".to_string(),
            },
            Token {
                t_type: TokenType::Eq,
                literal: "==".to_string(),
            },
            Token {
                t_type: TokenType::Int,
                literal: "10".to_string(),
            },
            Token {
                t_type: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                t_type: TokenType::Int,
                literal: "10".to_string(),
            },
            Token {
                t_type: TokenType::NotEq,
                literal: "!=".to_string(),
            },
            Token {
                t_type: TokenType::Int,
                literal: "9".to_string(),
            },
            Token {
                t_type: TokenType::Semicolon,
                literal: ";".to_string(),
            },
        ];

        let mut l = new(input);

        for tkn in tests {
            let tok = l.next_token();

            assert_eq!(tok.t_type, tkn.t_type);
            assert_eq!(tok.literal, tkn.literal);
        }
    }
}
