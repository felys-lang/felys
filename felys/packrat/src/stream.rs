pub struct Stream {
    pub(crate) body: String,
    pub strict: bool,
    pub cursor: usize,
}

impl Iterator for Stream {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        let skipped = self.body.chars().skip(self.cursor);
        for ch in skipped {
            self.cursor += 1;
            if self.strict || !ch.is_whitespace() {
                return Some(ch);
            }
        }
        None
    }
}

impl Stream {
    pub fn trim(&mut self) {
        if self.strict {
            return;
        }
        for ch in self.body.chars().skip(self.cursor) {
            if ch.is_whitespace() {
                self.cursor += 1;
            } else {
                break;
            }
        }
    }
}