use std::rc::Rc;
use crate::{ast::{Exp, RecProc}, err::RuntimeError, val::{ExpVal, Proc}};

#[derive(Debug, Clone)]
pub struct Env(Rc<EnvInner>);

#[derive(Debug, Clone)]
enum EnvInner {
    EmptyEnv,
    ExtendEnv(String, ExpVal, Env),
    Rec(Vec<RecDef>, Env),
}

#[derive(Debug, Clone)]
struct RecDef {
    name: String,
    vars: Vec<String>,
    body: Exp,
}

impl Env {
    pub fn empty() -> Self {
        Env(Rc::new(EnvInner::EmptyEnv))
    }

    pub fn extend(&self, var: String, val: ExpVal) -> Self {
        Env(Rc::new(EnvInner::ExtendEnv(var, val, self.clone())))
    }

    pub fn extend_rec(&self, procs: &[RecProc]) -> Self {
        let defs = procs.iter().map(|p| RecDef {
            name: p.name.clone(),
            vars: p.vars.clone(),
            body: p.body.clone(),
        }).collect();
        Env(Rc::new(EnvInner::Rec(defs, self.clone())))
    }

    pub fn apply(&self, search_var: &str) -> Result<ExpVal, RuntimeError> {
        match &*self.0 {
            EnvInner::EmptyEnv => Err(RuntimeError::NoBindingFound(
                format!("No binding for {} in current environment: {:?}", search_var, self))),
            EnvInner::ExtendEnv(saved_var, saved_val, saved_env) => {
                if saved_var == search_var {
                    Ok(saved_val.clone())
                } else {
                    saved_env.apply(search_var)
                }
            }
            EnvInner::Rec(defs, env) => {
                if let Some(def) = defs.iter().find(|d| d.name == search_var) {
                    Ok(ExpVal::Proc(Proc {
                        vars: def.vars.clone(),
                        body: def.body.clone(),
                        env: self.clone(),
                    }))
                } else {
                    env.apply(search_var)
                }
            }
        }
    }

    pub fn is_empty_env(&self) -> bool {
        match &*self.0 {
            EnvInner::EmptyEnv => true,
            _ => false,
        }
    }

    pub fn has_binding(&self, var: &str) -> bool {
        match self.apply(var) {
            Ok(_) => true,
            _ => false,
        }
    }
}