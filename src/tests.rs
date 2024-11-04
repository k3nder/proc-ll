use std::string::String;
use log::debug;
use regex::Regex;
use crate::Program;
use crate::token::Token;
use crate::Values;
use crate::Values::{Null};
#[test]
fn create_program() {
    let _ = Program::new();
}

#[test]
fn execute_key() {
    let main = Program::new();
    main.borrow_mut().push_internal_key("log", |token, _| {
        print!("{:?}", &token);
        Values::Null
    });
    let value = main.borrow_mut().exec("log  \"hello world\"");
    assert_eq!(value.unwrap(), Values::Null);
}
lazy_static::lazy_static! {
    static ref STRING_REGEX: Regex = Regex::new("\"(.+)\"").unwrap();
    static ref STRING_CONCAT_REGEX: Regex = Regex::new(r"\{\{([^{}]*)\}\}").unwrap();
}
pub struct StringToken;
impl Token for StringToken {
    fn exec(&self, input: &str, program: &mut Program) -> Option<Values> {
        let cont = if let Some(c) = STRING_REGEX.captures(input) {
            let mut c = c.get(0).map_or("", |m| m.as_str()).to_owned();
            c.remove(0);
            c.remove(c.len() - 1);

            let result = STRING_CONCAT_REGEX.replace_all(&c, move |caps: &regex::Captures| {
                // Obtenemos el contenido dentro de las llaves
                let content = &mut caps[1].to_string();

                debug!("STR CONCAT {}", content);

                let res = program.exec(content);
                let res = format!("{:?}", res);

                res
            });

            result.to_string()
        } else { "".to_owned() };
        Some(Values::String(cont))
    }

    fn is_token(&self, c: &str) -> bool {
        STRING_REGEX.is_match(c)
    }
}
impl StringToken {
    pub fn new() -> Box<Self> {
        Box::new(StringToken {})
    }
}
#[test]
fn execute_function() {
    let main = Program::new();

    main.borrow_mut().push_internal_token(StringToken::new());
    main.borrow_mut().push_internal_function("echo", |token, _| {
        println!("{:?}", &token[0]);
        token[0].clone()
    });

    let res = main.borrow_mut().exec("echo(\"hello world\")");
    assert_eq!(res.unwrap(), Values::String("hello world".to_string()));
}

#[test]
fn execute_token() {
    struct TokenT;
    impl TokenT {
        pub fn new() -> Box<TokenT> {
            Box::new(TokenT)
        }
    }
    impl Token for TokenT {
        fn exec(&self, input: &str, _: &mut Program) -> Option<Values> {
            let re = Regex::new("//(.+)").unwrap();

            if let Some(cap) = re.captures(input) {
                let c = cap.get(1).unwrap().as_str();
                print!("{}", c);
            }

            Some(Null)
        }

        fn is_token(&self, c: &str) -> bool {
            let re = Regex::new("//(.+)").unwrap();

            re.is_match(c)
        }
    }


    let main = Program::new();

    main.borrow_mut().push_internal_token(TokenT::new());

    main.borrow_mut().exec("//hello world");

}
#[test]
#[should_panic]
fn sub_context() {
    let program = Program::new();

    program.borrow_mut().push_internal_key("let", |token, prog| {
        let l: Vec<String> = token.split("=").map(|s| s.trim().to_string()).collect();

        let name = &l[0];
        let value = &l[1];

        prog.push_internal_memory(name, Values::String(value.clone()));

        Null
    });

    program.borrow_mut().push_internal_key("exec", |token, prog| {
        let sub_p = prog.new_depth_context();

        let x = sub_p.borrow_mut().exec(&token);

        x.unwrap()
    });

    program.borrow_mut().push_internal_key("log", |token, prog| {
        println!("{:?}", prog.exec(&token));
        Null
    });

    program.borrow_mut().exec("let i = hola");
    program.borrow_mut().exec("exec log $i");
    program.borrow_mut().exec("exec let e = nooo");
    program.borrow_mut().exec("log $e");
}

#[test]
#[should_panic]
fn not_found_func() {
    let prog = Program::new();
    prog.borrow_mut().exec("not(found)").unwrap();
}
#[test]
#[should_panic]
fn not_match_token() {
    let prog = Program::new();
    prog.borrow_mut().exec("not_found").unwrap();
}