extern crate pl;
use pl::{Value, ExprS};

#[test]
fn calls_run() {
    assert_eq!(Value::NumV(15), pl::run(String::from("Test")))
}
