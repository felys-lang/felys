//! > **Interface to customize Felys implementation**
//! 
//! ## Example
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
use crate::lexer::tokenize;
use crate::error::{Error, RuntimeError};
pub use crate::env::Object;


mod lexer;
mod expr;
mod flow;
mod ast;
mod env;
mod error;


/// Relevant outputs after one execution
pub struct Summary {
    /// Total runtime starting from initialization to retuning
    pub duration: Duration,
    /// A string concatenated from output buffer vector
    pub stdout: String,
    /// Return value of the whole execution
    pub code: Object
}


/// Core interpreter entry point
pub struct Worker {
    global: Scope,
    builtin: Scope,
    timeout: Duration,
    lang: Language
}

impl Worker {
    /// Configure and initialize a new worker
    pub fn new(mixin: HashMap<String, Object>, timeout: f64, lang: Language) -> Self {
        // this the built-in values that should never get removed
        let mut base = match lang {
            Language::CN => HashMap::from([
                ("——爱莉希雅——".into(), Object::String {value: "粉色妖精小姐♪".into()}),
                ("——作者——".into(), Object::String {value: "银河猫猫侠".into()})
            ]),
            Language::EN => HashMap::from([
                ("__elysia__".into(), Object::String {value: "爱莉希雅".into()}),
                ("__author__".into(), Object::String {value: "FelysNeko".into()})
            ])
        };
        // the user defined built-in values will overwrite default
        base.extend(mixin);
        Self {
            global: Scope::new(HashMap::new()),
            builtin: Scope::new(base),
            timeout: Duration::from_secs_f64(timeout),
            lang
        }
    }
    
    /// Execute code and flush the global variable scope
    pub fn exec(&mut self, code: String) -> Result<Summary, Error> {
        let start = Instant::now();
        let (tx, timer) = mpsc::channel();
        // build the environment for this execution
        // the first scope is the temporary global scope which is cloned from the worker
        let mut environ = Environ {
            builtin: &self.builtin,
            timer: &timer,
            body: vec![Scope::new(self.global.body.clone())]
        };
        
        // send a true to the channel when time is up
        // the other side will call .unwrap_or(false)
        let limit = self.timeout;
        thread::spawn(move || {
            if limit.is_zero() {
                tx.send(false)
            } else {
                thread::sleep(limit);
                tx.send(true)
            }
        });
        
        // lexer will go through the whole program before parsing
        let tokens = tokenize(code, &self.lang)?;
        let mut factory = ASTFactory::new(tokens);

        let mut stdout = Vec::new();
        let mut exit = Object::None;
        
        // the ast parsing is done statement by statement instead of all at once
        while let Some(stmt) = factory.produce() {
            if let Some(value) = stmt?.run(&mut environ, &mut stdout)? {
                // early return occurs
                exit = value;
                break
            }
        }
        
        // flush the global variable scope, reset if error occurs
        self.global = environ.body.pop()
            .unwrap_or(Scope::new(HashMap::new()));

        Ok(Summary {
            duration: start.elapsed(),
            stdout: stdout.join("\n"),
            code: exit
        })
    }
}


/// Abstraction of interaction with runtime environment
pub struct Context<'a> {
    /// Vector of arguments that the program passed in
    pub args: Vec<Object>,
    out: &'a mut Vec<String>
}

impl Context<'_> {
    /// Write a string to output buffer
    pub fn write(&mut self, s: String) {
        self.out.push(s)
    }
}


/// Abstraction of return value from a rust function
pub struct Output {
    result: Result<Object, RuntimeError>
}

impl Output {
    /// Throw an error to interpreter
    pub fn error(msg: String) -> Self {
        Self { result: Err(RuntimeError::FromRust { s: msg }) }
    }
}

impl From<Object> for Output {
    fn from(value: Object) -> Self {
        Self { result: Ok(value) }
    }
}


/// Available languages
#[derive(Clone)]
pub enum Language {
    /// Chinese
    CN,
    /// English
    EN
}

impl FromStr for Language {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lang = match s.to_ascii_lowercase().as_str() {
            "en" | "eng" => Self::EN,
            "cn" | "chn" => Self::CN,
            _ => return Err("")
        };
        Ok(lang)
    }
}
