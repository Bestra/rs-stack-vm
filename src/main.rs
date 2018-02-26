extern crate stack_vm;
extern crate clap;

use std::fs::File;
use std::io::prelude::*;
use clap::{App};

fn main() {
    let matches = App::new("stack_vm")
        .version("1.0")
        .author("Chris Westra")
        .about("a stack vm built in rust (in progress)")
        .args_from_usage(
            "-d 'Prints stack and frame information before running each opcode'
                              <INPUT>              'Sets the input file to use'
                                                                               ")
                          .get_matches();

    let debug = match matches.occurrences_of("d") {
        0 => false,
        _ => true
    };
    run_file(matches.value_of("INPUT").unwrap(), debug);
}

fn run_file(filename: &str, debug: bool) {
    let mut contents = String::new();
    let mut file = File::open(filename).unwrap();
    file.read_to_string(&mut contents).unwrap();
    let p = stack_vm::assembly::parse_Program(contents.as_str());
    let mut a = stack_vm::assembler::Assembler::new(p.unwrap());
    a.resolve_labels();
    let mut c = stack_vm::CPU::new(a.generate_op_codes());
    c.debug = debug;
    c.run();
}
