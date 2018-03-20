use cpu::Closure;
use std::cmp::PartialEq;
use std::fmt;

#[derive(Debug, Clone)]
pub struct FunctionDefinition {
    pub arity: usize,
    pub parameters: Vec<String>,
    pub name: String, // if a name is defined it's a constant
    pub label: String, // the jump label for the ref
    pub instruction_address: Option<usize>,
}

pub struct FunctionPrototype {
    pub arity: usize,
    pub name: String, // if a name is defined it's a constant
    pub label: String, // the jump label for the ref
    pub instruction_address: usize, // since prototypes are created at runtime
    // from a function definition we know the address will be defined
    pub closure: Closure,
}

impl fmt::Debug for FunctionPrototype {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let closure_frames = &self.closure.frames.len();
        fmt.debug_struct("FunctionPrototype")
            .field("arity", &self.arity)
            .field("name", &self.name)
            .field("label", &self.label)
            .field("instruction_address", &self.instruction_address)
            .field("closure", &format!("({} frames)", closure_frames))
            .finish()
    }
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

impl PartialEq for FunctionPrototype {
    fn eq(&self, other: &FunctionPrototype) -> bool {
        self.arity == other.arity &&
            self.name == other.name && self.label == other.label
    }
}
