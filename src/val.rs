use core::fmt;
use std::rc::Rc;

use crate::{ast::Exp, env::Env, err::RuntimeError};

#[derive(Debug, Clone)]
pub enum ExpVal {
    Int(i32),
    Bool(bool),
    List(Option<Rc<ListNode>>),
    Proc(Proc),
}

#[derive(Debug)]
pub struct ListNode {
    pub head: ExpVal,
    pub tail: Option<Rc<ListNode>>,
}

#[derive(Debug, Clone)]
pub struct Proc {
    pub vars: Vec<String>,
    pub body: Exp,
    pub env: Env,
}

pub type DenVal = ExpVal;

impl ExpVal {
    pub fn num_val(num: i32) -> ExpVal {
        ExpVal::Int(num)
    }

    pub fn bool_val(b: bool) -> ExpVal {
        ExpVal::Bool(b)
    }

    pub fn as_num(&self) -> Result<i32, RuntimeError> {
        match self {
            ExpVal::Int(num) => Ok(*num),
            _ => Err(RuntimeError::TypeError(format!("Expected Int, but got {:?}", self)))
        }
    }

    pub fn as_bool(&self) -> Result<bool, RuntimeError> {
        match self {
            ExpVal::Bool(b) => Ok(*b),
            _ => Err(RuntimeError::TypeError(format!("Expected Bool, but got {:?}", self)))
        }
    }

    pub fn as_proc(&self) -> Result<Proc, RuntimeError> {
        match self {
            ExpVal::Proc(f) => Ok(f.clone()),
            _ => Err(RuntimeError::TypeError(format!("Expected Proc, but got {:?}", self)))
        }
    }

    pub fn empty_list() -> ExpVal {
        ExpVal::List(None)
    }

    pub fn cons(head: ExpVal, tail_val: ExpVal) -> Result<ExpVal, RuntimeError> {
        match tail_val {
            ExpVal::List(tail) => {
                let new_node = ListNode {head, tail};
                Ok(ExpVal::List(Some(Rc::new(new_node))))
            }
            _ => Err(RuntimeError::TypeError(format!("Expected List, but got {:?}", tail_val)))
        }
    }

    pub fn car(&self) -> Result<ExpVal, RuntimeError> {
        match self {
            ExpVal::List(Some(node)) => {
                Ok(node.head.clone())
            }
            ExpVal::List(None) => Err(RuntimeError::EmptyListError(format!("Car: Empty list"))),
            _ => Err(RuntimeError::TypeError(format!("Expected List, but got {:?}", self)))
        }
    }

    pub fn cdr(&self) -> Result<ExpVal, RuntimeError> {
        match self {
            ExpVal::List(Some(node)) => {
                Ok(ExpVal::List(node.tail.clone()))
            }
            ExpVal::List(None) => Err(RuntimeError::EmptyListError(format!("Cdr: Empty list"))),
            _ => Err(RuntimeError::TypeError(format!("Expected List, but got {:?}", self)))
        }
    }

    pub fn is_null(&self) -> Result<bool, RuntimeError> {
        match self {
            ExpVal::List(None) => Ok(true),
            ExpVal::List(Some(_)) => Ok(false),
            _ => Err(RuntimeError::TypeError(format!("Expected List, but got {:?}", self)))
        }
    }
}

impl fmt::Display for Proc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "proc(")?;
        for (i, var) in self.vars.iter().enumerate() {
            if i > 0 { write!(f, ", ")?; }
            write!(f, "{}", var)?;
        }
        write!(f, ") {}", self.body)
    }
}

impl fmt::Display for ExpVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExpVal::Int(n) => write!(f, "{}", n),
            ExpVal::Bool(b) => write!(f, "{}", b),
            ExpVal::List(l) => {
                write!(f, "(")?;
                let mut current = l;
                let mut first = true;
                while let Some(node) = current {
                    if !first { write!(f, " ")?; }
                    write!(f, "{}", node.head)?;
                    current = &node.tail;
                    first = false;
                }
                write!(f, ")")
            }
            ExpVal::Proc(p) => write!(f, "{}", p),
        }
    }
}