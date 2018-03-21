extern crate rprompt;

use instruction::OpCode;
use assembler::AssemblyProgram;
use std::rc::Rc;
use std::cell::RefCell;
use value::Value;
use function::FunctionPrototype;

#[derive(Debug)]
pub struct Frame {
    variables: Vec<Value>,
}

impl Frame {
    pub fn get_variable(&self, key: usize) -> Value {
        self.variables.get(key).unwrap_or(&Value::Number(0)).clone()
    }

    pub fn set_variable(&mut self, pos: usize, val: Value) {
        if self.variables.len() > pos {
            self.variables[pos] = val;
        } else {
            self.variables.insert(pos, val)
        }
    }

    pub fn new() -> Frame {
        Frame {
            variables: Vec::with_capacity(8),
        }
    }
}

// A closure acts as the execution environment for a function.
// Closures have to be created at runtime by the VM rather than
// compile time.
//
// Frame indices for Closures should be paralleled by the compiler's
// Scope indices.
#[derive(Debug, Clone)]
pub struct Closure {
    pub frames: Vec<Rc<RefCell<Frame>>>,
    pub return_address: usize,
}

impl Closure {
    pub fn from(source: &Closure) -> Closure {
        Closure {
            frames: source.frames.iter().map(|e| Rc::clone(e)).collect(),
            return_address: 0,
        }
    }

    pub fn new() -> Closure {
        Closure {
            frames: vec![Rc::new(RefCell::new(Frame::new()))],
            return_address: 0,
        }
    }

    pub fn get_frame(&self, idx: usize) -> Rc<RefCell<Frame>> {
         Rc::clone(self.frames.get(idx).unwrap())
    }

    pub fn push(&mut self, frame: Frame) {
        self.frames.push(Rc::new(RefCell::new(frame)));
    }

    pub fn pop(&mut self) {
        self.frames.pop();
    }

    pub fn len(&self) -> usize {
        self.frames.len()
    }
}

pub struct CPU {
    stack: Vec<Value>,
    print_buffer: Vec<String>,
    program: AssemblyProgram,
    current_frame_idx: usize,
    call_stack: Vec<(Closure, Option<Rc<FunctionPrototype>>)>,
    // the function environment will either be the
    // CPU's environment or one that belongs to the function
    instruction_address: usize,
    halted: bool,
    pub debug: bool,
    debugger_mode: bool,
}

impl CPU {
    pub fn new(program: AssemblyProgram) -> CPU {
        CPU {
            call_stack: vec![(Closure::new(), None)],
            current_frame_idx: 0,
            debug: false,
            debugger_mode: false,
            halted: false,
            instruction_address: 0,
            print_buffer: Vec::new(),
            program: program,
            stack: Vec::new(),
        }
    }

    pub fn with_op_codes(v: Vec<OpCode>) -> CPU {
        let a = AssemblyProgram {
            instructions: Vec::with_capacity(8),
            op_codes: v,
                constant_pool: Vec::new(),
        };
        CPU::new(a)
    }

    pub fn print_buffer(&self) -> &Vec<String> {
        &self.print_buffer
    }

    pub fn run(&mut self) {
        while !self.halted {
            self.step();
        }
    }

    pub fn current_frame(&self) -> Rc<RefCell<Frame>> {
        self.get_frame(self.current_frame_idx)
    }

    pub fn get_frame(&self, idx: usize) -> Rc<RefCell<Frame>> {
        self.environment().get_frame(idx)
    }

    pub fn environment(&self) -> &Closure {
        match self.call_stack.last() {
            Some(&(ref env, _)) => {
                env
            }
            None => panic!("The call stack is empty")
        }
    }
    
    pub fn environment_mut(&mut self) -> &mut Closure {
        match self.call_stack.last_mut() {
            Some(&mut (ref mut env, _)) => {
                env
            }
            None => panic!("The call stack is empty")
        }
    }

    pub fn step(&mut self) {
        assert!(!self.halted, "A halted CPU cannot execute any further");
        self.decode_next_instruction();
    }

    fn bin_op<F>(&mut self, perform: F)
    where
        F: Fn(Value, Value) -> Value,
    {
        assert!(
            self.stack.len() >= 2,
            "stack needs at least 2 values to perform binary operation"
        );

        let top = self.stack.pop().unwrap();
        let bot = self.stack.pop().unwrap();
        self.stack.push(perform(top, bot));
    }

    fn assert_jump_address(&self, i: usize) {
        assert!(i < self.program.op_codes.len(), "Invalid jump address");
    }

    fn assert_return_address(&self) {
        assert!(self.call_stack.len() > 0, "Invalid Ret instruction: no current function call");
    }

    pub fn load_constant(&mut self, idx: usize) {
        let c = self.program.constant_pool.get(idx).unwrap().clone();
        self.stack.push(c);
    }

    pub fn load_local(&mut self, frame_idx: usize, var_pos: usize) {
            let v = self.get_frame(frame_idx).borrow().get_variable(var_pos);
            self.stack.push(v);
    }

    pub fn store_local(&mut self, frame_idx: usize, var_pos: usize) {
        assert!(
            self.stack.len() >= 1,
            "stack needs at least 1 value to store"
        );
        let k = self.stack.pop().unwrap();

        let target_frame = self.get_frame(frame_idx);
        target_frame.borrow_mut().set_variable(var_pos, k);
    }

    fn decode_next_instruction(&mut self) {
        assert!(self.instruction_address < self.program.op_codes.len());
        let next_ins = &self.program.op_codes[self.instruction_address];
        if self.debug || self.debugger_mode {
            println!("{:<3}: {:15} stack: {:?}, call stack frames: {}, environment frames: {}", self.instruction_address, next_ins, self.stack, self.call_stack.len(), self.environment().frames.len());
            println!("environment: {:?}", self.environment());
        }

        if self.debugger_mode {
            let reply = rprompt::prompt_reply_stdout("Debugger: ").unwrap();
            println!("Your reply is {}", reply);

            match reply.as_ref() {
                "e" => self.debugger_mode = false,
                _ => (),
            }
        }
        self.instruction_address += 1;

        match *next_ins {
            OpCode::Halt => self.halted = true,
            OpCode::Debugger => {
                println!("You've entered the debugger! Enter 'n' to continue or 'e' to exit");
                self.debugger_mode = true;
            },
            OpCode::NoOp => (), //no-op
            OpCode::Push(ref val) => self.stack.push(val.clone()),
            OpCode::Constant(i) => {
                self.load_constant(i);
            }
            OpCode::Add => {
                self.bin_op(|top, bot| bot + top);
            }
            OpCode::Subtract => {
                self.bin_op(|top, bot| bot - top);
            }

            OpCode::Multiply => {
                self.bin_op(|top, bot| top * bot);
            }

            OpCode::And => {
                self.bin_op(|top, bot| {
                    match (top, bot) {
                        (Value::Bool(t), Value::Bool(b)) => Value::Bool(t && b),
                        _ => panic!("need bools"),
                    }
                });
            }

            OpCode::Or => {
                self.bin_op(|top, bot| {
                    match (top, bot) {
                        (Value::Bool(t), Value::Bool(b)) => Value::Bool(t || b),
                        _ => panic!("need bools"),
                    }
                });
            }

            OpCode::IsEq => {
                self.bin_op(|top, bot| Value::Bool(top == bot));
            }

            OpCode::NEq => {
                self.bin_op(|top, bot| Value::Bool(top != bot));
            }

            OpCode::IsGt => {
                self.bin_op(|top, bot| Value::Bool(bot > top));
            }

            OpCode::IsGe => {
                self.bin_op(|top, bot| Value::Bool(bot >= top));
            }

            OpCode::IsLt => {
                self.bin_op(|top, bot| Value::Bool(bot < top));
            }

            OpCode::IsLe => {
                self.bin_op(|top, bot| Value::Bool(bot <= top));
            }

            OpCode::Not => {
                assert!(
                    self.stack.len() >= 1,
                    "stack needs at least 1 values to add"
                        );

                let top = self.stack.pop().unwrap() == Value::Bool(true);
                self.stack.push(Value::Bool(!top));
            }

            OpCode::Pop => {
                assert!(self.stack.len() >= 1, "stack needs at least 1 value to pop");
                self.stack.pop().unwrap();
            }

            OpCode::Dup => {
                assert!(
                    self.stack.len() >= 1,
                    "stack needs at least 1 value to duplicate"
                );
                let n = self.stack.last().unwrap();
                self.stack.push(n.clone());
            }

            OpCode::Jmp(addr) => {
                self.instruction_address = addr;
            }

            OpCode::JmpIf(addr) => {
                let k = self.stack.pop().unwrap();
                if k == Value::Bool(true) {
                    self.instruction_address = addr;
                }
            }

            OpCode::Store(frame_idx, var_pos) => {
                self.store_local(frame_idx, var_pos);
            }

            OpCode::Load(frame_idx, var_pos) => {
                self.load_local(frame_idx, var_pos);
            }

            OpCode::StoreLocal(var_pos) => {
                assert!(
                    self.stack.len() >= 1,
                    "stack needs at least 1 value to store"
                );
                let k = self.stack.pop().unwrap();
                self.current_frame().borrow_mut().set_variable(var_pos, k);
            }

            OpCode::LoadLocal(var_pos) => {
                let v = self.current_frame().borrow().get_variable(var_pos);
                self.stack.push(v);
            }

            OpCode::Print => {
                let k = self.stack.pop().unwrap();
                println!("{}", k);
                self.print_buffer.push(format!("{}", k))
            }

            OpCode::DebugPrint => {
                println!("{:?}", self.stack);
                println!("{:?}", self.current_frame());
            }

            // TODO: basically replace this with CallFn
            OpCode::Call(i) => {
                self.assert_jump_address(i);
                self.environment_mut().push(Frame::new());
                self.current_frame_idx += 1;
                self.instruction_address = i;
            }

            // creates a new closure, then pushes the new function prototype onto the stack
            OpCode::DefineFunction(ref function_def) => {
                let c = Closure::from(&self.environment());
                let function_proto = FunctionPrototype {
                    arity: function_def.arity,
                    name: function_def.name.clone(),
                    label: function_def.label.clone(),
                    instruction_address: function_def.instruction_address.unwrap(),
                    closure: c,
                };
                self.stack.push(Value::Fn(Rc::new(function_proto)));
            }

            OpCode::CallFn => {
                let function_proto = self.stack.pop().unwrap();
                match function_proto {
                    Value::Fn(proto) => {
                        // at the point a function is called its arguments
                        // will be on the stack in reverse order.
                        // foo(a, b) will be called with
                        // bottom --> [a, b] --> top
                        assert!(self.stack.len() >= proto.arity,
                                "For function to be called all its arguments must be on the stack");
                        let i = proto.instruction_address;
                        // WW this is the same OpCode::Call
                        self.assert_jump_address(i);

                        let mut env = Closure::from(&proto.closure);
                        env.return_address = self.instruction_address;
                        env.push(Frame::new());

                        self.call_stack.push((env, Some(Rc::clone(&proto))));

                        // set the environment to be the environment from the function's
                        // closure
                        self.current_frame_idx += 1;
                        self.instruction_address = i;
                        // MM this is the same OpCode::Call
                    }
                    _ => panic!("In order to call a function its definition must be at the top of the stack.")
                }
            }

            OpCode::PushFrame => {
                self.environment_mut().push(Frame::new());
                self.current_frame_idx += 1;
            }

            OpCode::PopFrame => {
                self.environment_mut().pop();
                self.current_frame_idx -= 1;
            }

            OpCode::Ret => {
                self.assert_return_address();
                // this return address will come from the closure at the top
                // of the call stack
                let return_address = self.environment().return_address;
                self.environment_mut().pop();
                self.call_stack.pop();
                self.current_frame_idx -= 1;
                self.instruction_address = return_address;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use function::FunctionDefinition;

    fn assert_current_frame_value(cpu: &CPU, index: usize, value: Value) {
        assert_eq!(cpu.current_frame().borrow().get_variable(index), value);
    }

    fn assert_halted_at(cpu: &CPU, l: usize) {
        assert_eq!(l, cpu.instruction_address);
        assert!(cpu.halted);
    }

    fn test_stack_for_instructions(prog: Vec<OpCode>, s: Vec<Value>) {
        let l = prog.len();
        let mut cpu = CPU::with_op_codes(prog);
        cpu.run();
        assert_eq!(l, cpu.instruction_address);
        assert!(cpu.halted);
        assert_eq!(cpu.stack, s);
    }

    fn test_final_address_for_instructions(prog: Vec<OpCode>, i: usize) {
        let mut cpu = CPU::with_op_codes(prog);
        cpu.run();
        assert!(cpu.halted);
        assert_eq!(i, cpu.instruction_address);
    }

    #[test]
    fn empty_program() {
        let mut cpu = CPU::with_op_codes(vec![OpCode::Halt]);
        cpu.step();
        assert_halted_at(&cpu, 1);
        assert_eq!(cpu.stack.len(), 0);
    }

    #[test]
    fn push_two_numbers() {
        test_stack_for_instructions(
            vec![
                OpCode::Push(Value::Number(42)),
                OpCode::Push(Value::Number(68)),
                OpCode::Halt,
            ],
            vec![Value::Number(42), Value::Number(68)],
        );
    }

    #[test]
    fn print_buffer() {
        let mut cpu = CPU::with_op_codes(vec![
            OpCode::Push(Value::Number(42)),
            OpCode::Print,
            OpCode::Halt
        ]);
        cpu.run();
        assert_halted_at(&cpu, 3);
        assert_eq!(cpu.print_buffer, vec!["42"]);
        assert_eq!(cpu.stack.len(), 0);
    }

    #[test]
    fn add_two_numbers() {
        test_stack_for_instructions(
            vec![
                OpCode::Push(Value::Number(42)),
                OpCode::Push(Value::Number(68)),
                OpCode::Add,
                OpCode::Halt,
            ],
            vec![Value::Number(110)],
        );
    }

    #[test]
    fn subtract_two_numbers() {
        test_stack_for_instructions(
            vec![
                OpCode::Push(Value::Number(1)),
                OpCode::Push(Value::Number(2)),
                OpCode::Subtract,
                OpCode::Halt,
            ],
            vec![Value::Number(-1)],
        );
    }

    #[test]
    fn multiply_two_numbers() {
        test_stack_for_instructions(
            vec![
                OpCode::Push(Value::Number(3)),
                OpCode::Push(Value::Number(5)),
                OpCode::Multiply,
                OpCode::Halt,
            ],
            vec![Value::Number(15)],
        );
    }

    #[test]
    fn pop() {
        test_stack_for_instructions(
            vec![
                OpCode::Push(Value::Number(3)),
                OpCode::Push(Value::Number(5)),
                OpCode::Pop,
                OpCode::Halt,
            ],
            vec![Value::Number(3)],
        )
    }

    #[test]
    fn dup() {
        test_stack_for_instructions(
            vec![OpCode::Push(Value::Number(3)), OpCode::Dup, OpCode::Halt],
            vec![Value::Number(3), Value::Number(3)],
        );
    }

    #[test]
    fn is_eq() {
        test_stack_for_instructions(
            vec![
                OpCode::Push(Value::Number(3)),
                OpCode::Push(Value::Number(3)),
                OpCode::IsEq,
                OpCode::Halt,
            ],
            vec![Value::Bool(true)],
        );
        test_stack_for_instructions(
            vec![
                OpCode::Push(Value::Number(5)),
                OpCode::Push(Value::Number(3)),
                OpCode::IsEq,
                OpCode::Halt,
            ],
            vec![Value::Bool(false)],
        )
    }

    #[test]
    fn is_lt() {
        test_stack_for_instructions(
            vec![
                OpCode::Push(Value::Number(4)),
                OpCode::Push(Value::Number(3)),
                OpCode::IsLt,
                OpCode::Halt,
            ],
            vec![Value::Bool(false)],
        );
        test_stack_for_instructions(
            vec![
                OpCode::Push(Value::Number(3)),
                OpCode::Push(Value::Number(5)),
                OpCode::IsLt,
                OpCode::Halt,
            ],
            vec![Value::Bool(true)],
        )
    }

    #[test]
    fn is_gt() {
        test_stack_for_instructions(
            vec![
                OpCode::Push(Value::Number(4)),
                OpCode::Push(Value::Number(3)),
                OpCode::IsGt,
                OpCode::Halt,
            ],
            vec![Value::Bool(true)],
        );
        test_stack_for_instructions(
            vec![
                OpCode::Push(Value::Number(3)),
                OpCode::Push(Value::Number(5)),
                OpCode::IsGt,
                OpCode::Halt,
            ],
            vec![Value::Bool(false)],
        )
    }

    #[test]
    fn test_jump() {
        test_final_address_for_instructions(
            vec![OpCode::Jmp(2), OpCode::Halt, OpCode::Jmp(1)],
            2,
        )
    }

    #[test]
    fn test_jump_if() {
        test_final_address_for_instructions(
            vec![
                OpCode::Push(Value::Bool(true)),
                OpCode::JmpIf(3),
                OpCode::Pop,
                OpCode::Push(Value::Number(0)),
                OpCode::JmpIf(2),
                OpCode::Halt,
            ],
            6,
        )
    }

    #[test]
    fn store_var() {
        let mut cpu = CPU::with_op_codes(vec![
            OpCode::Push(Value::Number(42)),
            OpCode::Store(0, 0),
            OpCode::Halt,
        ]);
        cpu.run();
        assert_halted_at(&cpu, 3);
        assert_current_frame_value(&cpu, 0, Value::Number(42));
    }

    #[test]
    fn load_uninitialized_var() {
        let mut cpu = CPU::with_op_codes(vec![OpCode::Load(0, 0), OpCode::Halt]);
        cpu.run();
        assert_eq!(cpu.stack, vec![Value::Number(0)]);
        assert_halted_at(&cpu, 2);
    }

    #[test]
    fn load_stored_var() {
        let mut cpu = CPU::with_op_codes(vec![
            OpCode::Push(Value::Number(42)),
            OpCode::Store(0, 0),
            OpCode::Load(0, 0),
            OpCode::Halt,
        ]);
        cpu.run();
        assert_eq!(cpu.stack, vec![Value::Number(42)]);
        assert_halted_at(&cpu, 4);
    }

    // #[test]
    // fn fn_no_args_no_return() {
    //     let expected_fn_def = FunctionDefinition {
    //         arity: 0,
    //         parameters: vec![],
    //         name: "foo".to_string(),
    //         label: "fn_1".to_string(),
    //         instruction_address: Some(3),
    //     };
    //     let mut cpu = CPU::with_op_codes(vec![
    //         OpCode::DefineFunction(expected_fn_def),
    //         OpCode::Store(0, 0),
    //         OpCode::Halt,
    //         OpCode::Load(0, 0),
    //         OpCode::PushFrame,
    //         OpCode::PopFrame,
    //         OpCode::Ret,
    //     ]);
    //     cpu.run();
    //     assert_halted_at(&cpu, 2);
    //     assert_eq!(cpu.stack, vec![]);
    // }

    // #[test]
    // fn fn_no_args_returns_int() {
    //     let mut cpu = CPU::with_op_codes(vec![
    //         OpCode::Call(2),
    //         OpCode::Halt,
    //         OpCode::Push(Value::Number(7)),
    //         OpCode::Ret,
    //     ]);
    //     cpu.run();
    //     assert_halted_at(&cpu, 2);
    //     assert_eq!(cpu.stack, vec![Value::Number(7)]);
    // }


    #[test]
    fn push_frame() {
        let mut cpu = CPU::with_op_codes(vec![
            OpCode::PushFrame,
            OpCode::Halt,
        ]);
        cpu.run();
        assert_halted_at(&cpu, 2);
        assert_eq!(cpu.environment().len(), 2);
    }

    #[test]
    fn pop_frame() {
        let mut cpu = CPU::with_op_codes(vec![
            OpCode::PushFrame,
            OpCode::PopFrame,
            OpCode::Halt,
        ]);
        cpu.run();
        assert_halted_at(&cpu, 3);
        assert_eq!(cpu.environment().len(), 1);
    }
}
