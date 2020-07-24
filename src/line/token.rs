use crate::token::Token;

pub(crate) type LineToken = Token<LineTokenKind>;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum LineTokenKind {
    SingleBlockFunction,
    MultiBlockFunction,
    InlineFunction,
    Indentation,
    Text,
    Separator,
}
