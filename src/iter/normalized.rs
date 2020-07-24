use std::str::Chars;

pub(crate) struct NormalizedChars<'a> {
    inner: Chars<'a>,
    current_char: Option<char>,
    peek_char: Option<char>,
}

impl<'a> NormalizedChars<'a> {
    pub(crate) fn new(inner: Chars<'a>) -> Self {
        let mut this = Self {
            inner,
            current_char: None,
            peek_char: None,
        };

        this.bump();
        this.bump();

        this
    }

    pub(crate) fn bump(&mut self) {
        self.current_char = self.peek_char;
        self.peek_char = self.inner.next();
    }
}

impl<'a> Iterator for NormalizedChars<'a> {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        match (self.current_char, self.peek_char) {
            (Some('\r'), Some('\n')) => {
                // skip '\r'
                self.bump();
                self.current_char
            },
            _ => {
                let ch = self.current_char;
                self.bump();
                ch
            }
        }
    }
}

trait IntoNormalizedChars<'a> {
    fn into_normalized_chars(self) -> NormalizedChars<'a>;
}

impl<'a> IntoNormalizedChars<'a> for Chars<'a> {
    fn into_normalized_chars(self) -> NormalizedChars<'a> {
        NormalizedChars::new(self)
    }
}
