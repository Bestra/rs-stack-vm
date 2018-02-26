use std::collections::HashMap;
use instruction::{Instruction, OpCode, Ref};

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

    pub fn resolve_labels(&mut self) {
        for (idx, instruction) in self.program.iter().enumerate() {
            match *instruction {
                Instruction::Label(ref s) => {
                    self.labels.insert(s.to_string(), idx);
                }
                _ => (),
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
                Instruction::Label(ref s) => OpCode::Label(s.clone()),
                Instruction::OpCode(ref o) => o.clone(),
                Instruction::Comment(ref c) => OpCode::Comment(c.clone()),
            })
            .collect()
    }
}


#[test]
fn adds_labels_to_hash() {
    let mut a = Assembler::new(vec![
        Instruction::Label("bar".to_string()),
        Instruction::OpCode(OpCode::Push(5)),
        Instruction::Label("foo".to_string()),
    ]);
    a.resolve_labels();
    assert_eq!(*a.labels.get("bar").unwrap(), 0);
    assert_eq!(*a.labels.get("foo").unwrap(), 2);
}

#[test]
fn turns_refs_to_ops() {
    let mut a = Assembler::new(vec![
        Instruction::OpCode(OpCode::Push(5)),
        Instruction::Ref(Ref::Call("main".to_string())),
        Instruction::Label("main".to_string()),
        Instruction::OpCode(OpCode::Push(5)),
        Instruction::OpCode(OpCode::Push(5)),
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
STORE 1
STORE 0
LOAD 1
LOAD 0
PRINT
ISGE
PRINT
JIF double_a
LOAD 1
RET

double_a:
LOAD 0
RET

main:
PUSH 6
PUSH 4
CALL double
");

    let mut a = Assembler::new(p.unwrap());
    a.resolve_labels();
    println!("{:#?}", a.generate_op_codes());
}

