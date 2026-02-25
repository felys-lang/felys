use crate::philia093::Interner;
use crate::utils::ast::{
    AssOp, BinOp, Block, Bool, Chunk, Expr, Impl, Item, Lit, Pat, Stmt, UnaOp,
};
use std::fmt::{Display, Formatter, Write};

impl Item {
    pub fn recover<W: Write>(
        &self,
        f: &mut W,
        start: &'static str,
        indent: usize,
        interner: &Interner,
    ) -> std::fmt::Result {
        match self {
            Item::Group(id, fields) => {
                write!(f, "group {}(", interner.resolve(id).unwrap())?;
                let mut iter = fields.iter();
                if let Some(field) = iter.next() {
                    write!(f, "{}", interner.resolve(field).unwrap())?;
                }
                for field in iter {
                    write!(f, ", {}", interner.resolve(field).unwrap())?;
                }
                write!(f, ");")
            }
            Item::Impl(_, _) => Ok(()),
            Item::Fn(id, args, block) => {
                write!(f, "fn {}(", interner.resolve(id).unwrap())?;
                if let Some(args) = args {
                    let mut iter = args.iter();
                    if let Some(first) = iter.next() {
                        write!(f, "{}", interner.resolve(first).unwrap())?;
                    }
                    for arg in iter {
                        write!(f, ", {}", interner.resolve(arg).unwrap())?;
                    }
                }
                write!(f, ") ")?;
                block.recover(f, start, indent, None, interner)
            }
            Item::Main(args, block) => {
                write!(f, "fn main({}) ", interner.resolve(args).unwrap())?;
                block.recover(f, start, indent, None, interner)
            }
        }
    }
}

impl Impl {
    pub fn recover<W: Write>(
        &self,
        f: &mut W,
        start: &'static str,
        indent: usize,
        interner: &Interner,
    ) -> std::fmt::Result {
        match self {
            Impl::Associated(id, args, block) => {
                write!(f, "fn {}(", interner.resolve(id).unwrap())?;
                if let Some(args) = args {
                    let mut iter = args.iter();
                    if let Some(first) = iter.next() {
                        write!(f, "{}", interner.resolve(first).unwrap())?;
                    }
                    for arg in iter {
                        write!(f, ", {}", interner.resolve(arg).unwrap())?;
                    }
                }
                write!(f, ") ")?;
                block.recover(f, start, indent, None, interner)
            }
            Impl::Method(id, args, block) => {
                write!(f, "fn {}(self", interner.resolve(id).unwrap())?;
                for arg in args {
                    write!(f, ", {}", interner.resolve(arg).unwrap())?;
                }
                write!(f, ") ")?;
                block.recover(f, start, indent, None, interner)
            }
        }
    }
}

impl Stmt {
    pub fn recover<W: Write>(
        &self,
        f: &mut W,
        start: &'static str,
        indent: usize,
        interner: &Interner,
    ) -> std::fmt::Result {
        match self {
            Stmt::Empty => write!(f, ";"),
            Stmt::Expr(expr) => expr.recover(f, start, indent, interner),
            Stmt::Semi(expr) => {
                expr.recover(f, start, indent, interner)?;
                write!(f, "; ")
            }
            Stmt::Assign(pat, op, expr) => {
                pat.recover(f, interner)?;
                write!(f, " {op} ")?;
                expr.recover(f, start, indent, interner)?;
                write!(f, "; ")
            }
        }
    }
}

impl Block {
    pub fn recover<W: Write>(
        &self,
        f: &mut W,
        start: &'static str,
        indent: usize,
        pointer: Option<(usize, &'static str)>,
        interner: &Interner,
    ) -> std::fmt::Result {
        writeln!(f, "{{")?;
        for (i, stmt) in self.0.iter().enumerate() {
            if let Some((ptr, replace)) = pointer
                && ptr == i
            {
                write!(f, "{}{}", replace, "    ".repeat(indent + 1))?;
                stmt.recover(f, replace, indent + 1, interner)?;
            } else {
                write!(f, "{}{}", start, "    ".repeat(indent + 1))?;
                stmt.recover(f, start, indent + 1, interner)?;
            }
            writeln!(f)?;
        }
        write!(f, "{}{}}}", start, "    ".repeat(indent))
    }
}

impl Pat {
    pub fn recover<W: Write>(&self, f: &mut W, interner: &Interner) -> std::fmt::Result {
        match self {
            Pat::Any => write!(f, "_"),
            Pat::Tuple(pats) => {
                write!(f, "(")?;
                let mut iter = pats.iter();
                if let Some(first) = iter.next() {
                    first.recover(f, interner)?;
                }
                for arg in iter {
                    write!(f, ", ")?;
                    arg.recover(f, interner)?;
                }
                write!(f, ")")
            }
            Pat::Ident(id) => write!(f, "{}", interner.resolve(id).unwrap()),
        }
    }
}

impl Expr {
    pub fn recover<W: Write>(
        &self,
        f: &mut W,
        start: &'static str,
        indent: usize,
        interner: &Interner,
    ) -> std::fmt::Result {
        match self {
            Expr::Block(block) => block.recover(f, start, indent, None, interner),
            Expr::Break(expr) => {
                write!(f, "break")?;
                if let Some(expr) = expr {
                    write!(f, " ")?;
                    expr.recover(f, start, indent, interner)?;
                }
                Ok(())
            }
            Expr::Continue => write!(f, "continue"),
            Expr::For(pat, expr, block) => {
                write!(f, "for ")?;
                pat.recover(f, interner)?;
                write!(f, " in ")?;
                expr.recover(f, start, indent, interner)?;
                write!(f, " ")?;
                block.recover(f, start, indent, None, interner)
            }
            Expr::If(expr, then, otherwise) => {
                write!(f, "if ")?;
                expr.recover(f, start, indent, interner)?;
                write!(f, " ")?;
                then.recover(f, start, indent, None, interner)?;
                if let Some(otherwise) = otherwise {
                    write!(f, " else ")?;
                    otherwise.recover(f, start, indent, interner)?;
                }
                Ok(())
            }
            Expr::Loop(block) => {
                write!(f, "loop ")?;
                block.recover(f, start, indent, None, interner)
            }
            Expr::Return(expr) => {
                write!(f, "return ")?;
                expr.recover(f, start, indent, interner)
            }
            Expr::While(expr, block) => {
                write!(f, "while ")?;
                expr.recover(f, start, indent, interner)?;
                write!(f, " ")?;
                block.recover(f, start, indent, None, interner)
            }
            Expr::Binary(lhs, op, rhs) => {
                lhs.recover(f, start, indent, interner)?;
                write!(f, " {op} ")?;
                rhs.recover(f, start, indent, interner)
            }
            Expr::Call(expr, args) => {
                expr.recover(f, start, indent, interner)?;
                write!(f, "(")?;
                if let Some(args) = args {
                    let mut iter = args.iter();
                    if let Some(first) = iter.next() {
                        first.recover(f, start, indent, interner)?;
                    }
                    for arg in iter {
                        write!(f, ", ")?;
                        arg.recover(f, start, indent, interner)?;
                    }
                }
                write!(f, ")")
            }
            Expr::Field(expr, id) => {
                expr.recover(f, start, indent, interner)?;
                write!(f, ".{}", interner.resolve(id).unwrap())
            }
            Expr::Method(expr, id, args) => {
                expr.recover(f, start, indent, interner)?;
                write!(f, ".{}", interner.resolve(id).unwrap())?;
                write!(f, "(")?;
                if let Some(args) = args {
                    let mut iter = args.iter();
                    if let Some(first) = iter.next() {
                        first.recover(f, start, indent, interner)?;
                    }
                    for arg in iter {
                        write!(f, ", ")?;
                        arg.recover(f, start, indent, interner)?;
                    }
                }
                write!(f, ")")
            }
            Expr::Index(expr, index) => {
                expr.recover(f, start, indent, interner)?;
                write!(f, "[")?;
                index.recover(f, start, indent, interner)?;
                write!(f, "]")
            }
            Expr::Tuple(args) => {
                write!(f, "(")?;
                let mut iter = args.iter();
                if let Some(first) = iter.next() {
                    first.recover(f, start, indent, interner)?;
                }
                for arg in iter {
                    write!(f, ", ")?;
                    arg.recover(f, start, indent, interner)?;
                }
                write!(f, ")")
            }
            Expr::List(args) => {
                write!(f, "[")?;
                if let Some(args) = args {
                    let mut iter = args.iter();
                    if let Some(first) = iter.next() {
                        first.recover(f, start, indent, interner)?;
                    }
                    for arg in iter {
                        write!(f, ", ")?;
                        arg.recover(f, start, indent, interner)?;
                    }
                }
                write!(f, "]")
            }
            Expr::Lit(lit) => lit.recover(f, interner),
            Expr::Paren(expr) => {
                write!(f, "(")?;
                expr.recover(f, start, indent, interner)?;
                write!(f, ")")
            }
            Expr::Unary(op, expr) => {
                write!(f, "{op}")?;
                expr.recover(f, start, indent, interner)
            }
            Expr::Path(_, path) => {
                let mut iter = path.iter();
                if let Some(first) = iter.next() {
                    write!(f, "{}", interner.resolve(first).unwrap())?;
                }
                for arg in iter {
                    write!(f, "::{}", interner.resolve(arg).unwrap())?;
                }
                Ok(())
            }
        }
    }
}

impl Lit {
    pub fn recover<W: Write>(&self, f: &mut W, interner: &Interner) -> std::fmt::Result {
        match self {
            Lit::Int(x) | Lit::Float(x) => write!(f, "{}", interner.resolve(x).unwrap()),
            Lit::Bool(x) => match x {
                Bool::True => write!(f, "true"),
                Bool::False => write!(f, "false"),
            },
            Lit::Str(chunks) => {
                write!(f, "\"")?;
                for chunks in chunks {
                    chunks.recover(f, interner)?;
                }
                write!(f, "\"")?;
                Ok(())
            }
        }
    }
}

impl Chunk {
    pub fn recover<W: Write>(&self, f: &mut W, interner: &Interner) -> std::fmt::Result {
        match self {
            Chunk::Slice(x) => write!(f, "{}", interner.resolve(x).unwrap()),
            Chunk::Unicode(x) => write!(f, "\\u{{{}}}", interner.resolve(x).unwrap()),
            Chunk::Escape(x) => write!(f, "\\{}", interner.resolve(x).unwrap()),
        }
    }
}

impl Display for AssOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AssOp::AddEq => write!(f, "+="),
            AssOp::SubEq => write!(f, "-="),
            AssOp::MulEq => write!(f, "*="),
            AssOp::DivEq => write!(f, "/="),
            AssOp::ModEq => write!(f, "%="),
            AssOp::Eq => write!(f, "="),
        }
    }
}

impl Display for BinOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BinOp::Or => write!(f, "or"),
            BinOp::And => write!(f, "and"),
            BinOp::Gt => write!(f, ">"),
            BinOp::Ge => write!(f, ">="),
            BinOp::Lt => write!(f, "<"),
            BinOp::Le => write!(f, "<="),
            BinOp::Eq => write!(f, "=="),
            BinOp::Ne => write!(f, "!="),
            BinOp::Add => write!(f, "+"),
            BinOp::Sub => write!(f, "-"),
            BinOp::Mul => write!(f, "*"),
            BinOp::Div => write!(f, "/"),
            BinOp::Mod => write!(f, "%"),
            BinOp::At => write!(f, "@"),
        }
    }
}

impl Display for UnaOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaOp::Not => write!(f, "not "),
            UnaOp::Pos => write!(f, "+"),
            UnaOp::Neg => write!(f, "-"),
        }
    }
}
