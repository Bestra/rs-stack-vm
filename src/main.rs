extern crate stack_vm;

use std::env;
use std::fs::File;
use std::io::prelude::*;


fn main() {
    let args: Vec<String> = env::args().collect();
    run_file(&args[1]);
}

fn run_file(filename: &str) {
    let mut contents = String::new();
    let mut file = File::open(filename).unwrap();
    file.read_to_string(&mut contents).unwrap();
    let p = stack_vm::assembly::parse_Program(contents.as_str());
    let mut a = stack_vm::assembler::Assembler::new(p.unwrap());
    a.resolve_labels();
    let mut c = stack_vm::CPU::new(a.generate_op_codes());
    c.run();
}
