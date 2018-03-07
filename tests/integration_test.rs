extern crate pl;
use pl::types::{Value, ExprS};

#[test]
fn tests_num() {
    assert_eq!(Value::NumV(15), pl::run("15"));
}

#[test]
fn tests_bool() {
    assert_eq!(Value::BoolV(true), pl::run("true"));
}

#[test]
fn tests_mult() {
    assert_eq!(
        Value::NumV(10), 
        pl::run_pre_parser(
            ExprS::MultS {
                l : Box::new(ExprS::NumS(5)),
                r : Box::new(ExprS::NumS(2)),
            }))
}


#[test]
fn tests_plus() {
    assert_eq!(
        Value::NumV(7), 
        pl::run_pre_parser(
            ExprS::PlusS {
                l : Box::new(ExprS::NumS(5)),
                r : Box::new(ExprS::NumS(2)),
            }))
}
