use crate::err::RuntimeError;

#[derive(Debug, Clone, PartialEq)]
pub enum ExpVal {
    Int(i32),
    Bool(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub enum DenVal {
    Int(i32),
    Bool(bool),
}

impl ExpVal {
    // num-val: Int -> ExpVal
    pub fn num_val(num: i32) -> ExpVal {
        ExpVal::Int(num)
    }

    // bool-val: Bool -> ExpVal
    pub fn bool_val(b: bool) -> ExpVal {
        ExpVal::Bool(b)
    }

    // expval->num: ExpVal -> Int
    pub fn expval_to_num(&self) -> Result<i32, RuntimeError> {
        match self {
            ExpVal::Int(num) => Ok(*num),
            _ => Err(RuntimeError::TypeError(format!("Expected Int, but got {:?}", self)))
        }
    }

    // expval->bool: ExpVal -> Bool
    pub fn expval_to_bool(&self) -> Result<bool, RuntimeError> {
        match self {
            ExpVal::Bool(b) => Ok(*b),
            _ => Err(RuntimeError::TypeError(format!("Expected Bool, but got {:?}", self)))
        }
    }
}