use crate::span::Span;

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct Token<K>
where
    K: Clone,
{
    pub(crate) kind: K,
    pub(crate) span: Span,
}

impl<K> Token<K>
where
    K: Clone,
{
    pub(crate) fn new(kind: K, span: Span) -> Self {
        Self { kind, span }
    }
}
