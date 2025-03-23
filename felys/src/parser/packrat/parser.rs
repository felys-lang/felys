use crate::parser::packrat::intern::Intern;
use crate::parser::packrat::memo::Memo;
use crate::parser::packrat::stream::Stream;

pub struct Parser {
    memo: Memo,
    stream: Stream,
    intern: Intern,
}
