#[derive(Debug, Clone, PartialEq)]
pub enum OpCode {
    Halt,
    Push(i32),
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
    Load(usize),
    Store(usize),
    Print,
    DebugPrint,
    Call(usize),
    Ret,
    Label(String),
    Comment(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    OpCode(OpCode),
    Ref(Ref),
    Label(String),
    Comment(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Ref {
    Jmp(String),
    JmpIf(String),
    Call(String),
}
