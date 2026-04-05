#[derive(Debug)]
pub struct Program {
    pub exp: Exp,
}

#[derive(Debug)]
pub enum Exp {
    ConstExp(i32),
    DiffExp(Box<Exp>, Box<Exp>),
    IsZeroExp(Box<Exp>),
    IfExp(Box<Exp>, Box<Exp>, Box<Exp>),
    VarExp(String),
    LetExp(String, Box<Exp>, Box<Exp>),
}