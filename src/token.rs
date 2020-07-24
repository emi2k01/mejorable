use crate::span::Span;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Token<K> {
    kind: K,
    span: Span,
}

impl<K> Token<K> {
    pub(crate) fn new(kind: K, span: Span) -> Self {
        Self {
            kind,
            span
        }
    }
}
