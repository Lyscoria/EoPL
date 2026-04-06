use crate::{ast::{Exp, Program}, env::Env, err::RuntimeError, val::{ExpVal, Proc}};

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

        Exp::MinusExp(exp1) => {
            let val = value_of(exp1, env)?;
            Ok(ExpVal::Int(-val.as_num()?))
        }

        Exp::DiffExp(exp1, exp2) => {
            let val1 = value_of(exp1, env)?;
            let val2 = value_of(exp2, env)?;
            Ok(ExpVal::Int(val1.as_num()? - val2.as_num()?))
        }

        Exp::AddExp(exp1, exp2) => {
            let val1 = value_of(exp1, env)?;
            let val2 = value_of(exp2, env)?;
            Ok(ExpVal::Int(val1.as_num()? + val2.as_num()?))
        }

        Exp::MulExp(exp1, exp2) => {
            let val1 = value_of(exp1, env)?;
            let val2 = value_of(exp2, env)?;
            Ok(ExpVal::Int(val1.as_num()? * val2.as_num()?))
        }

        Exp::DivExp(exp1, exp2) => {
            let val1 = value_of(exp1, env)?;
            let val2 = value_of(exp2, env)?;
            if val2.as_num()? == 0 {
                return Err(RuntimeError::DivisonByZero("Division by zero".to_string()));
            }
            Ok(ExpVal::Int(val1.as_num()? / val2.as_num()?))
        }

        Exp::IsZeroExp(exp) => {
            let val = value_of(exp, env)?;
            Ok(ExpVal::Bool(val.as_num()? == 0))
        }

        Exp::IsEqualExp(exp1, exp2) => {
            let val1 = value_of(exp1, env)?;
            let val2 = value_of(exp2, env)?;
            Ok(ExpVal::Bool(val1.as_num()? == val2.as_num()?))
        }

        Exp::IsGreaterExp(exp1, exp2) => {
            let val1 = value_of(exp1, env)?;
            let val2 = value_of(exp2, env)?;
            Ok(ExpVal::Bool(val1.as_num()? > val2.as_num()?))
        }

        Exp::IsLessExp(exp1, exp2) => {
            let val1 = value_of(exp1, env)?;
            let val2 = value_of(exp2, env)?;
            Ok(ExpVal::Bool(val1.as_num()? < val2.as_num()?))
        }

        Exp::EmptyListExp => {
            Ok(ExpVal::List(None))
        }

        Exp::ConsExp(exp1, exp2) => {
            let val1 = value_of(exp1, env)?;
            let val2 = value_of(exp2, env)?;
            Ok(ExpVal::cons(val1, val2)?)
        }

        Exp::CarExp(exp) => {
            let val = value_of(exp, env)?;
            Ok(val.car()?)
        }

        Exp::CdrExp(exp) => {
            let val = value_of(exp, env)?;
            Ok(val.cdr()?)
        }

        Exp::IsNullExp(exp) => {
            let val = value_of(exp, env)?;
            Ok(ExpVal::Bool(val.is_null()?))
        }

        Exp::ListExp(exps) => {
            let mut val = ExpVal::empty_list();
            for exp in exps.iter().rev() {
                val = ExpVal::cons(value_of(exp, env)?, val)?;
            }
            Ok(val)
        }

        Exp::PrintExp(exp) => {
            let val = value_of(exp, env)?;
            println!("{}", val);
            Ok(ExpVal::Int(1))
        }

        Exp::IfExp(exp1, exp2, exp3) => {
            let val = value_of(exp1, env)?.as_bool()?;
            if val {
                value_of(exp2, env)
            } else {
                value_of(exp3, env)
            }
        }

        Exp::CondExp(clauses) => {
            for (test, res) in clauses {
                let val = value_of(test, env)?;
                if val.as_bool()? == true {
                    return value_of(res, env);
                }
            }
            Err(RuntimeError::CondError("No cond clause matched.".to_string()))
        }

        Exp::LetExp(bindings, body) => {
            let mut new_env = env.clone();
            for (var, exp) in bindings {
                let val = value_of(exp, env)?;
                new_env = new_env.extend(var.to_string(), val);
            }
            value_of(body, &new_env)
        }

        Exp::LetStarExp(bindings, body) => {
            let mut new_env = env.clone();
            for (var, exp) in bindings {
                let val = value_of(exp, &new_env)?;
                new_env = new_env.extend(var.to_string(), val);
            }
            value_of(body, &new_env)
        }

        Exp::UnpackExp(vars, exp, body) => {
            let vals = value_of(exp, env)?;
            let mut remain_vals = vals;
            let mut new_env = env.clone(); 
            for var in vars {
                let val = remain_vals.car()?;
                new_env = new_env.extend(var.to_string(), val);
                remain_vals = remain_vals.cdr()?;
            }
            if remain_vals.is_null()? == false {
                return Err(RuntimeError::UnpackError(format!("Unpack: length inconsistent")));
            }
            value_of(body, &new_env)
        }

        Exp::ProcExp(var, body) => {
            Ok(ExpVal::Proc(Proc{
                var: var.clone(), 
                body: *body.clone(),
                env: env.clone()
            }))
        }

        Exp::CallExp(rator, rand) => {
            let proc = value_of(rator, env)?.as_proc()?;
            let arg = value_of(rand, env)?;
            let new_env = proc.env.extend(proc.var, arg);
            value_of(&proc.body, &new_env)
        }
    }
}