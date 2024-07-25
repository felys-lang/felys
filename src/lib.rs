use std::collections::HashMap;
use std::str::FromStr;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use crate::ast::ASTFactory;
use crate::env::{Environ, Scope};
use crate::lexer::tokenize;
use crate::error::{Error, RuntimeError};
pub use crate::env::Object;


mod lexer;
mod expr;
mod flow;
mod ast;
mod env;
mod error;


pub struct Summary {
    pub duration: Duration,
    pub stdout: String,
    pub code: Object
}


pub struct Worker {
    global: Scope,
    builtin: Scope,
    timeout: Duration,
    lang: Language
}

impl Worker {
    pub fn new(mixin: HashMap<String, Object>, timeout: f64, lang: Language) -> Self {
        let mut base = match lang {
            Language::CHN => HashMap::from([
                ("——爱莉希雅——".into(), Object::String {value: "粉色妖精小姐♪".into()}),
                ("——作者——".into(), Object::String {value: "银河猫猫侠".into()})
            ]),
            Language::ENG => HashMap::from([
                ("__elysia__".into(), Object::String {value: "爱莉希雅".into()}),
                ("__author__".into(), Object::String {value: "FelysNeko".into()})
            ])
        };
        base.extend(mixin);
        Self {
            global: Scope::new(HashMap::new()),
            builtin: Scope::new(base),
            timeout: Duration::from_secs_f64(timeout),
            lang
        }
    }
    
    pub fn exec(&mut self, code: String) -> Result<Summary, Error> {
        let start = Instant::now();
        let (tx, timer) = mpsc::channel();
        let mut environ = Environ {
            builtin: &self.builtin,
            timer: &timer,
            body: vec![Scope::new(self.global.body.clone())]
        };

        let tokens = tokenize(code, &self.lang)?;
        let mut factory = ASTFactory::new(tokens);
        
        let limit = self.timeout;
        thread::spawn(move || {
            if limit.is_zero() {
                tx.send(false)
            } else {
                thread::sleep(limit);
                tx.send(true)
            }
        });

        let mut stdout = Vec::new();
        let mut code = Object::None;
        while let Some(stmt) = factory.produce() {
            if let Some(value) = stmt?.run(&mut environ, &mut stdout)? {
                code = value;
                break
            }
        }
        
        self.global = environ.body.pop()
            .unwrap_or(Scope::new(HashMap::new()));

        Ok(Summary {
            duration: start.elapsed(),
            stdout: stdout.join("\n"),
            code
        })
    }
}


pub struct Context<'a> {
    pub args: Vec<Object>,
    out: &'a mut Vec<String>
}

impl Context<'_> {
    pub fn write(&mut self, s: String) {
        self.out.push(s)
    }
}


pub struct Output {
    result: Result<Object, RuntimeError>
}

impl Output {
    pub fn error(msg: String) -> Self {
        Self { result: Err(RuntimeError::FromRust { s: msg }) }
    }
}

impl From<Object> for Output {
    fn from(value: Object) -> Self {
        Self { result: Ok(value) }
    }
}

#[derive(Clone)]
pub enum Language {
    CHN,
    ENG
}

impl FromStr for Language {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lang = match s.to_ascii_lowercase().as_str() {
            "en" | "eng" => Self::ENG,
            "cn" | "chn" => Self::CHN,
            _ => return Err("")
        };
        Ok(lang)
    }
}
