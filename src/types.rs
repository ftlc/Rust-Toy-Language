use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Value {
    BoolV (bool),
    NumV (i32),
    _BoxV (i32),
}

pub enum ExprS {
    BoolS (bool),
    NumS (i32),
    PlusS {l: Box<ExprS>, r: Box<ExprS>},
    MultS {l: Box<ExprS>, r: Box<ExprS>}, 
    MinusS {l: Box<ExprS>, r: Box<ExprS>}, 
}


impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::NumV(n) => write!(f, "{}", n),
            Value::BoolV(b) => write!(f, "{}", b),
            Value::_BoxV(l) => write!(f, "{}", l),
        }
    }
}
