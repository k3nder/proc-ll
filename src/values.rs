use std::fmt::{Debug, Formatter};
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