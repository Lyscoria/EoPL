use std::fmt;

#[derive(Debug)]
pub struct Program {
    pub exp: Exp,
}

#[derive(Debug, Clone)]
pub struct RecProc {
    pub name: String,
    pub vars: Vec<String>,
    pub body: Exp,
}

#[derive(Debug, Clone)]
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
    CondExp(Vec<(Box<Exp>, Box<Exp>)>),

    VarExp(String),
    LetExp(Vec<(String, Box<Exp>)>, Box<Exp>),
    LetStarExp(Vec<(String, Box<Exp>)>, Box<Exp>),
    UnpackExp(Vec<String>, Box<Exp>, Box<Exp>),

    ProcExp(Vec<String>, Box<Exp>),
    CallExp(Box<Exp>, Vec<Exp>),
    LetProcExp(String, Vec<String>, Box<Exp>, Box<Exp>),
    LetRecExp(Vec<RecProc>, Box<Exp>),

    BeginExp(Vec<Exp>),

    LetMutExp(Vec<(String, Box<Exp>)>, Box<Exp>),
    AssignExp(String, Box<Exp>),
}

impl fmt::Display for Exp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Exp::ConstExp(n) => write!(f, "{}", n),

            Exp::MinusExp(e) => write!(f, "minus({})", e),
            Exp::DiffExp(e1, e2) => write!(f, "-({}, {})", e1, e2),
            Exp::AddExp(e1, e2) => write!(f, "+({}, {})", e1, e2),
            Exp::MulExp(e1, e2) => write!(f, "*({}, {})", e1, e2),
            Exp::DivExp(e1, e2) => write!(f, "/({}, {})", e1, e2),

            Exp::IsZeroExp(e) => write!(f, "zero?({})", e),
            Exp::IsEqualExp(e1, e2) => write!(f, "equal?({}, {})", e1, e2),
            Exp::IsGreaterExp(e1, e2) => write!(f, "greater?({}, {})", e1, e2),
            Exp::IsLessExp(e1, e2) => write!(f, "less?({}, {})", e1, e2),

            Exp::EmptyListExp => write!(f, "emptylist"),
            Exp::ConsExp(e1, e2) => write!(f, "cons({}, {})", e1, e2),
            Exp::CarExp(e) => write!(f, "car({})", e),
            Exp::CdrExp(e) => write!(f, "cdr({})", e),
            Exp::IsNullExp(e) => write!(f, "null?({})", e),
            Exp::ListExp(exps) => {
                write!(f, "list(")?;
                for (i, e) in exps.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", e)?;
                }
                write!(f, ")")
            }

            Exp::PrintExp(e) => write!(f, "print({})", e),

            Exp::IfExp(e1, e2, e3) => 
                write!(f, "if {} then {} else {}", e1, e2, e3),
            Exp::CondExp(clauses) => {
                write!(f, "cond")?;
                for (test, res) in clauses {
                    write!(f, " {} ==> {}", test, res)?;
                }
                write!(f, " end")
            }

            Exp::VarExp(v) => write!(f, "{}", v),
            Exp::LetExp(bindings, body) => {
                write!(f, "let")?;
                for (var, e) in bindings {
                    write!(f, " {} = {}", var, e)?;
                }
                write!(f, " in {}", body)
            }
            Exp::LetStarExp(bindings, body) => {
                write!(f, "let*")?;
                for (var, e) in bindings {
                    write!(f, " {} = {}", var, e)?;
                }
                write!(f, " in {}", body)
            }
            Exp::UnpackExp(vars, e, body) => {
                write!(f, "unpack")?;
                for var in vars {
                    write!(f, " {}", var)?;
                }
                write!(f, " = {} in {}", e, body)
            }

            Exp::ProcExp(vars, body) => {
                write!(f, "proc(")?;
                for (i, var) in vars.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", var)?;
                }
                write!(f, ") {}", body)
            }

            Exp::CallExp(rator, rands) => {
                write!(f, "({}", rator)?;
                for rand in rands {
                    write!(f, " {}", rand)?;
                }
                write!(f, ")")
            }

            Exp::LetProcExp(name, vars, body, let_body) => {
                write!(f, "letproc {}(", name)?;
                for (i, var) in vars.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", var)?;
                }
                write!(f, ") = {} in {}", body, let_body)
            }

            Exp::LetRecExp(procs, body) => {
                write!(f, "letrec")?;
                for b in procs {
                    write!(f, " {}(", b.name)?;
                    for (i, var) in b.vars.iter().enumerate() {
                        if i > 0 { write!(f, ", ")?; }
                        write!(f, "{}", var)?;
                    }
                    write!(f, ") = {}", b.body)?;
                }
                write!(f, " in {}", body)
            }

            Exp::BeginExp(exps) => {
                write!(f, "list(")?;
                for (i, e) in exps.iter().enumerate() {
                    if i > 0 { write!(f, "; ")?; }
                    write!(f, "{}", e)?;
                }
                write!(f, ")")
            }

            Exp::LetMutExp(bindings, body) => {
                write!(f, "let mut")?;
                for (var, e) in bindings {
                    write!(f, " {} = {}", var, e)?;
                }
                write!(f, " in {}", body)
            }

            Exp::AssignExp(var, exp) => write!(f, "set {} = {}", var, exp),
        }
    }
}