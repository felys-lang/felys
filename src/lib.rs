//! Interface to customize Felys implementation
//!
//! # Examples
//! Make sure `felys` is added to your `cargo.toml`, and then try it out:
//! ```rust
#![doc = include_str!("../examples/basic.rs")]
//! ```

use std::collections::HashMap;
use std::str::FromStr;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use crate::ast::ASTFactory;
use crate::env::{Environ, Scope};
pub use crate::env::Object;
use crate::error::{Error, RuntimeError};
use crate::lexer::tokenize;

mod lexer;
mod expr;
mod flow;
mod ast;
mod env;
mod error;


/// Relevant outputs after one execution
pub struct Summary {
    /// Time spent in different steps: (initialization, lexing, runtime)
    pub time: (Duration, Duration, Duration),
    /// A string concatenated from output buffer vector
    pub stdout: String,
    /// Return value of the whole execution
    pub exit: Object,
}


/// Core interpreter entry point
pub struct Worker {
    global: Scope,
    builtin: Scope,
    timeout: Duration,
    maxcall: usize,
    lang: Language,
}

impl Worker {
    /// Configure and initialize a new worker
    pub fn new(mixin: HashMap<String, Object>, timeout: f64, maxcall: usize, lang: Language) -> Self {
        let mut base = match lang {
            Language::ZH => HashMap::from([
                ("——爱莉希雅——".into(), Object::String("粉色妖精小姐♪".into())),
                ("——作者——".into(), Object::String("银河猫猫侠".into()))
            ]),
            Language::EN => HashMap::from([
                ("__elysia__".into(), Object::String("爱莉希雅".into())),
                ("__author__".into(), Object::String("FelysNeko".into()))
            ])
        };
        base.extend(mixin);
        Self {
            global: Scope::new(HashMap::new()),
            builtin: Scope::new(base),
            timeout: Duration::from_secs_f64(timeout),
            maxcall,
            lang,
        }
    }

    /// Execute code and flush the global variable scope
    pub fn exec(&mut self, code: String) -> Result<Summary, Error> {
        let start = Instant::now();
        let (tx, rx) = mpsc::channel();
        let mut environ = Environ {
            builtin: &self.builtin,
            timer: &rx,
            called: (0, self.maxcall),
            body: vec![Scope::new(self.global.body.clone())],
        };

        let limit = self.timeout;
        if !limit.is_zero() {
            thread::spawn(move || {
                thread::sleep(limit);
                tx.send(true)
            });
        }
        let t0 = start.elapsed();

        let tokens = tokenize(code, &self.lang)?;
        let mut factory = ASTFactory::new(tokens);
        let t1 = start.elapsed();

        let mut stdout = Vec::new();
        let mut exit = Object::None;
        while let Some(stmt) = factory.produce() {
            if let Some(value) = stmt?.run(&mut environ, &mut stdout)? {
                exit = value;
                break;
            }
        }

        self.global = environ.body.pop()
            .unwrap_or(Scope::new(HashMap::new()));

        Ok(Summary {
            time: (t0, t1 - t0, start.elapsed() - t1),
            stdout: stdout.join("\n"),
            exit,
        })
    }
}


/// Abstraction of interaction with runtime environment
pub struct Context<'a> {
    /// Vector of arguments that the program passed in
    pub args: Vec<Object>,
    out: &'a mut Vec<String>,
}

impl Context<'_> {
    /// Write a string to output buffer
    pub fn write(&mut self, s: String) {
        self.out.push(s)
    }
}


/// Abstraction of return value from a rust function
pub struct Output {
    result: Result<Object, RuntimeError>,
}

impl Output {
    /// Throw an error to the runtime backend
    pub fn error(msg: String) -> Self {
        Self { result: Err(RuntimeError::FromRust { s: msg }) }
    }
}

impl From<Object> for Output {
    fn from(value: Object) -> Self {
        Self { result: Ok(value) }
    }
}


/// Available coding languages
#[derive(Clone)]
pub enum Language {
    /// Chinese
    ZH,
    /// English
    EN,
}

impl FromStr for Language {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lang = match s.to_ascii_lowercase().as_str() {
            "en" => Self::EN,
            "zh" => Self::ZH,
            _ => return Err("")
        };
        Ok(lang)
    }
}
