use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use crate::token::Token;
use crate::program::ProgramBlock;
use crate::values::Values;
/// structure that contains all the data at the count level, saves the variables, tokens, functions and keys
/// Use example :
/// ```
///  use procc_ll::context::Context;
///  let mut context = Context::new();
///  context.push_key("key".to_owned(), |token, prog| {
///     prog.exec(&token);
///     procc_ll::values::Values::Null
///  });
/// ```
pub struct Context {
    pub(crate) tokens: Vec<Rc<RefCell<Box<dyn Token + 'static>>>>,
    pub(crate) functions: HashMap<String, Arc<dyn Fn(String, &mut ProgramBlock) -> Values>>,
    pub(crate) memory: HashMap<String , Values>,
    pub(crate) keys: HashMap<String, Arc<dyn Fn(String, &mut ProgramBlock) -> Values>>
}

impl Context {
    /// Create a new instance of Context
    pub fn new<'b>() -> Context {
        Context {
            tokens: Vec::new(),
            memory: HashMap::new(),
            keys: HashMap::new(),
            functions: HashMap::new()
        }
    }
    /// Localize the index in the list of tokens of the token
    ///
    /// # PANICS
    /// Return a panic if the token is not registered on the context
    pub fn token_index(&self, tok: &str) -> usize {
        for ix in 0..self.tokens.len() {
            let def_tok = &self.tokens[ix];
            if def_tok.borrow().is_token(&tok) { return ix; }
        }
        panic!("Token \"{}\" no registered", tok);
    }
    /// Register a new token in te context
    pub fn push_token(&mut self, tok: Box<dyn Token>) {
        self.tokens.push(Rc::new(RefCell::new(tok)));
    }
    /// Push a new data on the memory
    pub fn push_memory(&mut self, key: &str, tok: Values) {
        self.memory.insert(key.to_owned(),tok);
    }
    /// Register a new key
    pub fn push_key(&mut self, key: String, tok: impl Fn(String, &mut ProgramBlock) -> Values + 'static) {
        self.keys.insert(key, Arc::new(tok));
    }
    /// Add a new function
    pub fn push_function(&mut self, name: String, func: impl Fn(String, &mut ProgramBlock) -> Values + 'static) {
        self.functions.insert(name, Arc::new(func));
    }
    pub(crate) fn get_token(&self, idx: usize) -> Rc<RefCell<Box<dyn Token + 'static>>> {
        self.tokens.get(idx).unwrap().clone()
    }
}