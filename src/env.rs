use std::rc::Rc;
use crate::{err::RuntimeError, val::ExpVal};

#[derive(Debug, Clone)]
pub struct Env(Rc<EnvInner>);

#[derive(Debug, Clone)]
enum EnvInner {
    EmptyEnv,
    ExtendEnv(String, ExpVal, Env),
}

impl Env {
    // empty-env: () -> Env
    pub fn empty() -> Self {
        Env(Rc::new(EnvInner::EmptyEnv))
    }

    // extend-env: Env * Var * Val -> Env
    pub fn extend(&self, var: String, val: ExpVal) -> Self {
        Env(Rc::new(EnvInner::ExtendEnv(var, val, self.clone())))
    }

    // apply-env: Env * Var -> Val
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
        }
    }

    // empty-env?: Env -> Bool
    pub fn is_empty_env(&self) -> bool {
        match &*self.0 {
            EnvInner::EmptyEnv => true,
            _ => false,
        }
    }

    // has-binding?: Env * Var -> Bool
    pub fn has_binding(&self, var: &str) -> bool {
        match self.apply(var) {
            Ok(_) => true,
            _ => false,
        }
    }
}