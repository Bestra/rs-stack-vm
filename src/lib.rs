#![feature(nll)]

pub mod instruction;
pub mod assembly;
pub mod assembler;
pub mod ast;
pub mod compiler;
pub mod parser1;

use instruction::OpCode;

#[derive(Debug)]
pub struct Frame {
    variables: Vec<i32>,
    return_address: usize,
}

impl Frame {
    pub fn get_variable(&self, key: usize) -> i32 {
        *self.variables.get(key).unwrap_or(&0)
    }

    pub fn set_variable(&mut self, key: usize, val: i32) {
        self.variables.insert(key, val);
    }

    pub fn new(return_address: usize) -> Frame {
        Frame {
            variables: Vec::new(),
            return_address,
        }
    }
}

pub struct CPU {
    stack: Vec<i32>,
    program: Vec<OpCode>,
    current_frame_idx: usize,
    frames: Vec<Frame>,
    instruction_address: usize,
    halted: bool,
    pub debug: bool,
}

impl CPU {
    pub fn new(program: Vec<OpCode>) -> CPU {
        CPU {
            stack: Vec::new(),
            program: program,
            current_frame_idx: 0,
            frames: vec![Frame::new(0)],
            instruction_address: 0,
            halted: false,
            debug: false,
        }
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
        F: Fn(i32, i32) -> i32,
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
        assert!(i < self.program.len(), "Invalid jump address");
    }

    fn assert_return_address(&self) {
        assert!(self.frames.len() > 1, "Invalid Ret instruction: no current function call");
    }

    fn decode_next_instruction(&mut self) {
        assert!(self.instruction_address < self.program.len());
        let next_ins = &self.program[self.instruction_address];
        if self.debug {
            println!("{}: {:?} stack: {:?} locals: {:?}", self.instruction_address, next_ins, self.stack, self.current_frame());
        }
        self.instruction_address += 1;

        match *next_ins {
            OpCode::Halt => self.halted = true,
            OpCode::Label(_) => (), //no-op
            OpCode::Comment(_) => (), //no-op
            OpCode::Push(val) => self.stack.push(val),
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
                self.bin_op(|top, bot| ((top == 1) && (bot == 1)) as i32);
            }

            OpCode::Or => {
                self.bin_op(|top, bot| ((top == 1) || (bot == 1)) as i32);
            }

            OpCode::IsEq => {
                self.bin_op(|top, bot| (top == bot) as i32);
            }

            OpCode::IsGt => {
                self.bin_op(|top, bot| (top > bot) as i32);
            }

            OpCode::IsGe => {
                self.bin_op(|top, bot| (top >= bot) as i32);
            }

            OpCode::IsLt => {
                self.bin_op(|top, bot| (top < bot) as i32);
            }

            OpCode::Not => {
                assert!(
                    self.stack.len() >= 1,
                    "stack needs at least 1 values to add"
                );

                let top = self.stack.pop().unwrap() == 1;
                self.stack.push(!top as i32);
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
                self.stack.push(*n);
            }

            OpCode::Jmp(addr) => {
                self.instruction_address = addr;
            }

            OpCode::JmpIf(addr) => {
                let k = self.stack.pop().unwrap();
                if k == 1 {
                    self.instruction_address = addr;
                }
            }

            OpCode::Store(var_pos) => {
                assert!(
                    self.stack.len() >= 1,
                    "stack needs at least 1 value to store"
                );
                let k = self.stack.pop().unwrap();
                self.current_frame_mut().set_variable(var_pos, k);
            }

            OpCode::Load(var_pos) => {
                let v = self.current_frame().get_variable(var_pos);
                self.stack.push(v);
            }

            OpCode::Print => {
                let k = self.stack.pop().unwrap();
                println!("{}", k);
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

    fn assert_current_frame_value(cpu: &CPU, index: usize, value: i32) {
        assert_eq!(cpu.current_frame().get_variable(index), value);
    }

    fn assert_halted_at(cpu: &CPU, l: usize) {
        assert_eq!(l, cpu.instruction_address);
        assert!(cpu.halted);
    }

    fn test_stack_for_instructions(prog: Vec<OpCode>, s: Vec<i32>) {
        let l = prog.len();
        let mut cpu = CPU::new(prog);
        cpu.run();
        assert_eq!(l, cpu.instruction_address);
        assert!(cpu.halted);
        assert_eq!(cpu.stack, s);
    }

    fn test_final_address_for_instructions(prog: Vec<OpCode>, i: usize) {
        let mut cpu = CPU::new(prog);
        cpu.run();
        assert!(cpu.halted);
        assert_eq!(i, cpu.instruction_address);
    }

    #[test]
    fn empty_program() {
        let mut cpu = CPU::new(vec![OpCode::Halt]);
        cpu.step();
        assert_halted_at(&cpu, 1);
        assert_eq!(cpu.stack.len(), 0);
    }

    #[test]
    fn push_two_numbers() {
        test_stack_for_instructions(
            vec![
                OpCode::Push(42),
                OpCode::Push(68),
                OpCode::Halt,
            ],
            vec![42, 68],
        );
    }

    #[test]
    fn add_two_numbers() {
        test_stack_for_instructions(
            vec![
                OpCode::Push(42),
                OpCode::Push(68),
                OpCode::Add,
                OpCode::Halt,
            ],
            vec![110],
        );
    }

    #[test]
    fn subtract_two_numbers() {
        test_stack_for_instructions(
            vec![
                OpCode::Push(1),
                OpCode::Push(2),
                OpCode::Subtract,
                OpCode::Halt,
            ],
            vec![-1],
        );
    }

    #[test]
    fn multiply_two_numbers() {
        test_stack_for_instructions(
            vec![
                OpCode::Push(3),
                OpCode::Push(5),
                OpCode::Multiply,
                OpCode::Halt,
            ],
            vec![15],
        );
    }

    #[test]
    fn pop() {
        test_stack_for_instructions(
            vec![
                OpCode::Push(3),
                OpCode::Push(5),
                OpCode::Pop,
                OpCode::Halt,
            ],
            vec![3],
        )
    }

    #[test]
    fn dup() {
        test_stack_for_instructions(
            vec![OpCode::Push(3), OpCode::Dup, OpCode::Halt],
            vec![3, 3],
        );
    }

    #[test]
    fn is_eq() {
        test_stack_for_instructions(
            vec![
                OpCode::Push(3),
                OpCode::Push(3),
                OpCode::IsEq,
                OpCode::Halt,
            ],
            vec![1],
        );
        test_stack_for_instructions(
            vec![
                OpCode::Push(5),
                OpCode::Push(3),
                OpCode::IsEq,
                OpCode::Halt,
            ],
            vec![0],
        )
    }

    #[test]
    fn is_lt() {
        test_stack_for_instructions(
            vec![
                OpCode::Push(3),
                OpCode::Push(4),
                OpCode::IsLt,
                OpCode::Halt,
            ],
            vec![0],
        );
        test_stack_for_instructions(
            vec![
                OpCode::Push(5),
                OpCode::Push(3),
                OpCode::IsLt,
                OpCode::Halt,
            ],
            vec![1],
        )
    }

    #[test]
    fn is_gt() {
        test_stack_for_instructions(
            vec![
                OpCode::Push(3),
                OpCode::Push(4),
                OpCode::IsGt,
                OpCode::Halt,
            ],
            vec![1],
        );
        test_stack_for_instructions(
            vec![
                OpCode::Push(5),
                OpCode::Push(3),
                OpCode::IsGt,
                OpCode::Halt,
            ],
            vec![0],
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
                OpCode::Push(1),
                OpCode::JmpIf(3),
                OpCode::Pop,
                OpCode::Push(0),
                OpCode::JmpIf(2),
                OpCode::Halt,
            ],
            6,
        )
    }

    #[test]
    fn store_var() {
        let mut cpu = CPU::new(vec![
            OpCode::Push(42),
            OpCode::Store(0),
            OpCode::Halt,
        ]);
        cpu.run();
        assert_halted_at(&cpu, 3);
        assert_current_frame_value(&cpu, 0, 42);
    }

    #[test]
    fn load_uninitialized_var() {
        let mut cpu = CPU::new(vec![OpCode::Load(0), OpCode::Halt]);
        cpu.run();
        assert_eq!(cpu.stack, vec![0]);
        assert_halted_at(&cpu, 2);
    }

    #[test]
    fn load_stored_var() {
        let mut cpu = CPU::new(vec![
            OpCode::Push(42),
            OpCode::Store(0),
            OpCode::Load(0),
            OpCode::Halt,
        ]);
        cpu.run();
        assert_eq!(cpu.stack, vec![42]);
        assert_halted_at(&cpu, 4);
    }

    #[test]
    fn fn_no_args_no_return() {
        let mut cpu = CPU::new(vec![
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
        let mut cpu = CPU::new(vec![
            OpCode::Call(2),
            OpCode::Halt,
            OpCode::Push(7),
            OpCode::Ret,
        ]);
        cpu.run();
        assert_halted_at(&cpu, 2);
        assert_eq!(cpu.stack, vec![7]);
    }

    #[test]
    fn fn_doubles_given_arg() {
        let mut cpu = CPU::new(vec![
            OpCode::Push(3),
            OpCode::Call(3),
            OpCode::Halt,
            OpCode::Push(2),
            OpCode::Multiply,
            OpCode::Ret,
        ]);
        cpu.run();
        assert_halted_at(&cpu, 3);
        assert_eq!(cpu.stack, vec![6]);
    }

    #[test]
    fn test_max_a_b() {
        let mut cpu = CPU::new(vec![
            OpCode::Push(6),
            OpCode::Push(4),
            OpCode::Call(4),
            OpCode::Halt,
            OpCode::Store(1),
            OpCode::Store(0),
            OpCode::Load(1),
            OpCode::Load(0),
            OpCode::Print,
            OpCode::IsGe,
            OpCode::Print,
            OpCode::JmpIf(14),
            OpCode::Load(1),
            OpCode::Ret,
            OpCode::Load(0),
            OpCode::Ret,
        ]);
        cpu.run();
        assert_halted_at(&cpu, 4);
        assert_eq!(cpu.stack, vec![6]);
    }
}
