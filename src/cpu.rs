use instruction::OpCode;
use assembler::AssemblyProgram;
use value::Value;

#[derive(Debug)]
pub struct Frame {
    variables: Vec<Value>,
    return_address: usize,
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

    pub fn new(return_address: usize) -> Frame {
        Frame {
            variables: Vec::with_capacity(8),
            return_address,
        }
    }
}

pub struct CPU {
    stack: Vec<Value>,
    print_buffer: Vec<String>,
    program: AssemblyProgram,
    current_frame_idx: usize,
    frames: Vec<Frame>,
    instruction_address: usize,
    halted: bool,
    pub debug: bool,
}

impl CPU {
    pub fn new(program: AssemblyProgram) -> CPU {
        CPU {
            stack: Vec::new(),
            print_buffer: Vec::new(),
            program: program,
            current_frame_idx: 0,
            frames: vec![Frame::new(0)],
            instruction_address: 0,
            halted: false,
            debug: false,
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

    pub fn get_frame(&self, idx: usize) -> &Frame {
        self.frames.get(idx).unwrap()
    }

    pub fn get_frame_mut(&mut self, idx: usize) -> &mut Frame {
        self.frames.get_mut(idx).unwrap()
    }

    pub fn current_frame(&self) -> &Frame {
        self.frames.get(self.current_frame_idx).unwrap()
    }

    pub fn current_frame_mut(&mut self) -> &mut Frame {
        self.frames.get_mut(self.current_frame_idx).unwrap()
    }

    pub fn run(&mut self) {
        while !self.halted {
            self.step();
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
        assert!(self.frames.len() > 1, "Invalid Ret instruction: no current function call");
    }

    pub fn load_constant(&mut self, idx: usize) {
        let c = self.program.constant_pool.get(idx).unwrap().clone();
        self.stack.push(c);
    }

    pub fn load_local(&mut self, frame_idx: usize, var_pos: usize) {
            let v = self.get_frame(frame_idx).get_variable(var_pos);
            self.stack.push(v);
    }

    pub fn store_local(&mut self, frame_idx: usize, var_pos: usize) {
        assert!(
            self.stack.len() >= 1,
            "stack needs at least 1 value to store"
        );
        let k = self.stack.pop().unwrap();
        self.get_frame_mut(frame_idx).set_variable(var_pos, k);
    }

    fn decode_next_instruction(&mut self) {
        assert!(self.instruction_address < self.program.op_codes.len());
        let next_ins = &self.program.op_codes[self.instruction_address];
        if self.debug {
            println!("{:<3}: {:15} stack: {:?} locals: {:?}", self.instruction_address, next_ins, self.stack, self.current_frame());
        }
        self.instruction_address += 1;

        match *next_ins {
            OpCode::Halt => self.halted = true,
            OpCode::NoOp => (), //no-op
            OpCode::Push(ref val) => self.stack.push(val.clone()),
            OpCode::Constant(i) => {
                self.load_constant(i);
            }
            OpCode::Add => {
                self.bin_op(|top, bot| top + bot);
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
                self.current_frame_mut().set_variable(var_pos, k);
            }

            OpCode::LoadLocal(var_pos) => {
                let v = self.current_frame().get_variable(var_pos);
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

            OpCode::Call(i) => {
                self.assert_jump_address(i);
                self.frames.push(Frame::new(self.instruction_address));
                self.current_frame_idx += 1;
                self.instruction_address = i;
            }

            OpCode::CallFn => {
                let function_def = self.stack.pop().unwrap();
                match function_def {
                    Value::Fn(d) => {
                        // at the point a function is called its arguments
                        // will be on the stack in reverse order.
                        // foo(a, b) will be called with
                        // bottom --> [a, b] --> top
                        assert!(self.stack.len() >= d.arity,
                                "For function to be called all its arguments must be on the stack");
                        let i = d.instruction_address.unwrap();
                        // WW this is the same OpCode::Call
                        self.assert_jump_address(i);
                        self.frames.push(Frame::new(self.instruction_address));
                        self.current_frame_idx += 1;
                        self.instruction_address = i;
                        // MM this is the same OpCode::Call
                    }
                    _ => panic!("In order to call a function its definition must be at the top of the stack.")
                }
            }

            OpCode::PushFrame => {
                self.frames.push(Frame::new(0));
                self.current_frame_idx += 1;
            }

            OpCode::PopFrame => {
                self.frames.pop();
                self.current_frame_idx -= 1;
            }

            OpCode::Ret => {
                self.assert_return_address();
                let return_address = self.current_frame().return_address;
                self.frames.pop();
                self.current_frame_idx -= 1;
                self.instruction_address = return_address;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_current_frame_value(cpu: &CPU, index: usize, value: Value) {
        assert_eq!(cpu.current_frame().get_variable(index), value);
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

    #[test]
    fn fn_no_args_no_return() {
        let mut cpu = CPU::with_op_codes(vec![
            OpCode::Call(2),
            OpCode::Halt,
            OpCode::Ret,
        ]);
        cpu.run();
        assert_halted_at(&cpu, 2);
        assert_eq!(cpu.stack, vec![]);
    }

    #[test]
    fn fn_no_args_returns_int() {
        let mut cpu = CPU::with_op_codes(vec![
            OpCode::Call(2),
            OpCode::Halt,
            OpCode::Push(Value::Number(7)),
            OpCode::Ret,
        ]);
        cpu.run();
        assert_halted_at(&cpu, 2);
        assert_eq!(cpu.stack, vec![Value::Number(7)]);
    }

    #[test]
    fn fn_doubles_given_arg() {
        let mut cpu = CPU::with_op_codes(vec![
            OpCode::Push(Value::Number(3)),
            OpCode::Call(3),
            OpCode::Halt,
            OpCode::Push(Value::Number(2)),
            OpCode::Multiply,
            OpCode::Ret,
        ]);
        cpu.run();
        assert_halted_at(&cpu, 3);
        assert_eq!(cpu.stack, vec![Value::Number(6)]);
    }

    #[test]
    fn push_frame() {
        let mut cpu = CPU::with_op_codes(vec![
            OpCode::PushFrame,
            OpCode::Halt,
        ]);
        cpu.run();
        assert_halted_at(&cpu, 2);
        assert_eq!(cpu.frames.len(), 2);
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
        assert_eq!(cpu.frames.len(), 1);
    }
}
