use crate::{err::RuntimeError, val::ExpVal};

pub struct Store {
    slots: Vec<ExpVal>,
}

impl Store {
    pub fn new() -> Self {
        Store { slots: Vec::new() }
    }

    pub fn newref(&mut self, val: ExpVal) -> usize {
        self.slots.push(val);
        self.slots.len() - 1
    }

    pub fn deref(&self, addr: usize) -> Result<ExpVal, RuntimeError> {
        self.slots.get(addr).cloned()
            .ok_or_else(|| RuntimeError::AddressError("Invalid address".into()))
    }
    
    pub fn setref(&mut self, addr: usize, val: ExpVal) -> Result<(), RuntimeError> {
        if addr < self.slots.len() {
            self.slots[addr] = val;
            Ok(())
        } else {
            Err(RuntimeError::AddressError("Invalid address".into())) 
        }
    }
}