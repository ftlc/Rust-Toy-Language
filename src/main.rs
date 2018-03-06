extern crate pl;

fn main(){ 
    let val = pl::run(String::from("(+ 1 2)"));
    println!("Answer is: {}", val) 
}

