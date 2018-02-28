use parser1::parse_Exprs;
use ast::Expr;
use instruction::{Instruction, OpCode};

pub struct Compiler {
    instructions: Vec<Instruction>,
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler { instructions: Vec::new() }
    }

    fn process_node(&mut self, n: Expr) {
        match n {
            Expr::Literal{value: i} => self.instructions.push(Instruction::OpCode(OpCode::Push(i))),
            Expr::Binary{left, operator, right} => {
                self.process_node(*left);
                self.process_node(*right);

                let op = match operator.as_str() {
                    "+" => OpCode::Add,
                    "-" => OpCode::Subtract,
                    "*" => OpCode::Multiply,
                    _ => panic!("invalid operator {}", operator),
                };
                self.instructions.push(Instruction::OpCode(op))
            }
        }
    }

    pub fn generate_instructions(&mut self, exprs: Vec<Box<Expr>>) -> Vec<Instruction> {
        for e in exprs {
            self.process_node(*e);
        }
        self.instructions.push(Instruction::OpCode(OpCode::Halt));
        self.instructions.clone()
    }
}

#[test]
fn parses_addition() {
    let r = parse_Exprs("(1 + 3) * 12;");
    println!("{:#?}", r);

    let mut p = Compiler { instructions: Vec::new() };
    println!("{:#?}", p.generate_instructions(r.unwrap()));
}
