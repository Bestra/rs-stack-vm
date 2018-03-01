extern crate stack_vm;
extern crate clap;

use std::fs::File;
use std::io::prelude::*;
use clap::{App};

fn main() {
    let matches = App::new("stack_vm")
        .version("1.0")
        .author("Chris Westra")
        .about("assembler stack vm built in rust (in progress)")
        .args_from_usage(
            "-d 'Prints stack and frame information before running each opcode'
             -assembler 'Runs an assembly file'
                              <INPUT>              'Sets the input file to use'
                                                                               ")
                          .get_matches();

    let debug = match matches.occurrences_of("d") {
        0 => false,
        _ => true
    };

    let assem = match matches.occurrences_of("assembler") {
        0 => false,
        _ => true
    };
    run_file(matches.value_of("INPUT").unwrap(), debug, assem);
}

fn run_file(filename: &str, debug: bool, assem: bool) {
    let mut contents = String::new();
    let mut file = File::open(filename).unwrap();
    file.read_to_string(&mut contents).unwrap();
    if assem {
        run_assembly(contents, debug);
    } else {
        run_program(contents, debug);
    }
}

fn run_assembly(contents: String, debug: bool) {
    let p = stack_vm::assembly::parse_Program(contents.as_str());
    let mut assembler = stack_vm::assembler::Assembler::new(p.unwrap());
    assembler.resolve_labels();
    let mut cpu = stack_vm::CPU::new(assembler.generate_op_codes());
    cpu.debug = debug;
    cpu.run();
}

fn run_program(contents: String, debug: bool) {
    let p = stack_vm::parser1::parse_Program(contents.as_str());
    let mut compiler = stack_vm::compiler::Compiler::new();
    let instructions = compiler.generate_instructions(p.unwrap());
    let mut assembler = stack_vm::assembler::Assembler::new(instructions);
    assembler.resolve_labels();
    let mut cpu = stack_vm::CPU::new(assembler.generate_op_codes());
    cpu.debug = debug;
    cpu.run();
}
