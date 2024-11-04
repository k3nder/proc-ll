use std::cell::RefCell;
use std::rc::Rc;
use log::debug;
use regex::Regex;
use crate::context::Context;
use crate::token::Token;
use crate::values::Values;

pub mod context;
pub mod values;
pub mod token;
#[cfg(test)]
mod tests;
///measures the execution time and prints it on the screen,
/// example:
///```
/// use procc_ll::measure_time;
/// measure_time! ({println! ("hello ") });
/// ```
/// return the result of the code inside, only print the execution time if the RUST_LOG = debug
#[macro_export]
macro_rules! measure_time_debug {
    ($expression:expr) => {{
        use std::time::Instant;

        let start = Instant::now();
        let result = $expression;  // Ejecuta la expresión
        let duration = start.elapsed();

        debug!("Run Time: {:?}", duration);

        result  // Retorna el resultado de la expresión
    }};
}
#[macro_export]
macro_rules! measure_time {
    ($expression:expr) => {{
        use std::time::Instant;

        let start = Instant::now();
        let result = $expression;  // Ejecuta la expresión
        let duration = start.elapsed();

        println!("Run Time: {:?}", duration);

        result  // Retorna el resultado de la expresión
    }};
}


lazy_static::lazy_static! {
    static ref FUNC_REGEX: Regex = Regex::new(r"(.+)\((.+)\)").unwrap();
}
/// is a part of the program, contains the context, i is capable of executing
/// Use example:
/// ```
/// use procc_ll::Program;
/// let mut main = Program::new();
///
/// // now use the main ProgramBlock
/// ```

#[derive(Clone)]
pub struct Program {
    pub context: crate::context::Context,
}
impl Program {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Program { context: crate::context::Context::new() }))
    }
    pub fn new_from_context(context: crate::context::Context) -> Self {
        Program { context }
    }
    /// Executes a new token
    /// Use example:
    /// ```
    /// use procc_ll::Program;
    ///
    /// let mut main = Program::new();
    ///
    /// main.borrow_mut().push_internal_key("echo", |tok, prog| {
    ///     print!("{}" ,tok);
    ///     procc_ll::values::Values::Null
    /// });
    /// main.borrow_mut().exec("echo hello world!!");
    pub fn exec(&mut self, token: &str) -> Values {

        let token = token.trim();
        let token = token.replace("\n", "");

        // check keys
        debug!("[E] split token");
        let split: Vec<String> = measure_time_debug!({token.to_owned().split_whitespace() // Divide la cadena en espacios
            .map(|s| s.to_string()) // Convierte cada &str a String
            .collect()}); // Recolecta en un Vec<String>

        if self.context.keys.borrow().contains_key(&split[0]) {
            debug!("[V] token is has key");
            let content = token.replace(&split[0], "").trim().to_string();
            debug!("[E] getting function");
            let func = measure_time_debug!({self.context.get_key(&split[0])});
            return func(content, self);
        }

        // checking import
        debug!("[E] checking references");
        if token.starts_with("$") {
            debug!("[E] token is reference");
            let name = token.replace("$", "").trim().to_string();
            debug!("[E] getting memory value: {}", &name);
            return self.context.get_memory(&name);
        }

        // check functions
        debug!("[E] checking function");
        debug!("[REGEX] Matching regex");
        if measure_time_debug!({FUNC_REGEX.is_match(&token)}) {
            debug!("[V] token is function");
            debug!("[E] getting function infos");
            let (name, args): (String, String) = if let Some(cap) = FUNC_REGEX.captures(&token) {
                debug!("[V] getting name");
                let name = measure_time_debug!({cap.get(1).map_or("", |m| m.as_str()).to_string()});
                debug!("[E] getting args");
                let args = measure_time_debug!({cap.get(2).map_or("", |m| m.as_str()).to_string()});
                (name, args)
            } else { ("".to_owned(), "".to_owned()) };
            debug!("[INF] infos {} {}", name, args);
            if !self.context.functions.borrow().contains_key(&name) { panic!("Function {} not found", name); }
            let func = { self.context.get_function(&name) };

            let args = args.split(",").map(|s| self.exec(s)).collect::<Vec<Values>>();

            return (func)(args, self);
        }

        // if is not key
        debug!("[E] other tokens definition");
        debug!("[E] getting token definition index");
        let index = measure_time_debug!({self.context.token_index(&token)});
        debug!("[E] getting token");
        let def_tok = measure_time_debug!({self.context.get_token(index)});

        debug!("[E] executing function");
        let val = measure_time_debug!({def_tok.borrow().exec(&token, self).unwrap()});

        val
    }
    /// Push a new token in the context
    pub fn push_internal_token(&mut self, token: Box<dyn Token>) {
        self.context.push_token(token);
    }
    /// push a new key on the context
    pub fn push_internal_key(&mut self, key: &str, func: impl Fn(String, &mut Program) -> Values + 'static) {
        self.context.push_key(key.to_owned(), func);
    }
    /// push a new value on the context
    pub fn push_internal_memory(&mut self, key: &str, val: Values) {
        self.context.push_memory(key, val);
    }
    /// push a new function on the context
    pub fn push_internal_function(&mut self, name: &str, func: impl Fn(Vec<Values>, &mut Program) -> Values + 'static) {
        self.context.push_function(name.to_owned(), func);
    }
    pub fn new_depth_context(&mut self) -> Rc<RefCell<Program>> {
        let mut clone = self.clone();
        clone.context.sub_context = Some(Box::new(Context::new()));

        Rc::new(RefCell::new(clone))
    }
}
#[derive(Clone)]
pub enum Errors {
    Non,
    TokenNotMatched(String),
    FunctionNotFound(String),
}
impl Errors {
    pub fn to_str(&self) -> String {
        let (name, message) = match self {
            TokenNotMatched(msg) => ("ERRORS::TOKEN_NOT_FOUND", msg),
            Errors::FunctionNotFound(msg) => ("ERRORS::FUNCTION_NOT_FOUND", msg),
            _ => { ("ERRORS::UNKNOWN", &"UNKNOWN ERROR".to_owned()) }
        };
        format!("ERROR PROCESSING: {} : {}", name, message)
    }
}
impl Debug for Errors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*self.to_str())
    }
}
/// Values that retune the functions, tokens, keys i that also returns the exec of ProgramBlock
#[derive(PartialEq, Clone)]
pub enum Values {
    String(String),
    Number(f64),
    Boolean(bool),
    Array(Vec<Values>),
    Null
}
impl Debug for Values {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Values::String(v) => write!(f, "{}", v),
            Values::Number(v) => write!(f, "{}", v),
            Values::Boolean(v) => write!(f, "{}", v),
            Values::Array(v) => { write!(f, "[")?; v.fmt(f) },
            _ => { write!(f, "null") }
        }
    }
}
