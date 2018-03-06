use value::Value;
#[derive(Debug, Clone, PartialEq)]
pub enum OpCode {
    NoOp,
    Halt,
    Push(Value),
    Add,
    Subtract,
    Multiply,
    Not,
    And,
    Or,
    Pop,
    Dup,
    IsEq,
    IsGt,
    IsGe,
    IsLt,
    Jmp(usize),
    JmpIf(usize),
    LoadLocal(usize),
    StoreLocal(usize),
    Load(usize, usize),
    Store(usize, usize),
    Print,
    DebugPrint,
    Call(usize),
    // PushFrame,
    // PopFrame,
    Constant(usize),
    Ret,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    OpCode(OpCode),
    Ref(Ref),
    Label(String),
    Comment(String),
    Local(String, usize)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Ref {
    Jmp(String),
    JmpIf(String),
    Call(String),
}
