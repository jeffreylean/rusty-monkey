use crate::token::Token;

trait Node {
    fn token_literal(&self) -> &str;
}

trait Statement: Node {
    fn statement_node(&self);
}

trait Expression: Node {
    fn expression_node(&self);
}

struct Program<T: Statement> {
    pub statements: Vec<T>,
}

impl<T: Statement> Node for Program<T> {
    fn token_literal(&self) -> &str {
        if let Some(s) = self.statements.first() {
            return s.token_literal();
        } else {
            return "";
        }
    }
}

struct LetStatement<'a> {
    token: Token,
    name: Identifier<'a>,
    value: dyn Expression,
}

impl Statement for LetStatement<'_> {
    fn statement_node(&self) {}
}

impl Node for LetStatement<'_> {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

struct Identifier<'a> {
    token: Token,
    value: &'a str,
}

impl Expression for Identifier<'_> {
    fn expression_node(&self) {}
}

impl Node for Identifier<'_> {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}
