pub struct Stream {
    pub body: String,
    pub strict: bool,
    pub cursor: usize,
}

impl Iterator for Stream {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        for ch in self.body[self.cursor..].chars() {
            self.cursor += ch.len_utf8();
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
        for ch in self.body[self.cursor..].chars() {
            if ch.is_whitespace() {
                self.cursor += ch.len_utf8();
            } else {
                break;
            }
        }
    }
}