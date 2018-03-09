use std::fmt;
use types::ExprS;
// use std::borrow::ToOwned;


#[derive(Debug, PartialEq)]
enum Sexp {
    Bool (bool),
    Num (i32),
    Sym (String),
    List (Vec<Sexp>)
}

pub fn parse(s : &str) -> ExprS {
    let mut tokens = tokenize(s);
    println!("Vector: {:?}", tokens);
    let s : Sexp = build_sexp(&mut tokens);
    println!("SEXPL {}", s);

    parse_sexp(&s)
}


fn is_number(s: &str) -> Option<i32> {
    match s.parse::<i32>() {
        Ok(n) => Option::Some(n),
        Err(_) => Option::None
    }
}

fn is_bool(s: &str) -> Option<bool> {
    match s.parse::<bool>() {
        Ok(n) => Option::Some(n),
        Err(_) => Option::None
    }
}

fn parse_sexp(sexp : &Sexp) -> ExprS {
    match *sexp {
        Sexp::Num(n) => ExprS::NumS(n),
        Sexp::Bool(b) => ExprS::BoolS(b),
        Sexp::Sym(ref s) => ExprS::IdS(s.clone()),
        Sexp::List(ref list) => {
            let first = &list[0]; //Gross
            match *first {
                Sexp::Sym(ref s) => {
                    match s.as_ref() {
                        "+" => {
                            let second = &list[1];
                            let third = &list[2];
                            ExprS::PlusS{
                                l : Box::new(parse_sexp(second)),
                                r : Box::new(parse_sexp(third))
                            }
                        }
                        "*" => {
                            let second = &list[1];
                            let third = &list[2];
                            ExprS::MultS{
                                l : Box::new(parse_sexp(second)),
                                r : Box::new(parse_sexp(third))
                            }
                        }
                        "-" => {
                            let second = &list[1];
                            let third = &list[2];
                            ExprS::MinusS{
                                l : Box::new(parse_sexp(second)),
                                r : Box::new(parse_sexp(third))
                            }
                        }

                        _ => panic!("Not implemented yet")
                    }
                }
                _ => panic!("Not valid arithmetic!")
            }
        }
    }
}

fn build_sexp(tokens : &mut Vec<String>) -> Sexp {
    if tokens.len() == 0 {
        panic!("Syntax error: Unexpected EOF while parsing!");
    } 

    let token : String = tokens.remove(0);
    let token_str : &str = token.as_str();
    match token_str {
        "(" => {
            let mut v : Vec<Sexp> = Vec::new();
            while tokens[0] != ")" {
                v.push(build_sexp(tokens));
            }
            tokens.remove(0);
            Sexp::List(v)
        }
        _ => atomic(token_str),
    }
}

fn atomic(s : &str) -> Sexp {
    if let Some(n) = is_number(s) {
        Sexp::Num(n)
    } else if let Some(b) = is_bool(s) {
        Sexp::Bool(b)
    } else {
        Sexp::Sym(s.to_string())
    }
}

fn tokenize(s: &str) -> Vec<String> {
    let mut exp = s.replace("(", " ( ");
    exp = exp.replace(")", " ) ");
    let mut v: Vec<String> = exp.split_whitespace()
        .map(|s| s.to_string())
        .collect();
    v.retain(|i| !i.is_empty());
    return v;
}



impl fmt::Display for Sexp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Sexp::Num(n) => write!(f, "{}", n),
            Sexp::Bool(b) => write!(f, "{}", b),
            Sexp::Sym(ref s) => write!(f, "{}", s),
            Sexp::List(ref v) =>{
                write!(f, "[ {:?} ]", v)
            }
        }
    }
}
