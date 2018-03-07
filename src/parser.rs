use types::ExprS;

pub fn parse(_s : &str) -> ExprS {
    ExprS::PlusS {
        l : Box::new(ExprS::NumS(8)),
        r : Box::new(ExprS::NumS(7)),
    }
}
