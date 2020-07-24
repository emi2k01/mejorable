use std::vec::IntoIter;
use crate::line::token::LineToken;

pub(crate) mod tree;
use tree::Node;

pub(crate) struct Parser {
    source: String,
    tokens: IntoIter<LineToken>,
    current_token: Option<LineToken>,
    peek_token: Option<LineToken>,
    tree: Vec<Node>,
    errors: Vec<&'static str>,
}

impl Parser {
    pub fn new(source: String, tokens: Vec<LineToken>) -> Self {
        Self {
            source,
            tokens: tokens.into_iter(),
            current_token: None,
            peek_token: None,
            tree: vec![],
            errors: vec![],
        }
    }

    pub fn parse(&mut self) -> Self {
        while let Some(tok0) = self.current_token {
            
        }
    }

    fn bump(&mut self) {
        self.current_token = self.peek_token.take();
        self.peek_token = self.tokens.next();
    }
}
