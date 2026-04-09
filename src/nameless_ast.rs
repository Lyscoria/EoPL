use std::fmt;

#[derive(Debug)]
pub struct NamelessProgram {
    pub exp: NamelessExp,
}

#[derive(Debug, Clone)]
pub struct NamelessRecProc {
    pub arg_num: usize,
    pub body: NamelessExp,
}

#[derive(Debug, Clone)]
pub enum NamelessExp {
    ConstExp(i32),

    MinusExp(Box<NamelessExp>),
    DiffExp(Box<NamelessExp>, Box<NamelessExp>),
    AddExp(Box<NamelessExp>, Box<NamelessExp>),
    MulExp(Box<NamelessExp>, Box<NamelessExp>),
    DivExp(Box<NamelessExp>, Box<NamelessExp>),

    IsZeroExp(Box<NamelessExp>),
    IsEqualExp(Box<NamelessExp>, Box<NamelessExp>),
    IsGreaterExp(Box<NamelessExp>, Box<NamelessExp>),
    IsLessExp(Box<NamelessExp>, Box<NamelessExp>),

    ListExp(Vec<NamelessExp>),
    EmptyListExp,
    ConsExp(Box<NamelessExp>, Box<NamelessExp>),
    CarExp(Box<NamelessExp>),
    CdrExp(Box<NamelessExp>),
    IsNullExp(Box<NamelessExp>),

    PrintExp(Box<NamelessExp>),

    IfExp(Box<NamelessExp>, Box<NamelessExp>, Box<NamelessExp>),
    CondExp(Vec<(Box<NamelessExp>, Box<NamelessExp>)>),

    VarExp(usize, usize),
    LetExp(Vec<NamelessExp>, Box<NamelessExp>),
    LetStarExp(Vec<NamelessExp>, Box<NamelessExp>),
    UnpackExp(usize, Box<NamelessExp>, Box<NamelessExp>),

    ProcExp(usize, Box<NamelessExp>),
    CallExp(Box<NamelessExp>, Vec<NamelessExp>),
    LetProcExp(usize, Box<NamelessExp>, Box<NamelessExp>),

    LetRecExp(Vec<NamelessRecProc>, Box<NamelessExp>),
}

impl fmt::Display for NamelessExp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NamelessExp::ConstExp(n) => write!(f, "{}", n),

            NamelessExp::MinusExp(e) => write!(f, "minus({})", e),
            NamelessExp::DiffExp(e1, e2) => write!(f, "-({}, {})", e1, e2),
            NamelessExp::AddExp(e1, e2) => write!(f, "+({}, {})", e1, e2),
            NamelessExp::MulExp(e1, e2) => write!(f, "*({}, {})", e1, e2),
            NamelessExp::DivExp(e1, e2) => write!(f, "/({}, {})", e1, e2),

            NamelessExp::IsZeroExp(e) => write!(f, "zero?({})", e),
            NamelessExp::IsEqualExp(e1, e2) => write!(f, "equal?({}, {})", e1, e2),
            NamelessExp::IsGreaterExp(e1, e2) => write!(f, "greater?({}, {})", e1, e2),
            NamelessExp::IsLessExp(e1, e2) => write!(f, "less?({}, {})", e1, e2),

            NamelessExp::EmptyListExp => write!(f, "emptylist"),
            NamelessExp::ConsExp(e1, e2) => write!(f, "cons({}, {})", e1, e2),
            NamelessExp::CarExp(e) => write!(f, "car({})", e),
            NamelessExp::CdrExp(e) => write!(f, "cdr({})", e),
            NamelessExp::IsNullExp(e) => write!(f, "null?({})", e),
            NamelessExp::ListExp(exps) => {
                write!(f, "list(")?;
                for (i, e) in exps.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", e)?;
                }
                write!(f, ")")
            }

            NamelessExp::PrintExp(e) => write!(f, "print({})", e),

            NamelessExp::IfExp(e1, e2, e3) => write!(f, "if {} then {} else {}", e1, e2, e3),
            NamelessExp::CondExp(clauses) => {
                write!(f, "cond")?;
                for (test, res) in clauses {
                    write!(f, " {} ==> {}", test, res)?;
                }
                write!(f, " end")
            }

            NamelessExp::VarExp(d, o) => write!(f, "#{}@{}", d, o),

            NamelessExp::LetExp(exps, body) => {
                write!(f, "let ")?;
                for e in exps { write!(f, "val={}, ", e)?; }
                write!(f, "in {}", body)
            }
            
            NamelessExp::LetStarExp(exps, body) => {
                write!(f, "let* ")?;
                for e in exps { write!(f, "val={}, ", e)?; }
                write!(f, "in {}", body)
            }

            NamelessExp::UnpackExp(count, e, body) => {
                write!(f, "unpack {} vars = {} in {}", count, e, body)
            }

            NamelessExp::ProcExp(count, body) => {
                write!(f, "proc(args:{}) {}", count, body)
            }

            NamelessExp::CallExp(rator, rands) => {
                write!(f, "({}", rator)?;
                for rand in rands {
                    write!(f, " {}", rand)?;
                }
                write!(f, ")")
            }

            NamelessExp::LetProcExp(count, body, let_body) => {
                write!(f, "letproc(args:{}) = {} in {}", count, body, let_body)
            }

            NamelessExp::LetRecExp(procs, body) => {
                write!(f, "letrec")?;
                for p in procs {
                    write!(f, " (args:{}) = {}", p.arg_num, p.body)?;
                }
                write!(f, " in {}", body)
            }
        }
    }
}