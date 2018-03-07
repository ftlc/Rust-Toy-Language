extern crate pl;
use pl::types::{Value, ExprS};

#[test]
fn calls_run() {
    assert_eq!(Value::NumV(15), pl::run("Test"));
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
