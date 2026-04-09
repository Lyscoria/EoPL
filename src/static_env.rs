use crate::err::TranslateError;

#[derive(Debug, Clone)]
pub struct StaticEnv {
    pub scopes: Vec<Vec<String>>,
}

impl StaticEnv {
    pub fn empty() -> Self {
        StaticEnv { scopes: Vec::new() }
    }

    pub fn extend(&self, vars: Vec<String>) -> Self{
        let mut new_scopes = self.scopes.clone();
        new_scopes.insert(0, vars);
        StaticEnv { scopes: new_scopes }
    }

    pub fn apply(&self, var: &str) -> Result<(usize, usize), TranslateError> {
        for (depth, scope) in self.scopes.iter().enumerate() {
            if let Some(offset) = scope.iter().position(|x| x == var) {
                return Ok((depth, offset));
            }
        }
        Err(TranslateError::NoBindingFound(format!("No binding for {} in current environment.", var)))
    }
}