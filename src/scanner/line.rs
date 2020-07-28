pub(crate) mod token;

use std::str::Chars;

use token::{LineToken, LineTokenKind};

use crate::iter::double::IntoDoubleIter;
use crate::span::Span;
use crate::token::Token;

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct Line {
    indentation: u32,
    tokens: Vec<LineToken>,
}

pub(crate) struct LineScanner<'a> {
    input: Chars<'a>,
    current_char: Option<char>,
    peek_char: Option<char>,
    tokens: Vec<LineToken>,
    indentation: u32,
    offset: u32,
}

impl<'a> LineScanner<'a> {
    pub(crate) fn new(input: Chars<'a>) -> Self {
        let mut this = Self {
            input,
            current_char: None,
            peek_char: None,
            tokens: vec![],
            indentation: 0,
            offset: 0,
        };

        this.bump();
        this.bump();

        this.offset = 0;

        this
    }

    pub(crate) fn scan(mut self) -> Line {
        if self.is_separator() {
            self.scan_separator();
            return Line {
                indentation: 0,
                tokens: self.tokens,
            };
        }

        while let Some(tok0) = self.current_char {
            match (tok0, self.peek_char) {
                ('[', Some('[')) => {
                    if self.closes_multi_block_function() {
                        self.scan_multi_block_function_start();
                    } else {
                        self.scan_text_and_inline_functions();
                    }
                }
                ('[', _) => {
                    if self.closes_single_block_function() {
                        self.scan_single_block_function_start();
                    } else {
                        self.scan_text_and_inline_functions();
                    }
                }
                (' ', _) | ('\t', _) => self.indentation = self.scan_indentation(),

                _ => self.scan_text_and_inline_functions(),
            }
        }

        Line {
            indentation: self.indentation,
            tokens: self.tokens,
        }
    }

    fn scan_separator(&mut self) {
        let start_offset = self.offset;
        while self.current_char == Some(' ') || self.current_char == Some('\t') {
            self.bump();
        }
        self.push_token(LineTokenKind::Separator, start_offset);
    }

    fn scan_multi_block_function_start(&mut self) {
        let start_offset = self.offset;

        // bump opening brackets
        self.bump();
        self.bump();

        // bump until closing brackets
        while !(self.current_char == Some(']') && self.peek_char == Some(']')) {
            self.bump();
        }

        // bump closing brackets
        self.bump();
        self.bump();

        self.push_token(LineTokenKind::MultiBlockFunction, start_offset);
    }

    fn scan_single_block_function_start(&mut self) {
        let start_offset = self.offset;

        // bump opening bracket
        self.bump();

        // bump until closing bracket
        while self.current_char != Some(']') {
            self.bump();
        }

        // bump closing bracket
        self.bump();

        self.push_token(LineTokenKind::SingleBlockFunction, start_offset);
    }

    fn scan_inline_function(&mut self) {
        let start_offset = self.offset;

        // bump opening parenthesis
        self.bump();

        let mut got_paren_and_bracket = false;

        // bump until inline function end
        loop {
            if self.current_char == Some(')') && self.peek_char == Some('[') {
                got_paren_and_bracket = true;
            }
            if got_paren_and_bracket
                && self.current_char != Some('\\')
                && self.peek_char == Some(']')
            {
                break;
            }
            self.bump();
        }

        // bump twice because closing bracket is in `peek_char`
        self.bump();
        self.bump();

        self.push_token(LineTokenKind::InlineFunction, start_offset);
    }

    fn scan_text_and_inline_functions(&mut self) {
        let mut start_offset = self.offset;
        while let Some(cc) = self.current_char {
            if cc == '(' {
                if self.in_inline_function() {
                    // Because an inline function is in mid text, we stop the text token here,
                    // scan the inline function and then we start scanning text again.
                    // This process can repeat multiple times if there are multiple inline
                    // functions
                    self.push_token(LineTokenKind::Text, start_offset);

                    self.scan_inline_function();

                    start_offset = self.offset;
                } else {
                    self.bump();
                }
            } else {
                self.bump();
            }
        }

        let token_len = self.offset - start_offset;
        if token_len > 0 {
            self.push_token(LineTokenKind::Text, start_offset);
        }
    }

    fn scan_indentation(&mut self) -> u32 {
        let start_offset = self.offset;
        while self.current_char == Some(' ') || self.current_char == Some('\t') {
            self.bump();
        }

        let token_len = self.offset - start_offset;
        token_len
    }

    fn in_inline_function(&self) -> bool {
        // There must be a ")[" and then a "]"

        let line = self.input.clone().into_double_iter();

        let mut got_paren_and_bracket = false;

        for (ch0, ch1) in line {
            match (ch0, ch1) {
                (')', Some('[')) => got_paren_and_bracket = true,
                (']', _) => {
                    if got_paren_and_bracket {
                        return true;
                    }
                }
                _ => {}
            }
        }
        false
    }

    fn closes_multi_block_function(&self) -> bool {
        let mut line = self.input.clone().rev();
        let mut previous_is_closing_bracket = false;
        for ch in &mut line {
            match ch {
                ']' => {
                    if previous_is_closing_bracket {
                        return true;
                    } else {
                        previous_is_closing_bracket = true;
                    }
                }
                _ => {
                    previous_is_closing_bracket = false;
                }
            }
        }
        false
    }

    fn closes_single_block_function(&self) -> bool {
        let line = self.input.clone().rev();
        for ch in line {
            match ch {
                ']' => return true,
                _ => {}
            }
        }
        false
    }

    fn is_separator(&self) -> bool {
        if !self
            .current_char
            .map_or(true, |ch| ch.is_ascii_whitespace())
        {
            return false;
        }

        if !self.peek_char.map_or(true, |ch| ch.is_ascii_whitespace()) {
            return false;
        }

        let line = self.input.clone();
        for ch in line {
            if !ch.is_ascii_whitespace() {
                return false;
            }
        }
        return true;
    }

    /// Ignores tokens with a length of 0
    fn push_token(&mut self, kind: LineTokenKind, start_offset: u32) {
        let token_len = self.offset - start_offset;
        self.tokens
            .push(Token::new(kind, Span::new(start_offset, token_len)));
    }

    fn bump(&mut self) {
        self.current_char = self.peek_char;
        self.peek_char = self.input.next();
        self.offset += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tokens(src: &str) -> Line {
        let scanner = LineScanner::new(src.chars());
        scanner.scan()
    }

    #[test]
    fn test_single_block_function_without_options() {
        let result = tokens("[code]");
        let expected = Line {
            indentation: 0,
            tokens: vec![LineToken::new(
                LineTokenKind::SingleBlockFunction,
                Span::new(0, 6),
            )],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_multi_block_function_without_options() {
        let result = tokens("[[code]]");
        let expected = Line {
            indentation: 0,
            tokens: vec![LineToken::new(
                LineTokenKind::MultiBlockFunction,
                Span::new(0, 8),
            )],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_text() {
        let result = tokens("  Hey how are you???\t\t");
        let expected = Line {
            indentation: 2,
            tokens: vec![LineToken::new(LineTokenKind::Text, Span::new(2, 20))],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_text_with_inline_function() {
        let result = tokens("Hello, (world)[hey]");
        let expected = Line {
            indentation: 0,
            tokens: vec![
                LineToken::new(LineTokenKind::Text, Span::new(0, 7)),
                LineToken::new(LineTokenKind::InlineFunction, Span::new(7, 12)),
            ],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_separator() {
        let result = tokens("\t \t ");
        let expected = Line {
            indentation: 0,
            tokens: vec![LineToken::new(LineTokenKind::Separator, Span::new(0, 4))],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_indentation() {
        let result = tokens("    \tHola");
        let expected = Line {
            indentation: 5,
            tokens: vec![LineToken::new(LineTokenKind::Text, Span::new(5, 4))],
        };

        assert_eq!(result, expected);
    }
}
