use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

use ast::{Expr, Statement};
use instruction::{Instruction, OpCode};
use value::Value;
use parser1::parse_Program;
use assembler::{Assembler, AssemblyProgram};

pub struct Scope {
    index: usize,
    table: HashMap<String, usize>,
}

impl Scope {
    pub fn new() -> Scope {
        Scope { index: 0, table: HashMap::new() }
    }

    pub fn define(&mut self, symbol: String) -> usize {
        let current_index = self.index;
        self.table.insert(symbol, current_index);

        self.index += 1;
        current_index
    }

    pub fn get(&self, symbol: String) -> usize {
        *self.table.get(&symbol).unwrap()
    }

    pub fn defines(&self, symbol: String) -> bool {
        self.table.contains_key(&symbol)
    }
}

fn new_scope() -> Rc<RefCell<Scope>> {
    Rc::new(RefCell::new(Scope::new()))
}

pub struct Environment {
    scopes: Vec<Rc<RefCell<Scope>>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            scopes: vec![new_scope()],
        }
    }

    pub fn push(&mut self) {
       self.scopes.push(new_scope())
    }

    pub fn pop(&mut self) {
        self.scopes.pop();
    }

    pub fn define(&mut self, name: String) -> (usize, usize) {
        let scope = self.scopes.last().unwrap();
        let idx = self.scopes.len() - 1;

        (idx, scope.borrow_mut().define(name))
    }

    fn find_scope_for_var(&mut self, name: &str) -> Option<(usize, usize)> {
        for (idx, scope) in self.scopes.iter().enumerate().rev() {
            if scope.borrow_mut().defines(name.to_string()) {
                return Some((idx, scope.borrow_mut().get(name.to_string())));
            }
        }
        None
    }

    pub fn get(&mut self, name: String) -> (usize, usize) {
        match self.find_scope_for_var(&name) {
            Some((idx, i)) => (idx, i),
            None => panic!("No scope found for variable {}", name)
        }
    }
}

pub fn compile(program: &str) -> AssemblyProgram {
    let mut constants = HashMap::new();
    let mut constant_pool = Vec::new();
    let p = parse_Program(&mut constants, &mut constant_pool, program);
    println!("{:?}", constants);
    let mut compiler = Compiler::new();
    let instructions = compiler.generate_instructions(p.unwrap());
    let mut assembler = Assembler::new(instructions);
    assembler.resolve_labels();
    AssemblyProgram {
            op_codes: assembler.generate_op_codes(),
            constant_pool: constant_pool,
    }
}

pub struct Compiler {
    instructions: Vec<Instruction>,
    environment: Environment,
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            instructions: Vec::new(),
            environment: Environment::new(),
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
                self.process_expr(*expression);
                self.instructions.push(Instruction::OpCode(OpCode::Print));
            }

            Statement::Block { statements } => {
                self.instructions.push(Instruction::OpCode(OpCode::PushFrame));
                self.environment.push();
                self.process_statements(statements);
                self.instructions.push(Instruction::OpCode(OpCode::PopFrame));
                self.environment.pop();
            }

            Statement::Var { name, initializer } => {
                let (frame_idx, idx) = self.environment.define(name.clone());
                self.instructions.push(Instruction::Local(name, idx));
                if let Some(i) = initializer {
                    // if there's an initializer first we put its value on the stack...
                    self.process_expr(*i);
                    // and then store it
                    self.instructions.push(Instruction::OpCode(OpCode::Store(frame_idx, idx)));
                }
            }
        }
    }

    fn process_expr(&mut self, n: Expr) {
        match n {
            Expr::Literal { value: i } => {
                self.instructions.push(Instruction::OpCode(OpCode::Constant(i)))
            }
            Expr::Variable { name } => {
                let (frame_idx, i) = self.environment.get(name);
                self.instructions.push(Instruction::OpCode(OpCode::Load(frame_idx, i)))
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
            Expr::Assign {
                name,
                value
            } => {
                let (frame_idx, idx) = self.environment.get(name);
                self.process_expr(*value);
                // TODO: See if these semantics are correct. It works for
                // something like "print a = 3;" but might be silly elsewhere
                self.instructions.push(Instruction::OpCode(OpCode::Dup));
                self.instructions.push(Instruction::OpCode(OpCode::Store(frame_idx, idx)));
            }
        }
    }

    pub fn generate_instructions(&mut self, statements: Vec<Statement>) -> Vec<Instruction> {
        self.process_statements(statements);
        self.instructions.push(Instruction::OpCode(OpCode::Halt));
        self.instructions.clone()
    }
}

#[cfg(test)]
mod tests {
    use parser1::parse_Program;
    use super::*;
    use instruction::{Instruction, OpCode};
    #[test]
    fn compiles_simple_math() {
        let r = parse_Program(&mut HashMap::new(), &mut Vec::new(), "(1 + 3) * 12;");
        println!("{:#?}", r);

        let mut p = Compiler::new();
        let output = p.generate_instructions(r.unwrap());
        println!("{:#?}", output);
        assert_eq!(
            output,
            vec![
                Instruction::OpCode(OpCode::Constant(0)),
                Instruction::OpCode(OpCode::Constant(1)),
                Instruction::OpCode(OpCode::Add),
                Instruction::OpCode(OpCode::Constant(2)),
                Instruction::OpCode(OpCode::Multiply),
                Instruction::OpCode(OpCode::Halt),
            ]
        );
    }

    #[test]
    fn compiles_string_literal() {
        let r = parse_Program(&mut HashMap::new(), &mut Vec::new(), "print \"FOO\";");

        let mut p = Compiler::new();
        let output = p.generate_instructions(r.unwrap());
        assert_eq!(
            output,
            vec![
                Instruction::OpCode(OpCode::Constant(0)),
                Instruction::OpCode(OpCode::Print),
                Instruction::OpCode(OpCode::Halt),
            ]
        );
    }

    #[test]
    fn compiles_boolean_literal() {
        let r = parse_Program(&mut HashMap::new(), &mut Vec::new(), "print true;");

        let mut p = Compiler::new();
        let output = p.generate_instructions(r.unwrap());
        assert_eq!(
            output,
            vec![
                Instruction::OpCode(OpCode::Constant(0)),
                Instruction::OpCode(OpCode::Print),
                Instruction::OpCode(OpCode::Halt),
            ]
        );
    }

    #[test]
    fn compiles_print_statement() {
        let r = parse_Program(&mut HashMap::new(), &mut Vec::new(), "print 12;");

        let mut p = Compiler::new();
        let output = p.generate_instructions(r.unwrap());
        assert_eq!(
            output,
            vec![
                Instruction::OpCode(OpCode::Constant(0)),
                Instruction::OpCode(OpCode::Print),
                Instruction::OpCode(OpCode::Halt),
            ]
        );
    }

    #[test]
    fn compiles_var_declaration() {
        let r = parse_Program(&mut HashMap::new(), &mut Vec::new(), "var a;");

        let mut p = Compiler::new();
        let output = p.generate_instructions(r.unwrap());
        assert_eq!(
            output,
            vec![
                Instruction::Local("a".to_string(), 0),
                Instruction::OpCode(OpCode::Halt),
            ]
        );
    }

    #[test]
    fn compiles_var_def() {
        let r = parse_Program(&mut HashMap::new(), &mut Vec::new(), "var a = 3;");

        let mut p = Compiler::new();
        let output = p.generate_instructions(r.unwrap());
        assert_eq!(
            output,
            vec![
                Instruction::Local("a".to_string(), 0),
                Instruction::OpCode(OpCode::Constant(0)),
                Instruction::OpCode(OpCode::Store(0, 0)),
                Instruction::OpCode(OpCode::Halt),
            ]
        );
    }

    #[test]
    fn compiles_var_use() {
        let r = parse_Program(&mut HashMap::new(), &mut Vec::new(), "var a = 3; print a;");

        let mut p = Compiler::new();
        let output = p.generate_instructions(r.unwrap());
        assert_eq!(
            output,
            vec![
                Instruction::Local("a".to_string(), 0),
                Instruction::OpCode(OpCode::Constant(0)),
                Instruction::OpCode(OpCode::Store(0, 0)),
                Instruction::OpCode(OpCode::Load(0, 0)),
                Instruction::OpCode(OpCode::Print),
                Instruction::OpCode(OpCode::Halt),
            ]
        );
    }

    #[test]
    fn compiles_var_assignment() {
        let r = parse_Program(&mut HashMap::new(), &mut Vec::new(), "var a = 3; a = 4;");

        let mut p = Compiler::new();
        let output = p.generate_instructions(r.unwrap());
        assert_eq!(
            output,
            vec![
                Instruction::Local("a".to_string(), 0),
                Instruction::OpCode(OpCode::Constant(0)),
                Instruction::OpCode(OpCode::Store(0, 0)),
                Instruction::OpCode(OpCode::Constant(1)),
                Instruction::OpCode(OpCode::Dup), //assignment returns the value
                Instruction::OpCode(OpCode::Store(0, 0)),
                Instruction::OpCode(OpCode::Halt),
            ]
        );
    }

    #[test]
    fn compiles_empty_block() {
        let r = parse_Program(&mut HashMap::new(), &mut Vec::new(), "{}");

        let mut p = Compiler::new();
        let output = p.generate_instructions(r.unwrap());
        assert_eq!(
            output,
            vec![
                Instruction::OpCode(OpCode::PushFrame),
                Instruction::OpCode(OpCode::PopFrame),
                Instruction::OpCode(OpCode::Halt),
            ]
        );
    }

    #[test]
    fn compiles_block_without_closure() {
        let r = parse_Program(&mut HashMap::new(), &mut Vec::new(),
        "var a = 1; { var b = 2; }");

        let mut p = Compiler::new();
        let output = p.generate_instructions(r.unwrap());
        assert_eq!(
            output,
            vec![
                Instruction::Local("a".to_string(), 0),
                Instruction::OpCode(OpCode::Constant(0)),
                Instruction::OpCode(OpCode::Store(0, 0)),
                Instruction::OpCode(OpCode::PushFrame),
                Instruction::Local("b".to_string(), 0),
                Instruction::OpCode(OpCode::Constant(1)),
                Instruction::OpCode(OpCode::Store(1, 0)),
                Instruction::OpCode(OpCode::PopFrame),
                Instruction::OpCode(OpCode::Halt),
            ]
        );
    }

    #[test]
    fn compiles_block_with_closure() {
        let r = parse_Program(&mut HashMap::new(), &mut Vec::new(),
                              "var a = 1; { a = 2; }");

        let mut p = Compiler::new();
        let output = p.generate_instructions(r.unwrap());
        assert_eq!(
            output,
            vec![
                Instruction::Local("a".to_string(), 0),
                Instruction::OpCode(OpCode::Constant(0)),
                Instruction::OpCode(OpCode::Store(0, 0)),
                Instruction::OpCode(OpCode::PushFrame),
                Instruction::OpCode(OpCode::Constant(1)),
                Instruction::OpCode(OpCode::Dup),
                Instruction::OpCode(OpCode::Store(0, 0)),
                Instruction::OpCode(OpCode::PopFrame),
                Instruction::OpCode(OpCode::Halt),
            ]
        );
    }

}
