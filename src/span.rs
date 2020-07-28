/// This is an example of text
/// ^                        ^
/// | There are 25 chars ----|
/// | so its length is 25
/// |
/// |--- The letter "T" has an offset of 0 (it's the first char)
#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct Span {
    offset: u32,
    len: u32,
}

impl Span {
    pub(crate) fn new(offset: u32, len: u32) -> Self {
        Self { offset, len }
    }
}
