use value::Value;
use function::FunctionDefinition;
use std::fmt;

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
    NEq,
    IsGt,
    IsGe,
    IsLt,
    IsLe,
    Jmp(usize),
    JmpIf(usize),
    LoadLocal(usize),
    StoreLocal(usize),
    Load(usize, usize),
    Store(usize, usize),
    Print,
    DebugPrint,
    Debugger,
    Call(usize),
    CallFn,
    DefineFunction(FunctionDefinition),
    PushFrame,
    PopFrame,
    Constant(usize),
    Ret,
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use instruction::OpCode::*;
        let s = match self {
            &NoOp => format!("NO_OP"),
            &Halt => format!("HALT"),
            &Push(ref v) => format!("PUSH {}", v),
            &Add => format!("ADD"),
            &Subtract => format!("SUB"),
            &Multiply => format!("MUL"),
            &Not => format!("NOT"),
            &And => format!("AND"),
            &Or => format!("OR"),
            &Pop => format!("POP"),
            &Dup => format!("DUP"),
            &IsEq => format!("IS_EQ"),
            &NEq => format!("N_EQ"),
            &IsGt => format!("IS_GT"),
            &IsGe => format!("IS_GE"),
            &IsLt => format!("IS_LT"),
            &IsLe => format!("IS_LE"),
            &Jmp(i) => format!("JMP {}", i),
            &JmpIf(i) => format!("JMP_IF {}", i),
            &LoadLocal(i) => format!("LOAD_LOCAL {}", i),
            &StoreLocal(i) => format!("STORE_LOCAL {}", i),
            &Load( idx, i) => format!("LOAD {} {}", idx, i),
            &Store(idx, i) => format!("STORE {} {}", idx, i),
            &Print => format!("PRINT"),
            &DebugPrint => format!("DEBUG_PRINT"),
            &Debugger => format!("DEBUGGER"),
            &DefineFunction(_) => format!("DEFINE_FUNCTION"),
            &Call(i) => format!("CALL {}", i),
            &CallFn => format!("CALL_FN"),
            &PushFrame => format!("PUSH_FRAME"),
            &PopFrame => format!("POP_FRAME"),
            &Constant(i) => format!("CONSTANT {}", i),
            &Ret => format!("RET"),
        };
        f.pad(s.as_str())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    OpCode(OpCode),
    Ref(Ref),
    Label(String),
    Comment(String),
    Local(String, usize)
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use instruction::Instruction::*;
        let s = match self {
            &OpCode(ref o) => format!("{}", o),
            &Ref(ref r) => format!("{:?}", r),
            &Label(ref s) => format!("{}:", s),
            &Comment(ref s) => format!("//{}", s),
            &Local(ref s, i) => format!("LOCAL {} {}", s, i),
        };
        f.pad(s.as_str())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Ref {
    Jmp(String),
    JmpIf(String),
    Call(String),
}
