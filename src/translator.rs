use crate::{ast::{Exp, Program}, err::TranslateError, nameless_ast::{NamelessExp, NamelessProgram, NamelessRecProc}, static_env::StaticEnv};

pub fn translate_program(program: Program) -> Result<NamelessProgram, TranslateError> {
    let init_senv = StaticEnv::empty().extend(vec![
        "i".to_string(), 
        "v".to_string(), 
        "x".to_string()
    ]);
    Ok(NamelessProgram { 
        exp: translate_of(program.exp, &init_senv)? 
    })
}

pub fn translate_of(exp: Exp, senv: &StaticEnv) -> Result<NamelessExp, TranslateError> {
    match exp {
        Exp::ConstExp(n) => Ok(NamelessExp::ConstExp(n)),

        Exp::AddExp(exp1, exp2) =>
            Ok(NamelessExp::AddExp(
                Box::new(translate_of(*exp1, senv)?),
                Box::new(translate_of(*exp2, senv)?)
            )),

        Exp::DiffExp(exp1, exp2) =>
            Ok(NamelessExp::DiffExp(
                Box::new(translate_of(*exp1, senv)?),
                Box::new(translate_of(*exp2, senv)?)
            )),

        Exp::MulExp(exp1, exp2) =>
            Ok(NamelessExp::MulExp(
                Box::new(translate_of(*exp1, senv)?),
                Box::new(translate_of(*exp2, senv)?)
            )),

        Exp::DivExp(exp1, exp2) =>
            Ok(NamelessExp::DivExp(
                Box::new(translate_of(*exp1, senv)?),
                Box::new(translate_of(*exp2, senv)?)
            )),

        Exp::MinusExp(exp) =>
            Ok(NamelessExp::MinusExp(
                Box::new(translate_of(*exp, senv)?)
            )),

        Exp::IsZeroExp(exp) =>
            Ok(NamelessExp::IsZeroExp(
                Box::new(translate_of(*exp, senv)?)
            )),

        Exp::IsEqualExp(exp1, exp2) =>
            Ok(NamelessExp::IsEqualExp(
                Box::new(translate_of(*exp1, senv)?),
                Box::new(translate_of(*exp2, senv)?)
            )),

        Exp::IsGreaterExp(exp1, exp2) =>
            Ok(NamelessExp::IsGreaterExp(
                Box::new(translate_of(*exp1, senv)?),
                Box::new(translate_of(*exp2, senv)?)
            )),

        Exp::IsLessExp(exp1, exp2) =>
            Ok(NamelessExp::IsLessExp(
                Box::new(translate_of(*exp1, senv)?),
                Box::new(translate_of(*exp2, senv)?)
            )),

        Exp::ListExp(exps) => {
            let res: Result<Vec<NamelessExp>, TranslateError> = exps.into_iter().map(
                |exp| translate_of(exp, senv)).collect();
            Ok(NamelessExp::ListExp(res?))
        }

        Exp::EmptyListExp => Ok(NamelessExp::EmptyListExp),

        Exp::IsNullExp(exp) =>
            Ok(NamelessExp::IsNullExp(
                Box::new(translate_of(*exp, senv)?)
            )),

        Exp::ConsExp(exp1, exp2) =>
            Ok(NamelessExp::ConsExp(
                Box::new(translate_of(*exp1, senv)?),
                Box::new(translate_of(*exp2, senv)?)
            )),
        
        Exp::CarExp(exp) =>
            Ok(NamelessExp::CarExp(
                Box::new(translate_of(*exp, senv)?)
            )),

        Exp::CdrExp(exp) =>
            Ok(NamelessExp::CdrExp(
                Box::new(translate_of(*exp, senv)?)
            )),

        Exp::PrintExp(exp) =>
            Ok(NamelessExp::PrintExp(
                Box::new(translate_of(*exp, senv)?)
            )),

        Exp::IfExp(exp1, exp2, exp3) =>
            Ok(NamelessExp::IfExp(
                Box::new(translate_of(*exp1, senv)?),
                Box::new(translate_of(*exp2, senv)?),
                Box::new(translate_of(*exp3, senv)?)
            )),

        Exp::CondExp(clauses) => {
            let res = clauses.into_iter().map(|(exp1, exp2)| 
                {Ok((
                    Box::new(translate_of(*exp1, senv)?),
                    Box::new(translate_of(*exp2, senv)?)
                ))}).collect::<Result<Vec<_>, _>>();
            Ok(NamelessExp::CondExp(res?))
        }

        Exp::VarExp(var) => {
            let (depth, offset) = senv.apply(&var)?;
            Ok(NamelessExp::VarExp(depth, offset))
        }

        Exp::LetExp(bindings, body) => {
            let mut names = Vec::new();
            let mut nl_exps = Vec::new();
            for (name, exp) in bindings {
                names.push(name);
                nl_exps.push(translate_of(*exp, senv)?)
            }
            let new_senv = senv.extend(names);
            let nl_body = translate_of(*body, &new_senv)?;
            Ok(NamelessExp::LetExp(nl_exps, Box::new(nl_body)))
        }

        Exp::LetStarExp(bindings, body) => {
            let mut cur_senv = senv.clone();
            let mut nl_exps = Vec::new();
            for (name, exp) in bindings {
                nl_exps.push(translate_of(*exp, &cur_senv)?);
                cur_senv = cur_senv.extend(vec![name]);
            }
            let nl_body = translate_of(*body, &cur_senv)?;
            Ok(NamelessExp::LetStarExp(nl_exps, Box::new(nl_body)))
        }

        Exp::UnpackExp(vars, exp, body) => {
            let nl_exp = translate_of(*exp, senv)?;
            let new_senv = senv.extend(vars.clone());
            let nl_body = translate_of(*body, &new_senv)?;
            Ok(NamelessExp::UnpackExp(vars.len(), Box::new(nl_exp), Box::new(nl_body)))
        }

        Exp::ProcExp(vars, body) => {
            let new_senv = senv.extend(vars.clone());
            Ok(NamelessExp::ProcExp(vars.len(), Box::new(translate_of(*body, &new_senv)?)))
        }

        Exp::CallExp(rator, rands) => {
            let nl_rator = translate_of(*rator, senv)?;
            let nl_rands = rands.into_iter().map(|rand|
                translate_of(rand, senv)).collect::<Result<Vec<_>, _>>();
            Ok(NamelessExp::CallExp(Box::new(nl_rator), nl_rands?))
        }

        Exp::LetProcExp(name, vars, proc_body, let_body) => {
            let proc_senv = senv.extend(vars.clone());
            let nl_proc_body = translate_of(*proc_body, &proc_senv)?;
            let new_senv = senv.extend(vec![name]);
            let nl_let_body = translate_of(*let_body, &new_senv)?;
            Ok(NamelessExp::LetProcExp(vars.len(), Box::new(nl_proc_body), Box::new(nl_let_body)))
        }

        Exp::LetRecExp(procs, body) => {
            let proc_names: Vec<String> = procs.iter().map(|proc| proc.name.clone()).collect();
            let new_senv = senv.extend(proc_names);
            let nl_procs = procs.into_iter().map(|proc| {
                let proc_senv = new_senv.extend(proc.vars.clone());
                Ok(NamelessRecProc {
                    arg_num: proc.vars.len(),
                    body: translate_of(proc.body, &proc_senv)?,
                })
            }).collect::<Result<Vec<_>, _>>();
            let nl_body = translate_of(*body, &new_senv)?;
            Ok(NamelessExp::LetRecExp(nl_procs?, Box::new(nl_body)))
        }
    }
}