extern crate pl;
use pl::types::Value;

#[test]
fn tests_num() {
    assert_eq!(Value::NumV(15), pl::run("15"));
}

#[test]
fn tests_bool() {
    assert_eq!(Value::BoolV(true), pl::run("true"));
}

#[test]
fn tests_plus() {
    assert_eq!(Value::NumV(15), pl::run("(+ (+ 8 2) 5)"));
}

#[test]
fn tests_mult() {
    assert_eq!(Value::NumV(50), pl::run("(* (+ 8 2) 5)"));
}

#[test]
fn tests_minus() {
    assert_eq!(Value::NumV(5), pl::run("(- (+ 8 2) 5)"));
}

