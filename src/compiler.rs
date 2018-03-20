use std::collections::HashMap;
use std::cell::RefCell;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::rc::Rc;

use ast::{print_ast, Expr, Statement};
use instruction::{Instruction, OpCode, Ref};
use parser1::parse_Program;
use assembler::{Assembler, AssemblyProgram};
use value::Value;
use function::FunctionDefinition;

#[derive(Debug)]
pub struct Scope {
    index: usize,
    table: HashMap<String, usize>,
}

impl Scope {
    pub fn new() -> Scope {
        Scope {
            index: 0,
            table: HashMap::new(),
        }
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

#[derive(Debug)]
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
            None => panic!("No scope found for variable {}", name),
        }
    }
}

fn write_to_file(contents: String, filename: &str) {
    let path = Path::new(filename);
    let mut file = File::create(&path).unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}

pub fn compile(program: &str) -> AssemblyProgram {
    let mut constants = HashMap::new();
    let mut constant_pool: Vec<Value> = Vec::new();
    let ast = parse_Program(
        &mut constants,
        &mut constant_pool,
        program
    ).unwrap();
    match print_ast(&ast) {
        Ok(res) => write_to_file(res, "ast.dot"),
        Err(_) => println!("There was a problem generating the ast graph"),
    };
    let mut compiler = Compiler::new();
    let instructions = compiler.generate_instructions(ast);
    let mut assembler = Assembler::new(instructions.clone());
    assembler.resolve_labels();
    AssemblyProgram {
        instructions: instructions,
        op_codes: assembler.generate_op_codes(),
        constant_pool: constant_pool,
    }
}

pub struct Compiler {
    instructions: Vec<Instruction>,
    all_function_instructions: Vec<Vec<Instruction>>,
    environment: Environment,
    label_id: i32,
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            instructions: Vec::new(),
            all_function_instructions: vec![Vec::new()],
            environment: Environment::new(),
            label_id: 0,
        }
    }

    fn get_label_id(&mut self) -> i32 {
        self.label_id += 1;
        self.label_id
    }
    fn generate_label(&mut self, prefix: &str, l: i32) -> String {
        format!("{}_{}", prefix, l)
    }

    fn process_statements(&mut self, statements: Vec<Statement>) {
        for s in statements {
            self.process_statement(s);
        }
    }

    fn process_statement(&mut self, s: Statement) {
        match s {
            Statement::Program { statements } => self.process_statements(statements),
            Statement::Expression { expression } => self.process_expr(*expression),
            Statement::Debugger => self.instructions.push(Instruction::OpCode(OpCode::Debugger)),
            Statement::Print { expression } => {
                self.process_expr(*expression);
                self.instructions.push(Instruction::OpCode(OpCode::Print));
            }

            Statement::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.process_expr(*condition);
                let l = self.get_label_id();
                let else_label = self.generate_label("else", l);
                let then_label = self.generate_label("then", l);
                let end_label = self.generate_label("if_end", l);
                self.instructions
                    .push(Instruction::Ref(Ref::JmpIf(then_label.clone())));
                self.instructions
                    .push(Instruction::Ref(Ref::Jmp(else_label.clone())));
                self.instructions
                    .push(Instruction::Label(then_label.clone()));
                self.process_statement(*then_branch);
                self.instructions
                    .push(Instruction::Ref(Ref::Jmp(end_label.clone())));
                self.instructions
                    .push(Instruction::Label(else_label.clone()));
                if let Some(b) = else_branch {
                    self.process_statement(*b);
                }
                self.instructions
                    .push(Instruction::Label(end_label.clone()));
            }

            Statement::While { condition, body } => {
                let l = self.get_label_id();
                let cond_label = self.generate_label("while_condition", l);
                let body_label = self.generate_label("while_body", l);
                let end_label = self.generate_label("while_end", l);

                self.instructions
                    .push(Instruction::Label(cond_label.clone()));
                self.process_expr(*condition);

                self.instructions
                    .push(Instruction::Ref(Ref::JmpIf(body_label.clone())));
                self.instructions
                    .push(Instruction::Ref(Ref::Jmp(end_label.clone())));

                self.instructions
                    .push(Instruction::Label(body_label.clone()));
                self.process_statement(*body);
                self.instructions
                    .push(Instruction::Ref(Ref::Jmp(cond_label.clone())));
                self.instructions
                    .push(Instruction::Label(end_label.clone()));
            }

            Statement::Block { statements } => {
                self.instructions
                    .push(Instruction::OpCode(OpCode::PushFrame));
                self.environment.push();
                self.process_statements(statements);
                self.instructions
                    .push(Instruction::OpCode(OpCode::PopFrame));
                self.environment.pop();
            }

            Statement::Function { // function definition
                name,
                parameters,
                body,
            } => {
                let l = self.get_label_id();
                let fun_label = self.generate_label(&format!("fun_{}", name), l);

                let function_def = FunctionDefinition {
                    arity: parameters.len(),
                    parameters: parameters.clone(),
                    name: name.clone(),
                    label: fun_label.clone(),
                    instruction_address: None, // to be updated in an assembler pass
                };

                self.instructions.push(Instruction::OpCode(OpCode::DefineFunction(function_def)));
                //defineFunction will put the function prototype on the stack.

                let (frame_idx, idx) = self.environment.define(name.clone());
                self.instructions.push(Instruction::Local(name.clone(), idx));

                self.instructions
                    .push(Instruction::OpCode(OpCode::Store(frame_idx, idx)));

                let mut function_instructions = Vec::new();
                function_instructions.push(Instruction::Label(fun_label));

                // TODO: make this not horrible
                // 1. cache the current instruction pointer [a, a, a, a] (self.instructions.len() - 1)
                let first_function_instruction_idx = self.instructions.len();
                // 2. create instructions for the function block
                // first define all the local variables for the parameters
                for p in parameters.into_iter().rev() {
                    let (frame_idx, idx) = self.environment.define(p.clone());
                    self.instructions.push(Instruction::Local(p, idx));
                    self.instructions
                        .push(Instruction::OpCode(OpCode::Store(frame_idx, idx)));
                }

                self.process_statement(*body);

                // 3. take all those instructions off self.instructions and transfer them to self.function_instructions
                let mut new_function_instrs: Vec<Instruction> = self.instructions.drain(first_function_instruction_idx..).collect();
                function_instructions.append(&mut new_function_instrs);
                function_instructions.push(Instruction::OpCode(OpCode::Ret));
                self.all_function_instructions.push(function_instructions);
            }

            Statement::Return { value } => {
                match value {
                    Some(v) => self.process_expr(*v),
                    None => ()
                }
                self.instructions.push(Instruction::OpCode(OpCode::Ret));
            }

            Statement::Var { name, initializer } => {
                let (frame_idx, idx) = self.environment.define(name.clone());
                self.instructions.push(Instruction::Local(name, idx));
                if let Some(i) = initializer {
                    // if there's an initializer first we put its value on the stack...
                    self.process_expr(*i);
                    // and then store it
                    self.instructions
                        .push(Instruction::OpCode(OpCode::Store(frame_idx, idx)));
                }
            }
        }
    }

    fn process_expr(&mut self, n: Expr) {
        match n {
            Expr::Literal { value: i } => self.instructions
                .push(Instruction::OpCode(OpCode::Constant(i))),
            Expr::Variable { name } => {
                let (frame_idx, i) = self.environment.get(name);
                self.instructions
                    .push(Instruction::OpCode(OpCode::Load(frame_idx, i)))
            }

            Expr::Binary {
                left,
                operator,
                right,
            } => {
                self.process_expr(*left);
                self.process_expr(*right);
                use ast::BinaryOperator::*;

                let op = match operator {
                    Plus => OpCode::Add,
                    Minus => OpCode::Subtract,
                    Star => OpCode::Multiply,
                    Eq => OpCode::IsEq,
                    BangEq => OpCode::NEq,
                    Lt => OpCode::IsLt,
                    Gt => OpCode::IsGt,
                    LtEq => OpCode::IsLe,
                    GtEq => OpCode::IsGe,
                };
                self.instructions.push(Instruction::OpCode(op))
            }

            Expr::Call { callee, arguments } => {
                // push args for fn onto stack
                for a in arguments.into_iter() {
                    self.process_expr(*a);
                }

                // evaluate the callee to put a FunctionDefinition on the stack
                self.process_expr(*callee);
                self.instructions.push(Instruction::OpCode(OpCode::CallFn))
            }

            Expr::Assign { name, value } => {
                let (frame_idx, idx) = self.environment.get(name);
                self.process_expr(*value);
                // TODO: See if these semantics are correct. It works for
                // something like "print a = 3;" but might be silly elsewhere
                self.instructions.push(Instruction::OpCode(OpCode::Dup));
                self.instructions
                    .push(Instruction::OpCode(OpCode::Store(frame_idx, idx)));
            }

            Expr::Logical {
                left,
                right,
                operator,
            } => {
                use ast::LogicalOperator::*;
                match operator {
                    And => {
                        self.process_expr(*left);
                        let l = self.get_label_id();
                        let true_label = self.generate_label("and", l);
                        let false_label = self.generate_label("!and", l);
                        self.instructions.push(Instruction::OpCode(OpCode::Dup));
                        self.instructions
                            .push(Instruction::Ref(Ref::JmpIf(true_label.clone())));
                        self.instructions
                            .push(Instruction::Ref(Ref::Jmp(false_label.clone())));

                        self.instructions
                            .push(Instruction::Label(true_label.clone()));
                        self.process_expr(*right);

                        self.instructions
                            .push(Instruction::Label(false_label.clone()));
                    }

                    Or => {
                        self.process_expr(*left);
                        let l = self.get_label_id();
                        let true_label = self.generate_label("or", l);
                        let false_label = self.generate_label("!or", l);
                        self.instructions.push(Instruction::OpCode(OpCode::Dup));
                        self.instructions
                            .push(Instruction::Ref(Ref::JmpIf(true_label.clone())));
                        self.instructions
                            .push(Instruction::Ref(Ref::Jmp(false_label.clone())));

                        self.instructions
                            .push(Instruction::Label(false_label.clone()));
                        self.process_expr(*right);

                        self.instructions
                            .push(Instruction::Label(true_label.clone()));
                    }
                }
            }
        }
    }

    pub fn generate_instructions(&mut self, program: Statement) -> Vec<Instruction> {
        self.process_statement(program);
        self.instructions.push(Instruction::OpCode(OpCode::Halt));
        for mut function_block in self.all_function_instructions.iter_mut() {
            self.instructions.append(&mut function_block);
        };
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
        let r = parse_Program(
            &mut HashMap::new(),
            &mut Vec::new(),
            "var a = 1; { var b = 2; }",
        );

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
        let r = parse_Program(
            &mut HashMap::new(),
            &mut Vec::new(),
            "var a = 1; { a = 2; }",
        );

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

    #[test]
    fn compiles_logical_and() {
        let r = parse_Program(&mut HashMap::new(), &mut Vec::new(), "false && true;");

        let mut p = Compiler::new();
        let output = p.generate_instructions(r.unwrap());
        assert_eq!(
            output,
            vec![
                Instruction::OpCode(OpCode::Constant(0)),
                Instruction::OpCode(OpCode::Dup),
                Instruction::Ref(Ref::JmpIf("and_1".to_string())),
                Instruction::Ref(Ref::Jmp("!and_1".to_string())),
                Instruction::Label("and_1".to_string()),
                Instruction::OpCode(OpCode::Constant(1)),
                Instruction::Label("!and_1".to_string()),
                Instruction::OpCode(OpCode::Halt),
            ]
        );
    }

    #[test]
    fn compiles_logical_and_print() {
        let r = parse_Program(&mut HashMap::new(), &mut Vec::new(), "print false && true;");

        let mut p = Compiler::new();
        let output = p.generate_instructions(r.unwrap());
        assert_eq!(
            output,
            vec![
                Instruction::OpCode(OpCode::Constant(0)),
                Instruction::OpCode(OpCode::Dup),
                Instruction::Ref(Ref::JmpIf("and_1".to_string())),
                Instruction::Ref(Ref::Jmp("!and_1".to_string())),
                Instruction::Label("and_1".to_string()),
                Instruction::OpCode(OpCode::Constant(1)),
                Instruction::Label("!and_1".to_string()),
                Instruction::OpCode(OpCode::Print),
                Instruction::OpCode(OpCode::Halt),
            ]
        );
    }

    #[test]
    fn compiles_logical_or() {
        let r = parse_Program(&mut HashMap::new(), &mut Vec::new(), "false || true;");

        let mut p = Compiler::new();
        let output = p.generate_instructions(r.unwrap());
        assert_eq!(
            output,
            vec![
                Instruction::OpCode(OpCode::Constant(0)),
                Instruction::OpCode(OpCode::Dup),
                Instruction::Ref(Ref::JmpIf("or_1".to_string())),
                Instruction::Ref(Ref::Jmp("!or_1".to_string())),
                Instruction::Label("!or_1".to_string()),
                Instruction::OpCode(OpCode::Constant(1)),
                Instruction::Label("or_1".to_string()),
                Instruction::OpCode(OpCode::Halt),
            ]
        );
    }

    #[test]
    fn parses_function_call() {
        use ast::{Expr, Statement};
        let r = parse_Program(&mut HashMap::new(), &mut Vec::new(), "foo(1);");

        let out = Statement::Program {
            statements: vec![
                Statement::Expression {
                    expression: Box::new(Expr::Call {
                        callee: Box::new(Expr::Variable {
                            name: "foo".to_string(),
                        }),
                        arguments: vec![Box::new(Expr::Literal { value: 0 })],
                    }),
                },
            ],
        };
        assert_eq!(r.unwrap(), out);
    }

    #[test]
    fn parses_chained_function_calls() {
        use ast::{Expr, Statement};
        let r = parse_Program(&mut HashMap::new(), &mut Vec::new(), "foo(1)(2);");

        let out = Statement::Program {
            statements: vec![
                Statement::Expression {
                    expression: Box::new(Expr::Call {
                        arguments: vec![Box::new(Expr::Literal { value: 1 })],
                        callee: Box::new(
                            Expr::Call {
                                callee: Box::new(Expr::Variable { name: "foo".to_string() }),
                                arguments: vec![Box::new(Expr::Literal { value: 0 })],
                            })
                    }),
                },
            ],
        };

        assert_eq!(r.unwrap(), out);
    }

    #[test]
    fn compiles_function_call() {
        let r = parse_Program(&mut HashMap::new(),
                              &mut Vec::new(),
                              "var foo; foo(1, 2);");

        let mut p = Compiler::new();
        let output = p.generate_instructions(r.unwrap());
        assert_eq!(
            output,
            vec![
                Instruction::Local("foo".to_string(), 0),
                Instruction::OpCode(OpCode::Constant(0)),
                Instruction::OpCode(OpCode::Constant(1)),
                Instruction::OpCode(OpCode::Load(0, 0)),
                Instruction::OpCode(OpCode::CallFn),
                Instruction::OpCode(OpCode::Halt),
            ]
        );
    }

    #[test]
    fn compiles_function_def() {
        let r = parse_Program(&mut HashMap::new(),
                              &mut Vec::new(),
                              "fun foo() {};");

        let mut p = Compiler::new();
        let output = p.generate_instructions(r.unwrap());
        let f_label = "fun_foo_1".to_string();
        let expected_fn_def = FunctionDefinition {
            arity: 0,
            parameters: vec![],
            name: "foo".to_string(),
            label: f_label.clone(),
            instruction_address: None,
        };

        assert_eq!(
            format!("{:?}", output),
            format!("{:?}",vec![
                Instruction::Local("foo".to_string(), 0),
                Instruction::OpCode(OpCode::DefineFunction(expected_fn_def)),
                Instruction::OpCode(OpCode::Store(0, 0)),
                Instruction::OpCode(OpCode::Halt),
                Instruction::Label(f_label.clone()),
                Instruction::OpCode(OpCode::PushFrame),
                Instruction::OpCode(OpCode::PopFrame),
                Instruction::OpCode(OpCode::Ret),
            ])
        );
    }

    #[test]
    fn compiles_function_def_with_params() {
        let r = parse_Program(&mut HashMap::new(),
                              &mut Vec::new(),
                              "fun foo(a, b) {};");

        let mut p = Compiler::new();
        let output = p.generate_instructions(r.unwrap());
        let f_label = "fun_foo_1".to_string();
        let expected_fn_def = FunctionDefinition {
            arity: 2,
            parameters: vec!["a".to_string(), "b".to_string()],
            name: "foo".to_string(),
            label: f_label.clone(),
            instruction_address: None,
        };

        assert_eq!(
            format!("{:?}", output),
            format!("{:?}",vec![
                Instruction::Local("foo".to_string(), 0),
                Instruction::OpCode(OpCode::DefineFunction(expected_fn_def)),
                Instruction::OpCode(OpCode::Store(0, 0)),
                Instruction::OpCode(OpCode::Halt),
                Instruction::Label(f_label.clone()),
                Instruction::Local("a".to_string(), 1),
                Instruction::OpCode(OpCode::Store(0, 1)),
                Instruction::Local("b".to_string(), 2),
                Instruction::OpCode(OpCode::Store(0, 2)),
                Instruction::OpCode(OpCode::PushFrame),
                Instruction::OpCode(OpCode::PopFrame),
                Instruction::OpCode(OpCode::Ret),
            ])
        );
    }
}
