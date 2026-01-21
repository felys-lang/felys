use crate::elysia::Object;
use crate::philia093::Intern;
use crate::utils::ast::{
    AssOp, BinOp, Block, Bool, Chunk, Expr, Item, Lit, Pat, Path, Root, Stmt, UnaOp,
};
use crate::utils::group::Group;
use crate::utils::ir::Pointer;
use std::fmt::{Display, Formatter, Write};

impl Root {
    pub fn recover<W: Write>(
        &self,
        f: &mut W,
        start: &'static str,
        indent: usize,
        intern: &Intern,
    ) -> std::fmt::Result {
        for item in self.0.iter() {
            item.recover(f, start, indent, intern)?;
        }
        Ok(())
    }
}

impl Item {
    pub fn recover<W: Write>(
        &self,
        f: &mut W,
        start: &'static str,
        indent: usize,
        intern: &Intern,
    ) -> std::fmt::Result {
        match self {
            Item::Group(_, _) => Ok(()),
            Item::Impl(_, _) => Ok(()),
            Item::Fn(_, _, block) => block.recover(f, start, indent, None, intern),
            Item::Main(_, block) => block.recover(f, start, indent, None, intern),
        }
    }
}

impl Stmt {
    pub fn recover<W: Write>(
        &self,
        f: &mut W,
        start: &'static str,
        indent: usize,
        intern: &Intern,
    ) -> std::fmt::Result {
        match self {
            Stmt::Empty => write!(f, ";"),
            Stmt::Expr(expr) => expr.recover(f, start, indent, intern),
            Stmt::Semi(expr) => {
                expr.recover(f, start, indent, intern)?;
                write!(f, "; ")
            }
            Stmt::Assign(pat, op, expr) => {
                pat.recover(f, intern)?;
                write!(f, " {op} ")?;
                expr.recover(f, start, indent, intern)?;
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
        intern: &Intern,
    ) -> std::fmt::Result {
        writeln!(f, "{{")?;
        for (i, stmt) in self.0.iter().enumerate() {
            if let Some((ptr, replace)) = pointer
                && ptr == i
            {
                write!(f, "{}{}", replace, "    ".repeat(indent + 1))?;
                stmt.recover(f, replace, indent + 1, intern)?;
            } else {
                write!(f, "{}{}", start, "    ".repeat(indent + 1))?;
                stmt.recover(f, start, indent + 1, intern)?;
            }
            writeln!(f)?;
        }
        write!(f, "{}{}}}", start, "    ".repeat(indent))
    }
}

impl Pat {
    pub fn recover<W: Write>(&self, f: &mut W, intern: &Intern) -> std::fmt::Result {
        match self {
            Pat::Any => write!(f, "_"),
            Pat::Tuple(pats) => {
                write!(f, "(")?;
                let mut iter = pats.iter();
                if let Some(first) = iter.next() {
                    first.recover(f, intern)?;
                }
                for arg in iter {
                    write!(f, ", ")?;
                    arg.recover(f, intern)?;
                }
                write!(f, ")")
            }
            Pat::Ident(id) => write!(f, "{}", intern.get(id).unwrap()),
        }
    }
}

impl Expr {
    pub fn recover<W: Write>(
        &self,
        f: &mut W,
        start: &'static str,
        indent: usize,
        intern: &Intern,
    ) -> std::fmt::Result {
        match self {
            Expr::Block(block) => block.recover(f, start, indent, None, intern),
            Expr::Break(expr) => {
                write!(f, "break")?;
                if let Some(expr) = expr {
                    write!(f, " ")?;
                    expr.recover(f, start, indent, intern)?;
                }
                Ok(())
            }
            Expr::Continue => write!(f, "continue"),
            Expr::For(pat, expr, block) => {
                write!(f, "for ")?;
                pat.recover(f, intern)?;
                write!(f, " in ")?;
                expr.recover(f, start, indent, intern)?;
                write!(f, " ")?;
                block.recover(f, start, indent, None, intern)
            }
            Expr::If(expr, then, otherwise) => {
                write!(f, "if ")?;
                expr.recover(f, start, indent, intern)?;
                write!(f, " ")?;
                then.recover(f, start, indent, None, intern)?;
                if let Some(otherwise) = otherwise {
                    write!(f, " else ")?;
                    otherwise.recover(f, start, indent, intern)?;
                }
                Ok(())
            }
            Expr::Loop(block) => {
                write!(f, "loop ")?;
                block.recover(f, start, indent, None, intern)
            }
            Expr::Return(expr) => {
                write!(f, "return ")?;
                expr.recover(f, start, indent, intern)
            }
            Expr::While(expr, block) => {
                write!(f, "while ")?;
                expr.recover(f, start, indent, intern)?;
                write!(f, " ")?;
                block.recover(f, start, indent, None, intern)
            }
            Expr::Binary(lhs, op, rhs) => {
                lhs.recover(f, start, indent, intern)?;
                write!(f, " {op} ")?;
                rhs.recover(f, start, indent, intern)
            }
            Expr::Call(expr, args) => {
                expr.recover(f, start, indent, intern)?;
                write!(f, "(")?;
                if let Some(args) = args {
                    let mut iter = args.iter();
                    if let Some(first) = iter.next() {
                        first.recover(f, start, indent, intern)?;
                    }
                    for arg in iter {
                        write!(f, ", ")?;
                        arg.recover(f, start, indent, intern)?;
                    }
                }
                write!(f, ")")
            }
            Expr::Field(expr, id) => {
                expr.recover(f, start, indent, intern)?;
                write!(f, ".{}", intern.get(id).unwrap())
            }
            Expr::Method(expr, id, args) => {
                expr.recover(f, start, indent, intern)?;
                write!(f, ".{}", intern.get(id).unwrap())?;
                write!(f, "(")?;
                if let Some(args) = args {
                    let mut iter = args.iter();
                    if let Some(first) = iter.next() {
                        first.recover(f, start, indent, intern)?;
                    }
                    for arg in iter {
                        write!(f, ", ")?;
                        arg.recover(f, start, indent, intern)?;
                    }
                }
                write!(f, ")")
            }
            Expr::Index(expr, index) => {
                expr.recover(f, start, indent, intern)?;
                write!(f, "[")?;
                index.recover(f, start, indent, intern)?;
                write!(f, "]")
            }
            Expr::Tuple(args) => {
                write!(f, "(")?;
                let mut iter = args.iter();
                if let Some(first) = iter.next() {
                    first.recover(f, start, indent, intern)?;
                }
                for arg in iter {
                    write!(f, ", ")?;
                    arg.recover(f, start, indent, intern)?;
                }
                write!(f, ")")
            }
            Expr::List(args) => {
                write!(f, "[")?;
                if let Some(args) = args {
                    let mut iter = args.iter();
                    if let Some(first) = iter.next() {
                        first.recover(f, start, indent, intern)?;
                    }
                    for arg in iter {
                        write!(f, ", ")?;
                        arg.recover(f, start, indent, intern)?;
                    }
                }
                write!(f, "]")
            }
            Expr::Lit(lit) => lit.recover(f, intern),
            Expr::Paren(expr) => {
                write!(f, "(")?;
                expr.recover(f, start, indent, intern)?;
                write!(f, ")")
            }
            Expr::Unary(op, expr) => {
                write!(f, "{op}")?;
                expr.recover(f, start, indent, intern)
            }
            Expr::Path(path) => path.recover(f, intern),
        }
    }
}

impl Path {
    pub fn recover<W: Write>(&self, f: &mut W, intern: &Intern) -> std::fmt::Result {
        let mut iter = self.0.iter();
        if let Some(first) = iter.next() {
            write!(f, "{}", intern.get(first).unwrap())?;
        }
        for arg in iter {
            write!(f, "::{}", intern.get(arg).unwrap())?;
        }
        Ok(())
    }
}

impl Lit {
    pub fn recover<W: Write>(&self, f: &mut W, intern: &Intern) -> std::fmt::Result {
        match self {
            Lit::Int(x) | Lit::Float(x) => write!(f, "{}", intern.get(x).unwrap()),
            Lit::Bool(x) => match x {
                Bool::True => write!(f, "true"),
                Bool::False => write!(f, "false"),
            },
            Lit::Str(chunks) => {
                for chunks in chunks {
                    chunks.recover(f, intern)?;
                }
                Ok(())
            }
        }
    }
}

impl Chunk {
    pub fn recover<W: Write>(&self, f: &mut W, intern: &Intern) -> std::fmt::Result {
        match self {
            Chunk::Slice(x) => write!(f, "{}", intern.get(x).unwrap()),
            Chunk::Unicode(x) => write!(f, "\\u{{{}}}", intern.get(x).unwrap()),
            Chunk::Escape(x) => write!(f, "\\{}", intern.get(x).unwrap()),
        }
    }
}

impl Object {
    pub fn recover<W: Write>(
        &self,
        f: &mut W,
        indent: usize,
        groups: &[Group],
    ) -> std::fmt::Result {
        const INDENT: &str = "    ";
        match self {
            Object::Pointer(ptr, idx) => match ptr {
                Pointer::Function => write!(f, "<{idx}>"),
                Pointer::Group => write!(f, "<{idx}>"),
            },
            Object::List(objs) => {
                write!(f, "[")?;
                let mut iter = objs.iter();
                if let Some(first) = iter.next() {
                    first.recover(f, indent, groups)?
                }
                for obj in iter {
                    write!(f, ", ")?;
                    obj.recover(f, indent, groups)?
                }
                write!(f, "]")
            }
            Object::Tuple(objs) => {
                write!(f, "(")?;
                let mut iter = objs.iter();
                if let Some(first) = iter.next() {
                    first.recover(f, indent, groups)?
                }
                for obj in iter {
                    write!(f, ", ")?;
                    obj.recover(f, indent, groups)?
                }
                write!(f, ")")
            }
            Object::Group(id, objs) => {
                let group = groups.get(*id).unwrap();
                let indent = indent + 1;
                writeln!(f, "{id}: {{")?;
                let mut iter = objs.iter().enumerate();
                if let Some((i, first)) = iter.next() {
                    write!(f, "{}{}: ", INDENT.repeat(indent), group.fields[i])?;
                    first.recover(f, indent, groups)?;
                    writeln!(f, ",")?;
                }
                for (i, obj) in iter {
                    write!(f, "{}{}: ", INDENT.repeat(indent), group.fields[i])?;
                    obj.recover(f, indent, groups)?;
                    writeln!(f, ",")?;
                }
                write!(f, "}}")
            }
            Object::Str(x) => write!(f, "\"{}\"", x),
            Object::Int(x) => write!(f, "{}", x),
            Object::Float(x) => write!(f, "{}", x),
            Object::Bool(x) => write!(f, "{}", x),
            Object::Void => write!(f, "<void>"),
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
            BinOp::Dot => write!(f, "@"),
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
