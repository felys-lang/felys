pub struct Stream {
    data: String,
    strict: bool,
    cursor: usize,
}

impl Iterator for Stream {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let remaining = self.data.get(self.cursor..)?;
        let mut chars = remaining.chars();

        let ch = if self.strict {
            chars.next()?
        } else {
            chars.find(|c| !c.is_whitespace())?
        };

        self.cursor += remaining.len() - chars.as_str().len();
        Some(ch)
    }
}
