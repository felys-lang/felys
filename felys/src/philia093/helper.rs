use crate::philia093::PhiLia093;
use crate::utils::stages::Cyrene;
use std::fmt::{Display, Formatter};

impl PhiLia093 {
    pub fn parse(mut self) -> Result<Cyrene, String> {
        let root = self.root();
        if let Some((cursor, msg)) = self.__snapshot {
            let data = self.__stream.data;

            let row = data[..cursor].chars().filter(|c| *c == '\n').count();
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
            let error = Fault {
                snippet,
                row,
                col,
                msg,
            };
            Err(error.to_string())
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

struct Fault {
    snippet: String,
    row: usize,
    col: usize,
    msg: &'static str,
}

impl Display for Fault {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let row = self.row + 1;
        let col = self.col + 1;
        let padding = " ".repeat(row.to_string().len());
        writeln!(f, "PhiLia093: {} at {}:{}", self.msg, row, col)?;
        writeln!(f, " {} |", padding)?;
        writeln!(f, " {} | {}", row, self.snippet)?;
        writeln!(f, " {} | {}^", padding, " ".repeat(self.col))
    }
}
