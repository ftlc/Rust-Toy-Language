extern crate sexp; 
use std::fmt;

fn main(){ 
    let val = run(String::from("(+ 1 2)"));
    println!("Answer is: {}", val) 
}


enum ExprC {
    NumC (i32),
    PlusC {l: Box<ExprC>, r: Box<ExprC>}
}


pub enum ExprS {
    NumS (i32),
    PlusS {l: Box<ExprS>, r: Box<ExprS>}
}

#[derive(Debug, PartialEq)]
pub enum Value {
    NumV (i32),
    _BoxV (i32),
}

fn parse(_s : String) -> ExprS {
    ExprS::PlusS {
        l : Box::new(ExprS::NumS(8)),
        r : Box::new(ExprS::NumS(7)),
    }
}

fn desugar(expr_s : ExprS ) ->ExprC {
    match expr_s {
        ExprS::NumS(n) => ExprC::NumC(n),
        ExprS::PlusS { l, r } => {
            let l_c = desugar(*l);
            let r_c = desugar(*r);
            return ExprC::PlusC { l: Box::new(l_c), r: Box::new(r_c) };
        },
    }
}
fn interp(expr_c : ExprC) -> Value {
    match expr_c {
        ExprC::NumC(n) => Value::NumV(n),
        ExprC::PlusC {l, r} => {
            let l_n = interp(*l);
            let r_n = interp(*r);
            return num_plus(l_n, r_n);
        },
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

pub fn run(s : String) -> Value {
    interp(desugar(parse( s )))
}

pub fn run_pre_parser(expr_s : ExprS) -> Value {
    interp(desugar(expr_s))
}



impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::NumV(n) => write!(f, "{}", n),
            Value::_BoxV(l) => write!(f, "{}", l),
        }
    }
}
