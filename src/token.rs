use crate::Program;
use crate::values::Values;
///trait that is used to define a structure, contains one method to run the token and another to verify that the token is compatible
pub trait Token {
    /// Execute the token, requires the string token and the program block
    fn exec(&self, input: &str, program: &mut Program) -> Option<Values>;
    /// Verify it is a token
    fn is_token(&self, c: &str) -> bool;
}

