use std::collections::HashMap;
use instruction::{Instruction, OpCode, Ref};
use value::Value;

pub struct AssemblyProgram {
    pub op_codes: Vec<OpCode>,
    pub constant_pool: Vec<Value>,
}

pub struct Assembler {
    program: Vec<Instruction>,
    labels: HashMap<String, usize>,
}

impl Assembler {
    pub fn new(program: Vec<Instruction>) -> Assembler {
        Assembler {
            program,
            labels: HashMap::new(),
        }
    }

    pub fn assemble(&mut self) -> Vec<OpCode> {
        self.resolve_labels();
        self.generate_op_codes()
    }

    pub fn resolve_labels(&mut self) {
        for (idx, instruction) in self.program.iter().enumerate() {
            if let Instruction::Label(ref s) = *instruction {
                self.labels.insert(s.to_string(), idx);
            }
        }
    }

    pub fn generate_op_codes(&mut self) -> Vec<OpCode> {
        self.program
            .iter()
            .map(|i| match *i {
                Instruction::Ref(ref r) => match *r {
                    Ref::Jmp(ref s) => {
                        let addr = self.labels.get(s).unwrap();
                        OpCode::Jmp(*addr)
                    }

                    Ref::JmpIf(ref s) => {
                        let addr = self.labels.get(s).unwrap();
                        OpCode::JmpIf(*addr)
                    }

                    Ref::Call(ref s) => {
                        let addr = self.labels.get(s).unwrap();
                        OpCode::Call(*addr)
                    }
                },
                Instruction::OpCode(ref o) => o.clone(),
                Instruction::Label(_) |
                Instruction::Local(_, _) |
                Instruction::Comment(_) => OpCode::NoOp,
            })
            .collect()
    }
}


#[test]
fn adds_labels_to_hash() {
    let mut a = Assembler::new(vec![
        Instruction::Label("bar".to_string()),
        Instruction::OpCode(OpCode::Push(Value::Number(5))),
        Instruction::Label("foo".to_string()),
    ]);
    a.resolve_labels();
    assert_eq!(*a.labels.get("bar").unwrap(), 0);
    assert_eq!(*a.labels.get("foo").unwrap(), 2);
}

#[test]
fn turns_refs_to_ops() {
    let mut a = Assembler::new(vec![
        Instruction::OpCode(OpCode::Push(Value::Number(5))),
        Instruction::Ref(Ref::Call("main".to_string())),
        Instruction::Label("main".to_string()),
        Instruction::OpCode(OpCode::Push(Value::Number(5))),
        Instruction::OpCode(OpCode::Push(Value::Number(5))),
    ]);
    a.resolve_labels();
    let ops = a.generate_op_codes();
    assert_eq!(ops[1], OpCode::Call(2));
}

#[test]
fn assembly_1() {
    let p = super::assembly::parse_Program("
CALL main

double:
STORE_LOCAL 1
STORE_LOCAL 0
LOAD_LOCAL 1
LOAD_LOCAL 0
PRINT
ISGE
PRINT
JIF double_a
LOAD_LOCAL 1
RET

double_a:
LOAD_LOCAL 0
RET

main:
PUSH 6
PUSH 4
PUSH \"HEY\"
CALL double
");

    let mut a = Assembler::new(p.unwrap());
    a.resolve_labels();
    println!("{:#?}", a.generate_op_codes());
}

