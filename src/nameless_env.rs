use std::rc::Rc;

use crate::nameless_ast::NamelessRecProc;
use crate::val::{ExpVal, NamelessProc};
use crate::err::RuntimeError;

#[derive(Debug, Clone)]
pub struct NamelessEnv(pub Rc<NamelessEnvInner>);

#[derive(Debug, Clone)]
pub enum NamelessEnvInner {
    EmptyEnv,
    ExtendEnv(Vec<ExpVal>, NamelessEnv),
    Rec(Vec<NamelessRecProc>, NamelessEnv),
}

impl NamelessEnv {
    pub fn empty() -> Self {
        NamelessEnv(Rc::new(NamelessEnvInner::EmptyEnv))
    }

    pub fn extend(&self, vals: Vec<ExpVal>) -> Self {
        NamelessEnv(Rc::new(NamelessEnvInner::ExtendEnv(vals, self.clone())))
    }

    pub fn apply(&self, depth: usize, offset: usize) -> Result<ExpVal, RuntimeError> {
        match (&*self.0, depth) {
            (NamelessEnvInner::EmptyEnv, _) => {
                Err(RuntimeError::NoBindingFound(format!("Address depth {} out of range", depth)))
            }
            
            (NamelessEnvInner::ExtendEnv(vals, _), 0) => {
                vals.get(offset)
                    .cloned()
                    .ok_or_else(|| RuntimeError::NoBindingFound(format!(
                        "Address offset {} out of range", offset)))
            }
            
            (NamelessEnvInner::ExtendEnv(_, env), d) => {
                env.apply(d - 1, offset)
            }

            (NamelessEnvInner::Rec(procs, env), d) => {
                if d == 0 {
                    let proc_def = procs.get(offset).ok_or_else(|| {
                        RuntimeError::NoBindingFound(format!("Rec offset {} out of range", offset))
                    })?;
                    Ok(ExpVal::NamelessProc(NamelessProc {
                        arg_num: proc_def.arg_num,
                        body: proc_def.body.clone(),
                        env: self.clone()
                    }))
                } else {
                    env.apply(d - 1, offset)
                }
            }
        }
    }
}