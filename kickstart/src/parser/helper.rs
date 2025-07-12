use crate::parser::Packrat;

impl Packrat {
    pub fn action(&mut self) -> Option<usize> {
        self.__attempt(|x| {
            x.__expect("{")?;
            let mut string = String::new();
            let mut counter = 0;
            while let Some(ch) = x.stream.peek() {
                match ch {
                    '{' => counter += 1,
                    '}' => counter -= 1,
                    _ => (),
                }
                if counter == -1 {
                    break;
                }
                string.push(ch);
                x.stream.next();
            }
            x.__expect("}")?;
            if counter == -1 {
                Some(x.intern.id(string.as_str()))
            } else {
                None
            }
        })
    }

    pub fn eof(&mut self) -> Option<()> {
        self.stream.trim();
        if self.stream.next().is_none() {
            Some(())
        } else {
            None
        }
    }
}
