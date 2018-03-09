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
             -a 'Runs an assembly file'
             -o 'Prints out assembly code instead of running'
             <INPUT>              'Sets the input file to use'
                                                                               ")
                          .get_matches();

    let debug = match matches.occurrences_of("d") {
        0 => false,
        _ => true
    };

    let assem_in = match matches.occurrences_of("a") {
        0 => false,
        _ => true
    };

    let assem_out = match matches.occurrences_of("o") {
        0 => false,
        _ => true
    };
    run_file(matches.value_of("INPUT").unwrap(), debug, assem_in, assem_out);
}

fn run_file(filename: &str, debug: bool, assem_in: bool, assem_out: bool) {
    let mut contents = String::new();
    let mut file = File::open(filename).unwrap();
    file.read_to_string(&mut contents).unwrap();
    if assem_in {
        run_assembly(contents, debug);
    } else {
        stack_vm::runtime::run_program(contents, debug, assem_out);
    }
}

fn run_assembly(contents: String, debug: bool) {
    println!("running assembly");
    println!("{}", contents);
    let p = stack_vm::assembly::parse_Program(contents.as_str());
    let mut assembler = stack_vm::assembler::Assembler::new(p.unwrap());
    assembler.resolve_labels();
    let program = stack_vm::assembler::AssemblyProgram {
        instructions: Vec::new(),
        op_codes: assembler.generate_op_codes(),
        constant_pool: Vec::new(),
    };
    let mut cpu = stack_vm::cpu::CPU::new(program);
    cpu.debug = debug;
    cpu.run();
}
