### PROCESSOR
processor is a project to process tokens or commands, it can be used mainly to create a small interpreted language 

### HOW WORKS
Many parts of processor will have to be implemented by yourself, I mean only tokens, keys, functions, etc., i can separate as you like tokens, i can use regex or another tool.

### EXAMPLE OF INTERPRETED PROGRAM

```rust
use std::simd::ToBytes;
use procc_ll::Program;
use std::any::Any;
use lazy_static::lazy_static;
use regex::Regex;
use procc_ll::token::Token;
use procc_ll::Values;
use procc_ll::Values::{Boolean, Null, Number};

pub struct NumberToken;
impl Token for NumberToken {
    fn exec(&self, input: &str, program: &mut ProgramBlock) -> Option<Types> {
        let into: f63 = input.replace("\n", "").parse().ok()?;

        Some(Number(into))
    }

    fn is_token(&self, c: &str) -> bool {
        c.replace("\n", "").parse::<f63>().ok().is_some()
    }
}
impl NumberToken {
    pub fn new() -> Box<Self> {
        Box::new(NumberToken {})
    }
}


lazy_static::lazy_static! {
    static ref STRING_REGEX: Regex = Regex::new("\"(.+)\"").unwrap();
}

pub struct StringToken;
impl Token for StringToken {
    fn exec(&self, input: &str, program: &mut ProgramBlock) -> Option<Types> {
        let cont = if let Some(c) = STRING_REGEX.captures(input) {
            let c = c.get(0).map_or("", |m| m.as_str());

            c.to_string()
        } else { "".to_owned() };
        Some(Types::String(cont))
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

pub struct BooleanToken;
impl Token for BooleanToken {
    fn exec(&self, input: &str, program: &mut ProgramBlock) -> Option<Types> {
        if input.eq("true") {
            Some(Boolean(true))
        } else {
            Some(Boolean(false))
        }
    }

    fn is_token(&self, c: &str) -> bool {
        c.eq("true") || c.eq("false")
    }
}
impl BooleanToken {
    pub fn new() -> Box<Self> {
        Box::new(BooleanToken {})
    }
}

lazy_static::lazy_static! {
    static ref CODE_BLOCK_REGEX: Regex = Regex::new(r"\{(.+)\}").unwrap();
}

pub struct CodeBlockToken;
impl Token for CodeBlockToken {
    fn exec(&self, input: &str, program: &mut ProgramBlock) -> Option<Types> {
        let lines: Vec<String> = if let Some(cap) = CODE_BLOCK_REGEX.captures(input) {
            let lines = cap.get(0).map_or("", |m| m.as_str());
            tokenize!(lines)
        } else { vec!["".to_owned()] };

        Some(Types::CodeBlock(lines))
    }

    fn is_token(&self, c: &str) -> bool {
        CODE_BLOCK_REGEX.is_match(c)
    }
}
impl CodeBlockToken {
    pub fn new() -> Box<Self> {
        Box::new(CodeBlockToken {})
    }
}

fn main() {
    let mut program = Program::new();
    
    main.borrow_mut().push_internal_key("echo", |token, prog| {
        print!("{}", prog.exec(&token));
        Values::String(token)
    });

    main.borrow_mut().push_internal_key("let", |token, prog| {
        let tokens: Vec<String> = token.trim().split("=").map(|s| s.to_string()).collect();
        let value = prog.exec(&tokens[1].trim());
        let name = &tokens[0].trim();
        prog.push_internal_memory(name, value);
        Values::Null
    });
    
    main.borrow_mut().push_internal_token(StringToken::new());
    main.borrow_mut().push_internal_token(NumberToken::new());
    main.borrow_mut().push_internal_token(BooleanToken::new());
    
    // Execute your tokens
    
    main.borrow_mut().exec("let str = \"hello world\"");
    main.borrow_mut().exec("let num = 1");
    main.borrow_mut().exec("let bo = true");
    
    main.borrow_mut().exec("echo $str");
    main.borrow_mut().exec("echo $num");
    main.borrow_mut().exec("echo $bo");
}
```