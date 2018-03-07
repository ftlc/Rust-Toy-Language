extern crate sexp; 
mod parser;
mod types;
use types::{ExprS, Value};
// type Definitions
// #[derive(Debug, PartialEq)]
// pub enum Value {
    // BoolV (bool),
    // NumV (i32),
    // _BoxV (i32),
// }

enum ExprC {
    BoolC (bool),
    NumC (i32),
    PlusC {l: Box<ExprC>, r: Box<ExprC>},
    MultC {l: Box<ExprC>, r: Box<ExprC>},
    
}

// pub enum ExprS {
    // BoolS (bool),
    // NumS (i32),
    // PlusS {l: Box<ExprS>, r: Box<ExprS>},
    // MultS {l: Box<ExprS>, r: Box<ExprS>}, 
    // MinusS {l: Box<ExprS>, r: Box<ExprS>}, 
// }

// fn parse(_s : &str) -> ExprS {
    // ExprS::PlusS {
        // l : Box::new(ExprS::NumS(8)),
        // r : Box::new(ExprS::NumS(7)),
    // }
// }

fn desugar(expr_s : ExprS ) ->ExprC {
    match expr_s {
        ExprS::NumS(n) => ExprC::NumC(n),
        ExprS::BoolS(b) => ExprC::BoolC(b),
        ExprS::PlusS { l, r } => {
            let l_c = desugar(*l);
            let r_c = desugar(*r);
            return ExprC::PlusC { l: Box::new(l_c), r: Box::new(r_c) };
        },
        ExprS::MultS { l, r } => {
            let l_c = desugar(*l);
            let r_c = desugar(*r);
            return ExprC::MultC { l: Box::new(l_c), r: Box::new(r_c) };
        }
        ExprS::MinusS { l, r } => {
            let l_c = desugar(*l);
            let r_c = desugar(*r);
            return ExprC::PlusC {
                l : Box::new(l_c),
                r : Box::new(
                    ExprC::MultC { 
                        l : Box::new(ExprC::NumC(-1)),
                        r : Box::new(r_c)
                    })
            }
        }
    }
}
fn interp(expr_c : ExprC) -> Value {
    match expr_c {
        ExprC::BoolC(b) => Value::BoolV(b),
        ExprC::NumC(n) => Value::NumV(n),
        ExprC::PlusC {l, r} => {
            let l_n = interp(*l);
            let r_n = interp(*r);
            return num_plus(l_n, r_n);
        },
        ExprC::MultC {l, r} => {
            let l_n = interp(*l);
            let r_n = interp(*r);
            return num_mult(l_n, r_n);
        }
    }
}

fn num_plus(l : Value, r : Value) -> Value {
    if let Value::NumV(n) = l {
        if let Value::NumV(m) = r {
            return Value::NumV(n + m)
        } else {
            panic!("Right side is not a number!")
        }

    } else {
        panic!("Left side is not a number!")
    }
}

fn num_mult(l : Value, r : Value) -> Value {
    if let Value::NumV(n) = l {
        if let Value::NumV(m) = r {
            return Value::NumV(n * m)
        } else {
            panic!("Right side is not a number!")
        }

    } else {
        panic!("Left side is not a number!")
    }

}

pub fn run(s : &str) -> Value {
    interp(desugar(parser::parse( s )))
}

pub fn run_pre_parser(expr_s : ExprS) -> Value {
    interp(desugar(expr_s))
}



// impl fmt::Display for Value {
    // fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // match *self {
            // Value::NumV(n) => write!(f, "{}", n),
            // Value::BoolV(b) => write!(f, "{}", b),
            // Value::_BoxV(l) => write!(f, "{}", l),
        // }
    // }
// }
