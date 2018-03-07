use compiler;
use cpu;

pub fn run_program(contents: String, debug: bool, assem_out: bool) -> Option<Vec<String>> {
    let a = compiler::compile(contents.as_str());
    if assem_out {
        println!("{:#?}", a.instructions);
        None
    } else {
        let mut cpu = cpu::CPU::new(a);
        cpu.debug = debug;
        cpu.run();
        Some(cpu.print_buffer().clone())
    }
}
