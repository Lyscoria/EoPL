#[derive(Debug)]
pub struct Program {
    pub exp: Exp,
}

#[derive(Debug)]
pub enum Exp {
    ConstExp(i32),

    MinusExp(Box<Exp>),
    DiffExp(Box<Exp>, Box<Exp>),
    AddExp(Box<Exp>, Box<Exp>),
    MulExp(Box<Exp>, Box<Exp>),
    DivExp(Box<Exp>, Box<Exp>),

    IsZeroExp(Box<Exp>),
    IsEqualExp(Box<Exp>, Box<Exp>),
    IsGreaterExp(Box<Exp>, Box<Exp>),
    IsLessExp(Box<Exp>, Box<Exp>),

    ListExp(Vec<Exp>),
    EmptyListExp,
    ConsExp(Box<Exp>, Box<Exp>),
    CarExp(Box<Exp>),
    CdrExp(Box<Exp>),
    IsNullExp(Box<Exp>),

    PrintExp(Box<Exp>),

    IfExp(Box<Exp>, Box<Exp>, Box<Exp>),

    VarExp(String),

    CondExp(Vec<(Box<Exp>, Box<Exp>)>),
    
    LetExp(Vec<(String, Box<Exp>)>, Box<Exp>),
    LetStarExp(Vec<(String, Box<Exp>)>, Box<Exp>),
    
    UnpackExp(Vec<String>, Box<Exp>, Box<Exp>),
}