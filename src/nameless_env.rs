use crate::val::ExpVal;
use crate::err::RuntimeError;

#[derive(Debug, Clone)]
pub struct NamelessEnv {
    pub scopes: Vec<Vec<ExpVal>>,
}

impl NamelessEnv {
    pub fn empty() -> Self {
        NamelessEnv {
            scopes: Vec::new(),
        }
    }

    pub fn extend(&self, vals: Vec<ExpVal>) -> Self {
        let mut new_scopes = self.scopes.clone();
        new_scopes.insert(0, vals);
        NamelessEnv {
            scopes: new_scopes,
        }
    }

    pub fn apply(&self, depth: usize, offset: usize) -> Result<ExpVal, RuntimeError> {
        self.scopes.get(depth) 
            .and_then(|scope| scope.get(offset))
            .cloned()
            .ok_or_else(|| {
                RuntimeError::NoBindingFound(format!(
                    "Nameless lookup failed at depth {}, offset {}",
                    depth, offset
                ))
            })
    }

    pub fn apply_val(&self, depth: usize, offset: usize) -> Result<ExpVal, RuntimeError> {
        self.apply(depth, offset)
    }
}