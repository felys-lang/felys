pub struct Stream {
    data: String,
    pub strict: bool,
    pub cursor: usize,
}

impl Iterator for Stream {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let remaining = &self.data[self.cursor..];
        let mut chars = remaining.char_indices();

        let (idx, ch) = if self.strict {
            chars.next()?
        } else {
            chars.find(|(_, c)| !c.is_whitespace())?
        };

        self.cursor += idx + ch.len_utf8();
        Some(ch)
    }
}

impl Stream {
    pub fn new(data: String) -> Self {
        Self {
            data,
            strict: false,
            cursor: 0,
        }
    }

    pub fn trim(&mut self) {
        if self.strict {
            return;
        }

        let remaining = &self.data[self.cursor..];
        let mut chars = remaining.char_indices();
        
        match chars.find(|(_, c)| !c.is_whitespace()) {
            Some((idx, _)) => self.cursor += idx,
            None => self.cursor += remaining.len(),
        }
    }
}
