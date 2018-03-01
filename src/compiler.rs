use parser1::parse_Program;
use ast::{Expr, Statement};
use instruction::{Instruction, OpCode};

pub struct Compiler {
    instructions: Vec<Instruction>,
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            instructions: Vec::new(),
        }
    }

    fn process_statements(&mut self, statements: Vec<Statement>) {
        for s in statements {
            self.process_statement(s);
        }
    }

    fn process_statement(&mut self, s: Statement) {
        match s {
            Statement::Expression { expression } => self.process_expr(*expression),
            Statement::Print { expression } => {
                let e = self.process_expr(*expression);
                self.instructions.push(Instruction::OpCode(OpCode::Print));
            }
        }
    }

    fn process_expr(&mut self, n: Expr) {
        match n {
            Expr::Literal { value: i } => {
                self.instructions.push(Instruction::OpCode(OpCode::Push(i)))
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                self.process_expr(*left);
                self.process_expr(*right);

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

    pub fn generate_instructions(&mut self, statements: Vec<Statement>) -> Vec<Instruction> {
        self.process_statements(statements);
        self.instructions.push(Instruction::OpCode(OpCode::Halt));
        self.instructions.clone()
    }
}

#[test]
fn compiles_simple_math() {
    let r = parse_Program("(1 + 3) * 12;");
    println!("{:#?}", r);

    let mut p = Compiler {
        instructions: Vec::new(),
    };
    let output = p.generate_instructions(r.unwrap());
    println!("{:#?}", output);
    assert_eq!(
        output,
        vec![
            Instruction::OpCode(OpCode::Push(1)),
            Instruction::OpCode(OpCode::Push(3)),
            Instruction::OpCode(OpCode::Add),
            Instruction::OpCode(OpCode::Push(12)),
            Instruction::OpCode(OpCode::Multiply),
            Instruction::OpCode(OpCode::Halt),
        ]
    );
}

#[test]
fn compiles_print_statement() {
    let r = parse_Program("print 12;");

    let mut p = Compiler {
        instructions: Vec::new(),
    };
    let output = p.generate_instructions(r.unwrap());
    assert_eq!(
        output,
        vec![
            Instruction::OpCode(OpCode::Push(12)),
            Instruction::OpCode(OpCode::Print),
            Instruction::OpCode(OpCode::Halt),
        ]
    );
}
