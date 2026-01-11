use crate::cyrene::Cyrene;
use crate::philia093::PhiLia093;

impl PhiLia093 {
    pub fn parse(mut self) -> Result<Cyrene, String> {
        let root = self.root();
        if let Some((cursor, msg)) = self.__snapshot {
            let data = self.__stream.data;

            let row = data[..cursor].chars().filter(|c| *c == '\n').count() + 1;
            let mut col = 0;
            let mut start = cursor;
            for ch in data[..cursor].chars().rev() {
                if ch == '\n' {
                    break;
                }
                start -= ch.len_utf8();
                col += 1;
            }
            let mut end = cursor;
            for ch in data[cursor..].chars() {
                if ch == '\n' {
                    break;
                }
                end += ch.len_utf8();
            }

            let snippet = data[start..end].to_string();
            Err(format!("{snippet}:{col}:{row}:{msg}"))
        } else {
            let cyrene = Cyrene {
                root: root.unwrap(),
                intern: self.__intern,
            };
            Ok(cyrene)
        }
    }

    pub fn n2i(&mut self) -> Option<usize> {
        let id = self.NAME()?;
        let ident = self.__intern.get(&id).unwrap();
        if (self.__keywords)(ident) {
            None
        } else {
            Some(id)
        }
    }
}
