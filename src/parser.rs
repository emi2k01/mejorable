use crate::line::token::{LineToken, LineTokenKind};
use std::vec::IntoIter;

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

    pub fn parse(&mut self) {
        while let Some(tok0) = &self.current_token {
            match tok0.kind {
                LineTokenKind::SingleBlockFunction => self.parse_single_block_function(),
                _ => {}
            }
        }
    }

    fn parse_single_block_function(&mut self) {}

    fn parse_multi_block_function(&mut self) {
        todo!()
    }

    fn parse_text(&mut self) {
        todo!()
    }

    fn bump(&mut self) {
        self.current_token = self.peek_token.take();
        self.peek_token = self.tokens.next();
    }
}
