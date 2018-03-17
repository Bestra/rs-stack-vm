use cpu::Closure;
use std::cmp::PartialEq;
#[derive(Debug, Clone)]
pub struct FunctionDefinition {
    pub arity: usize,
    pub name: String, // if a name is defined it's a constant
    pub label: String, // the jump label for the ref
    pub instruction_address: Option<usize>,
}

#[derive(Debug)]
pub struct FunctionPrototype {
    pub arity: usize,
    pub name: String, // if a name is defined it's a constant
    pub label: String, // the jump label for the ref
    pub instruction_address: Option<usize>,
    pub closure: Closure,
}

pub struct NativeFunctionDefinition {
    pub arity: usize,
    pub name: Option<String>, // if a name is defined it's a constant
}


impl PartialEq for FunctionDefinition {
    fn eq(&self, other: &FunctionDefinition) -> bool {
        self.arity == other.arity &&
            self.name == other.name && self.label == other.label
    }
}
