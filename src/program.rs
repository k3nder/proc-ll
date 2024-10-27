use std::cell::RefCell;
use std::rc::Rc;
use log::debug;
use regex::Regex;
use crate::measure_time;
use crate::token::Token;
use crate::values::Values;

/// Contains the regex of the functions
lazy_static::lazy_static! {
    static ref FUNC_REGEX: Regex = Regex::new(r"(.+)\((.+)\)").unwrap();
}
/// is a part of the program, contains the context, i is capable of executing
/// Use example:
/// ```
/// use processor::program::{Program, ProgramBlock};
/// let mut program = Program::new();
/// let main = program.create_block();
///
/// // now use the main ProgramBlock
/// ```
pub struct ProgramBlock {
    pub context: crate::context::Context,
}
impl ProgramBlock {
    pub(crate) fn new() -> Self {
        ProgramBlock { context: crate::context::Context::new() }
    }
    pub(crate) fn new_from_context(context: crate::context::Context) -> Self {
        ProgramBlock { context }
    }
    /// Executes a new token
    /// Use example:
    /// ```
    /// use processor::program::{Program, ProgramBlock};
    ///
    /// let mut program = Program::new();
    /// let main = program.create_block();
    ///
    /// main.borrow_mut().push_internal_key("echo", |tok, prog| {
    ///     print!("{}" ,tok);
    ///     processor::values::Values::Null
    /// });
    /// main.borrow_mut().exec("echo hello world!!");
    pub fn exec(&mut self, token: &str) -> Values {

        let token = token.trim();
        let token = token.replace("\n", "");

        // check keys
        debug!("[E] split token");
        let split: Vec<String> = measure_time!({token.to_owned().split_whitespace() // Divide la cadena en espacios
            .map(|s| s.to_string()) // Convierte cada &str a String
            .collect()}); // Recolecta en un Vec<String>

        if self.context.keys.contains_key(&split[0]) {
            debug!("[V] token is has key");
            let content = token.replace(&split[0], "").trim().to_string();
            debug!("[E] getting function");
            let func = measure_time!({self.context.keys.get(&split[0]).unwrap().clone()});

            return (func)(content, self);
        }

        // checking import
        debug!("[E] checking references");
        if token.starts_with("$") {
            debug!("[E] token is reference");
            let name = token.replace("$", "").trim().to_string();
            debug!("[E] getting memory value: {}", &name);
            return self.context.memory.get(&name).unwrap().clone();
        }

        // check functions
        debug!("[E] checking function");
        debug!("[REGEX] Matching regex");
        if measure_time!({FUNC_REGEX.is_match(&token)}) {
            debug!("[V] token is function");
            debug!("[E] getting function infos");
            let (name, args): (String, String) = if let Some(cap) = FUNC_REGEX.captures(&token) {
                debug!("[V] getting name");
                let name = measure_time!({cap.get(1).map_or("", |m| m.as_str()).to_string()});
                debug!("[E] getting args");
                let args = measure_time!({cap.get(2).map_or("", |m| m.as_str()).to_string()});
                (name, args)
            } else { ("".to_owned(), "".to_owned()) };
            debug!("[INF] infos {} {}", name, args);
            if !self.context.functions.contains_key(&name) { panic!("Function {} not found", name); }
            let func = { self.context.functions.get(&name).unwrap().clone() };
            return (func)(args, self);
        }

        // if is not key
        debug!("[E] other tokens definition");
        debug!("[E] getting token definition index");
        let index = measure_time!({self.context.token_index(&token).unwrap()});
        debug!("[E] getting token");
        let def_tok = measure_time!({self.context.get_token(index)});

        debug!("[E] executing function");
        let val = measure_time!({def_tok.borrow().exec(&token, self).unwrap()});

        val
    }
    /// Push a new token in the context
    pub fn push_internal_token(&mut self, token: Box<dyn Token>) {
        self.context.push_token(token);
    }
    /// push a new key on the context
    pub fn push_internal_key(&mut self, key: &str, func: impl Fn(String, &mut ProgramBlock) -> Values + 'static) {
        self.context.push_key(key.to_owned(), func);
    }
    /// push a new value on the context
    pub fn push_internal_memory(&mut self, key: &str, val: Values) {
        self.context.push_memory(key, val);
    }
    /// push a new function on the context
    pub fn push_internal_function(&mut self, name: &str, func: impl Fn(String, &mut ProgramBlock) -> Values + 'static) {
        self.context.push_function(name.to_owned(), func);
    }
}
/// structure used to build ProgramBlock
/// Use example:
/// ```
/// use processor::program::Program;
/// let mut prog = Program::new();
/// let program_block = prog.create_block();
/// ```
pub struct Program {
    pub(crate) blocks: Vec<Rc<RefCell<ProgramBlock>>>,
}
impl Program {
    /// Create a new Program instance
    pub fn new() -> Self {
        Program { blocks: vec![] }
    }
    /// Create a new ProgramBlock
    pub fn create_block(&mut self) -> Rc<RefCell<ProgramBlock>> {
        let block = Rc::new(RefCell::new(ProgramBlock::new()));
        self.blocks.push(block.clone());
        block
    }
}