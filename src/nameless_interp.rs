use std::rc::Rc;

use crate::nameless_ast::{NamelessExp, NamelessProgram};
use crate::nameless_env::{NamelessEnv, NamelessEnvInner};
use crate::err::RuntimeError;
use crate::val::{ExpVal, NamelessProc};

pub fn value_of_program(program: &NamelessProgram) -> Result<ExpVal, RuntimeError> {
    let init_env = NamelessEnv::empty()
        .extend(vec![
            ExpVal::Int(1),
            ExpVal::Int(5),
            ExpVal::Int(10),
        ]);
    value_of(&program.exp, &init_env)
}

pub fn value_of(exp: &NamelessExp, env: &NamelessEnv) -> Result<ExpVal, RuntimeError> {
    match exp {
        NamelessExp::ConstExp(num) => Ok(ExpVal::Int(*num)),

        NamelessExp::MinusExp(e) => {
            let val = value_of(e, env)?;
            Ok(ExpVal::Int(-val.as_num()?))
        }

        NamelessExp::DiffExp(e1, e2) => {
            let val1 = value_of(e1, env)?;
            let val2 = value_of(e2, env)?;
            Ok(ExpVal::Int(val1.as_num()? - val2.as_num()?))
        }

        NamelessExp::AddExp(e1, e2) => {
            let val1 = value_of(e1, env)?;
            let val2 = value_of(e2, env)?;
            Ok(ExpVal::Int(val1.as_num()? + val2.as_num()?))
        }

        NamelessExp::MulExp(e1, e2) => {
            let val1 = value_of(e1, env)?;
            let val2 = value_of(e2, env)?;
            Ok(ExpVal::Int(val1.as_num()? * val2.as_num()?))
        }

        NamelessExp::DivExp(e1, e2) => {
            let val1 = value_of(e1, env)?;
            let val2 = value_of(e2, env)?;
            let n2 = val2.as_num()?;
            if n2 == 0 {
                return Err(RuntimeError::DivisionByZero("Division by zero".to_string()));
            }
            Ok(ExpVal::Int(val1.as_num()? / n2))
        }

        NamelessExp::IsZeroExp(e) => {
            let val = value_of(e, env)?;
            Ok(ExpVal::Bool(val.as_num()? == 0))
        }

        NamelessExp::IsEqualExp(e1, e2) => {
            let val1 = value_of(e1, env)?;
            let val2 = value_of(e2, env)?;
            Ok(ExpVal::Bool(val1.as_num()? == val2.as_num()?))
        }

        NamelessExp::IsGreaterExp(e1, e2) => {
            let val1 = value_of(e1, env)?;
            let val2 = value_of(e2, env)?;
            Ok(ExpVal::Bool(val1.as_num()? > val2.as_num()?))
        }

        NamelessExp::IsLessExp(e1, e2) => {
            let val1 = value_of(e1, env)?;
            let val2 = value_of(e2, env)?;
            Ok(ExpVal::Bool(val1.as_num()? < val2.as_num()?))
        }

        NamelessExp::EmptyListExp => {
            Ok(ExpVal::empty_list())
        }

        NamelessExp::ConsExp(e1, e2) => {
            let val1 = value_of(e1, env)?;
            let val2 = value_of(e2, env)?;
            ExpVal::cons(val1, val2)
        }

        NamelessExp::CarExp(e) => {
            let val = value_of(e, env)?;
            val.car()
        }

        NamelessExp::CdrExp(e) => {
            let val = value_of(e, env)?;
            val.cdr()
        }

        NamelessExp::IsNullExp(e) => {
            let val = value_of(e, env)?;
            Ok(ExpVal::Bool(val.is_null()?))
        }

        NamelessExp::ListExp(exps) => {
            let mut res = ExpVal::empty_list();
            for e in exps.iter().rev() {
                res = ExpVal::cons(value_of(e, env)?, res)?;
            }
            Ok(res)
        }

        NamelessExp::PrintExp(e) => {
            let val = value_of(e, env)?;
            println!("{}", val);
            Ok(ExpVal::Int(1))
        }

        NamelessExp::IfExp(e1, e2, e3) => {
            let cond = value_of(e1, env)?.as_bool()?;
            if cond {
                value_of(e2, env)
            } else {
                value_of(e3, env)
            }
        }

        NamelessExp::CondExp(clauses) => {
            for (test, res) in clauses {
                if value_of(test, env)?.as_bool()? {
                    return value_of(res, env);
                }
            }
            Err(RuntimeError::CondError("No cond clause matched.".to_string()))
        }

        NamelessExp::VarExp(depth, offset) => {
            env.apply(*depth, *offset)
        }

        NamelessExp::LetExp(exps, body) => {
            let mut vals = Vec::new();
            for e in exps {
                vals.push(value_of(e, env)?);
            }
            let new_env = env.extend(vals);
            value_of(body, &new_env)
        }

        NamelessExp::LetStarExp(exps, body) => {
            let mut current_env = env.clone();
            for e in exps {
                let val = value_of(e, &current_env)?;
                current_env = current_env.extend(vec![val]);
            }
            value_of(body, &current_env)
        }

        NamelessExp::UnpackExp(count, e, body) => {
            let list_val = value_of(e, env)?;
            let mut vals = Vec::new();
            let mut current = list_val;
            for _ in 0..*count {
                vals.push(current.car()?);
                current = current.cdr()?;
            }
            if !current.is_null()? {
                return Err(RuntimeError::UnpackError("Unpack: length inconsistent".to_string()));
            }
            let new_env = env.extend(vals);
            value_of(body, &new_env)
        }

        NamelessExp::ProcExp(arg_num, body) => {
            Ok(ExpVal::NamelessProc(NamelessProc {
                arg_num: *arg_num,
                body: *body.clone(),
                env: env.clone(),
            }))
        }

        NamelessExp::CallExp(rator, rands) => {
            let proc = value_of(rator, env)?.as_nameless_proc()?;
            let mut args = Vec::new();
            for rand in rands {
                args.push(value_of(rand, env)?);
            }
            if args.len() != proc.arg_num {
                return Err(RuntimeError::ArgNumberError(format!(
                    "Expected {} args, but got {}", proc.arg_num, args.len())));
            }
            let new_env = proc.env.extend(args);
            value_of(&proc.body, &new_env)
        }

        NamelessExp::LetProcExp(arg_num, body, let_body) => {
            let proc = ExpVal::NamelessProc(NamelessProc {
                arg_num: *arg_num,
                body: *body.clone(),
                env: env.clone(),
            });
            let new_env = env.extend(vec![proc]);
            value_of(let_body, &new_env)
        }

        NamelessExp::LetRecExp(procs, body) => {
            let new_env = NamelessEnv(Rc::new(
                NamelessEnvInner::Rec(procs.clone(), env.clone())
            ));
            value_of(body, &new_env)
        }
    }
}