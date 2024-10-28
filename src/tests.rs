use std::string::String;
use regex::Regex;
use crate::Program;
use crate::token::Token;
use crate::values::Values;
use crate::values::Values::{Null};
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
    assert_eq!(value, Values::Null);
}

#[test]
fn execute_function() {
    let main = Program::new();
    main.borrow_mut().push_internal_function("echo", |token, _| {
        Values::String(token)
    });

    let res = main.borrow_mut().exec("echo(hello world)");
    assert_eq!(res, Values::String("hello world".to_string()));
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

        x
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