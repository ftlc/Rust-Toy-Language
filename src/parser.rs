use types::ExprS;
// use std::borrow::ToOwned;

pub fn parse(s : &str) -> ExprS {
   
    let tokens = tokenize(s);
    println!("Vector: {:?}", tokens);
    build_exp(tokens)
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

fn build_exp(mut tokens : Vec<String>) -> ExprS {

    if tokens.len() == 0 {
        panic!("Syntax error: Unexpected EOF while parsing!");
    } 

    let token : String = tokens.remove(0);
    let token_str : &str = token.as_str();
    match token_str {
        _ => atomic(token_str),
    }
}

fn atomic(s : &str) -> ExprS {
    if let Some(n) = is_number(s) {
        ExprS::NumS(n)
    } else if let Some(b) = is_bool(s) {
        ExprS::BoolS(b)
    } else {
        ExprS::IdS(s.to_string())
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
