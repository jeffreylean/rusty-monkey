use std::cell::RefCell;
use std::rc::Rc;

use crate::lexer::Lexer;
use crate::token::Token;

struct Parser<'a> {
    lex: Rc<RefCell<Lexer<'a>>>,

    cur_token: Rc<RefCell<Token>>,
    peek_token: Rc<RefCell<Token>>,
}
fn new<'a>(lex: Rc<RefCell<Lexer<'a>>>) -> Parser<'a> {
    let curr = lex.borrow_mut().next_token();

    let peek = lex.borrow_mut().next_token();

    let parser = Parser {
        lex: lex.clone(),
        cur_token: Rc::new(RefCell::new(curr)),
        peek_token: Rc::new(RefCell::new(peek)),
    };

    parser
}

impl<'a> Parser<'a> {
    fn next_token(&'a mut self) {
        self.cur_token = self.peek_token.clone();
        let next = self.lex.borrow_mut().next_token();
        self.peek_token = Rc::new(RefCell::new(next));
    }

    fn parse_program(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer;

    #[test]
    fn test_let_statement() {
        let input = r#"
            let x = 5;
            let y = 10;
            let foobar = 838383;
        "#;

        let lex = lexer::new(input);
        let parser = new(Rc::new(RefCell::new(lex)));

        let program = parser.parse_program();
    }
}
