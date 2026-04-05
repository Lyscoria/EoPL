use crate::{ast::{Exp, Program}, env::Env, err::RuntimeError, val::ExpVal};

pub fn value_of_program(program: &Program) -> Result<ExpVal, RuntimeError> {
    let init_env = Env::empty()
        .extend("i".to_string(), ExpVal::Int(1))
        .extend("v".to_string(), ExpVal::Int(5))
        .extend("x".to_string(), ExpVal::Int(10));
    value_of(&program.exp, &init_env)
}

pub fn value_of(exp: &Exp, env: &Env) -> Result<ExpVal, RuntimeError> {
    match exp {
        Exp::ConstExp(num) => Ok(ExpVal::Int(*num)),

        Exp::VarExp(var) => env.apply(var),

        Exp::DiffExp(exp1, exp2) => {
            let val1 = value_of(exp1, env)?;
            let val2 = value_of(exp2, env)?;
            Ok(ExpVal::Int(val1.expval_to_num()? - val2.expval_to_num()?))
        }

        Exp::IsZeroExp(exp) => {
            let val = value_of(exp, env)?;
            Ok(ExpVal::Bool(val.expval_to_num()? == 0))
        }

        Exp::IfExp(exp1, exp2, exp3) => {
            let val = value_of(exp1, env)?.expval_to_bool()?;
            if val {
                value_of(exp2, env)
            } else {
                value_of(exp3, env)
            }
        }

        Exp::LetExp(var, exp1, exp2) => {
            let val = value_of(exp1, env)?;
            value_of(exp2, &env.extend(var.to_string(), val))
        }
    }
}