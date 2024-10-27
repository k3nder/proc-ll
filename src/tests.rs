use regex::Regex;
use crate::program::{Program, ProgramBlock};
use crate::token::Token;
use crate::values::Values;
use crate::values::Values::Null;
#[test]
fn create_program() {
    let mut program = Program::new();
    let _ = program.create_block();
}

#[test]
fn execute_key() {
    let mut program = Program::new();
    let main = program.create_block();
    main.borrow_mut().push_internal_key("log", |token, _| {
        print!("{:?}", &token);
        Values::Null
    });
    let value = main.borrow_mut().exec("log  \"hello world\"");
    assert_eq!(value, Values::Null);
}

#[test]
fn execute_function() {
    let mut program = Program::new();
    let main = program.create_block();
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
        fn exec(&self, input: &str, _: &mut ProgramBlock) -> Option<Values> {
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


    let mut program = Program::new();
    let main = program.create_block();

    main.borrow_mut().push_internal_token(TokenT::new());

    main.borrow_mut().exec("//hello world");

}