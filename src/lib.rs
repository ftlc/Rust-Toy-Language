extern crate sexp; 
mod parser;
pub mod types;
use types::ExprS;
use types::Value;


enum ExprC {
    BoolC (bool),
    NumC (i32),
    IdC (String),
    PlusC {l: Box<ExprC>, r: Box<ExprC>},
    MultC {l: Box<ExprC>, r: Box<ExprC>},
    IfC {c: Box<ExprC>, t: Box<ExprC>, e: Box<ExprC>},
}


fn desugar(expr_s : ExprS ) ->ExprC {
    match expr_s {
        ExprS::NumS(n) => ExprC::NumC(n),
        ExprS::BoolS(b) => ExprC::BoolC(b),
        ExprS::IdS(s) => ExprC::IdC(s),
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
        ExprS::IfS { c, t, e } => {
            ExprC::IfC { 
                c: Box::new(desugar(*c)),
                t: Box::new(desugar(*t)),
                e: Box::new(desugar(*e))
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
        },
        ExprC::IfC {c, t, e} => {
            if_imp(*c, *t, *e)
        }
        _ => panic!("Not implemented")
    }
}


fn if_imp(c: ExprC, t: ExprC, e: ExprC) -> Value {
    if let Value::BoolV(cond) = interp(c) {
        if cond {
            interp(t)
        } else {
            interp(e)
        }
    } else {
        panic!("Conditional is not a bool!")
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


