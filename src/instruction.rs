
#[derive(Debug)]
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
    Call(usize),
    Ret,
}

#[derive(Debug)]
pub enum Instruction {
    OpCode(OpCode),
    Ref(Ref),
    Label(String)
}

#[derive(Debug)]
pub enum Ref {
    Jmp(String),
    JmpIf(String),
    Call(String),
}
