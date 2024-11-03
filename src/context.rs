use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use crate::token::Token;
use crate::Program;
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
#[derive(Clone)]
pub struct Context {
    pub(crate) tokens: Rc<RefCell<Vec<Rc<RefCell<Box<dyn Token + 'static>>>>>>,
    pub(crate) functions: Rc<RefCell<HashMap<String, Arc<dyn Fn(Vec<Values>, &mut Program) -> Values>>>>,
    pub(crate) memory: Rc<RefCell<HashMap<String , Values>>>,
    pub(crate) keys: Rc<RefCell<HashMap<String, Arc<dyn Fn(String, &mut Program) -> Values>>>>,
    pub(crate) sub_context: Option<Box<Context>>
}

impl Context {
    /// Create a new instance of Context
    pub fn new<'b>() -> Context {
        Context {
            tokens: Rc::new(RefCell::new(Vec::new())),
            memory: Rc::new(RefCell::new(HashMap::new())),
            keys: Rc::new(RefCell::new(HashMap::new())),
            functions: Rc::new(RefCell::new(HashMap::new())),
            sub_context: None
        }
    }
    /// Localize the index in the list of tokens of the token
    ///
    /// # PANICS
    /// Return a panic if the token is not registered on the context
    pub fn token_index(&self, tok: &str) -> usize {
        for ix in 0..self.tokens.borrow().len() {
            let def_tok = &self.tokens.borrow()[ix];
            if def_tok.borrow().is_token(&tok) { return ix; }
        }
        panic!("Token \"{}\" no registered", tok);
    }
    /// Register a new token in te context
    pub fn push_token(&mut self, tok: Box<dyn Token>) {
        self.tokens.borrow_mut().push(Rc::new(RefCell::new(tok)));
    }
    /// Push a new data on the memory
    pub fn push_memory(&mut self, key: &str, tok: Values) {
        if !self.memory.borrow().contains_key(key) {
            if let Some(subc) = &mut self.sub_context {
                subc.push_memory(key, tok);
                return;
            }
        }
        self.memory.borrow_mut().insert(key.to_owned(),tok);
    }
    /// Register a new key
    pub fn push_key(&mut self, key: String, tok: impl Fn(String, &mut Program) -> Values + 'static) {
        self.keys.borrow_mut().insert(key, Arc::new(tok));
    }
    /// Add a new function
    pub fn push_function(&mut self, name: String, func: impl Fn(Vec<Values>, &mut Program) -> Values + 'static) {
        if let Some(subc) = &mut self.sub_context {
            if !self.functions.borrow().contains_key(&name) {
                subc.push_function(name, func);
                return;
            }
        }
        self.functions.borrow_mut().insert(name, Arc::new(func));
    }
    pub fn get_token(&self, idx: usize) -> Rc<RefCell<Box<dyn Token + 'static>>> {
        self.tokens.borrow().get(idx).unwrap().clone()
    }
    pub fn get_memory(&self, key: &str) -> Values {
        if !self.memory.borrow().contains_key(key) {
            if let Some(subc) = &self.sub_context {
                if subc.memory.borrow().contains_key(key) {
                    return subc.get_memory(key);
                } else {
                    panic!("Memory \"{}\" no longer exists", key);
                }
            } else {
                panic!("Memory {} not pushed", key);
            }
        }
        self.memory.borrow().get(key).unwrap().clone()
    }
    pub fn get_key(&self, key: &str) -> Arc<dyn Fn(String, &mut Program) -> Values> {
        self.keys.borrow().get(key).unwrap().clone()
    }
    pub fn get_function(&self, key: &str) -> Arc<dyn Fn(Vec<Values>, &mut Program) -> Values> {
        if !self.functions.borrow().contains_key(key) {
            if let Some(subc) = &self.sub_context {
                if subc.functions.borrow().contains_key(key) {
                    return subc.get_function(key);
                } else {
                    panic!("Function \"{}\" no longer exists", key);
                }
            } else {
                panic!("Function {} not pushed", key);
            }
        }
        self.functions.borrow().get(key).unwrap().clone()
    }


    pub fn has_token(&self, idx: usize) -> bool {
        self.tokens.borrow().len() > idx
    }
    pub fn has_memory(&self, key: &str) -> bool {
        let has_sub_context = {
            if let Some(subc) = &self.sub_context {
                subc.has_memory(key)
            } else {
                false
            }
        };
        self.memory.borrow().contains_key(key) || has_sub_context
    }
    pub fn has_key(&self, key: &str) -> bool {
        self.keys.borrow().contains_key(key)
    }
    pub fn has_function(&self, key: &str) -> bool {
        let has_sub_context = {
            if let Some(subc) = &self.sub_context {
                subc.has_function(key)
            } else {
                false
            }
        };
        self.functions.borrow().contains_key(key) || has_sub_context
    }

    pub fn gifs_token(&self, idx: usize) -> Option<Rc<RefCell<Box<dyn Token + 'static>>>> {
        if self.has_token(idx) {
           Some(self.get_token(idx))
        } else { None }
    }
    pub fn gifs_memory(&self, key: &str) -> Option<Values> {
        if self.has_memory(key) {
            Some(self.get_memory(key))
        } else { None }
    }
    pub fn gifs_key(&self, key: &str) -> Option<Arc<dyn Fn(String, &mut Program) -> Values>> {
        if self.has_memory(key) {
            Some(self.get_key(key))
        } else { None }
    }
    pub fn gifs_function(&self, key: &str) -> Option<Arc<dyn Fn(String, &mut Program) -> Values>> {
        if self.has_memory(key) {
            Some(self.get_key(key))
        } else { None }
    }
}