#![feature(nll)]

pub enum Instruction {
    Halt,
    Push(i32),
    Add,
    Subtract,
    Multiply,
    Not,
    And,
    Or,
    Pop,
    Dup,
    IsEq,
    IsGt,
    IsLt,
}

pub struct CPU {
    stack: Vec<i32>,
    program: Vec<Instruction>,
    instruction_address: usize,
    halted: bool,
}

impl CPU {
    pub fn new(program: Vec<Instruction>) -> CPU {
        CPU {
            stack: Vec::new(),
            program: program,
            instruction_address: 0,
            halted: false,
        }
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

    fn decode_next_instruction(&mut self) {
        assert!(self.instruction_address < self.program.len());
        let next_ins = &self.program[self.instruction_address];
        self.instruction_address += 1;
        match *next_ins {
            Instruction::Halt => self.halted = true,
            Instruction::Push(val) => self.stack.push(val),
            Instruction::Add => {
                self.bin_op(|top, bot| top + bot);
            }
            Instruction::Subtract => {
                self.bin_op(|top, bot| bot - top);
            }

            Instruction::Multiply => {
                self.bin_op(|top, bot| top * bot);
            }

            Instruction::And => {
                self.bin_op(|top, bot| ((top == 1) && (bot == 1)) as i32);
            }

            Instruction::Or => {
                self.bin_op(|top, bot| ((top == 1) || (bot == 1)) as i32);
            }

            Instruction::IsEq => {
                self.bin_op(|top, bot| (top == bot) as i32);
            }

            Instruction::IsGt => {
                self.bin_op(|top, bot| (top > bot) as i32);
            }

            Instruction::IsLt => {
                self.bin_op(|top, bot| (top < bot) as i32);
            }

            Instruction::Not => {
                assert!(
                    self.stack.len() >= 1,
                    "stack needs at least 1 values to add"
                );

                let top = self.stack.pop().unwrap() == 1;
                self.stack.push(!top as i32);
            }

            Instruction::Pop => {
                assert!(self.stack.len() >= 1, "stack needs at least 1 value to pop");
                self.stack.pop();
            }

            Instruction::Dup => {
                assert!(
                    self.stack.len() >= 1,
                    "stack needs at least 1 value to duplicate"
                );
                let n = self.stack.last().unwrap();
                self.stack.push(*n);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_stack_for_instructions(prog: Vec<Instruction>, s: Vec<i32>) {
        let l = prog.len();
        let mut cpu = CPU::new(prog);
        cpu.run();
        assert_eq!(l, cpu.instruction_address);
        assert!(cpu.halted);
        assert_eq!(cpu.stack, s);
    }

    #[test]
    fn empty_program() {
        let mut cpu = CPU::new(vec![Instruction::Halt]);
        cpu.step();
        assert_eq!(1, cpu.instruction_address);
        assert!(cpu.halted);
        assert_eq!(cpu.stack.len(), 0);
    }

    #[test]
    fn push_two_numbers() {
        test_stack_for_instructions(
            vec![
                Instruction::Push(42),
                Instruction::Push(68),
                Instruction::Halt,
            ],
            vec![42, 68],
        );
    }

    #[test]
    fn add_two_numbers() {
        test_stack_for_instructions(
            vec![
                Instruction::Push(42),
                Instruction::Push(68),
                Instruction::Add,
                Instruction::Halt,
            ],
            vec![110],
        );
    }

    #[test]
    fn subtract_two_numbers() {
        test_stack_for_instructions(
            vec![
                Instruction::Push(1),
                Instruction::Push(2),
                Instruction::Subtract,
                Instruction::Halt,
            ],
            vec![-1],
        );
    }

    #[test]
    fn multiply_two_numbers() {
        test_stack_for_instructions(
            vec![
                Instruction::Push(3),
                Instruction::Push(5),
                Instruction::Multiply,
                Instruction::Halt,
            ],
            vec![15],
        );
    }

    #[test]
    fn pop() {
        test_stack_for_instructions(
            vec![
                Instruction::Push(3),
                Instruction::Push(5),
                Instruction::Pop,
                Instruction::Halt,
            ],
            vec![3],
        )
    }

    #[test]
    fn dup() {
        test_stack_for_instructions(
            vec![Instruction::Push(3), Instruction::Dup, Instruction::Halt],
            vec![3, 3],
        );
    }

    #[test]
    fn is_eq() {
        test_stack_for_instructions(
            vec![
                Instruction::Push(3),
                Instruction::Push(3),
                Instruction::IsEq,
                Instruction::Halt,
            ],
            vec![1],
        );
        test_stack_for_instructions(
            vec![
                Instruction::Push(5),
                Instruction::Push(3),
                Instruction::IsEq,
                Instruction::Halt,
            ],
            vec![0],
        )
    }

    #[test]
    fn is_lt() {
        test_stack_for_instructions(
            vec![
                Instruction::Push(3),
                Instruction::Push(4),
                Instruction::IsLt,
                Instruction::Halt,
            ],
            vec![0],
        );
        test_stack_for_instructions(
            vec![
                Instruction::Push(5),
                Instruction::Push(3),
                Instruction::IsLt,
                Instruction::Halt,
            ],
            vec![1],
        )
    }

    #[test]
    fn is_gt() {
        test_stack_for_instructions(
            vec![
                Instruction::Push(3),
                Instruction::Push(4),
                Instruction::IsGt,
                Instruction::Halt,
            ],
            vec![1],
        );
        test_stack_for_instructions(
            vec![
                Instruction::Push(5),
                Instruction::Push(3),
                Instruction::IsGt,
                Instruction::Halt,
            ],
            vec![0],
        )
    }
}
