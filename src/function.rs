#[derive(Debug)]
pub struct FunctionDefinition {
    pub arity: usize,
    pub name: String, // if a name is defined it's a constant
    pub label: String, // the jump label for the ref
    pub instruction_address: Option<usize>,
}

pub struct NativeFunctionDefinition {
    pub arity: usize,
    pub name: Option<String>, // if a name is defined it's a constant
}
