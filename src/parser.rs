use types::ExprS;

pub fn parse(s : &str) -> ExprS {
    let mut s = s.replace("(", " ( ");
    s = s.replace(")", " ) ");
    let v: Vec<&str> =s.split(' ').collect();
    println!("{:?}", v);
    




    ExprS::PlusS {
        l : Box::new(ExprS::NumS(8)),
        r : Box::new(ExprS::NumS(7)),
    }
}
