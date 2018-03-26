extern crate cursive;
use self::cursive::Cursive;
use self::cursive::views::{Dialog, TextView, LinearLayout};

use compiler;
use cpu::CPU;

pub fn run_program(contents: String, debug: bool, assem_out: bool) -> Option<Vec<String>> {
    let a = compiler::compile(contents.as_str());
    if assem_out {
        for i in a.instructions {
            println!("{}", i);
        }
        None
    } else {
        let mut cpu = CPU::new(a);
        cpu.debug = debug;
        cpu.run_to_debugger();
        if cpu.halted {
            Some(cpu.print_buffer().clone())
        } else {
            let finished_cpu = start_debugger(cpu);
            Some(finished_cpu.print_buffer().clone())
        }
    }
}

fn pretty_text_stack(cpu: &CPU) -> String {
    let mut v = Vec::new();

    if cpu.stack.len() > 0 {
        for i in &cpu.stack {
            v.push(format!("{:#?}", i))
        }

        v.join("\n")
    } else {
        "               ".to_string()
    }
}

fn start_debugger(mut cpu: CPU) -> CPU {
    while !cpu.halted {
        show_debugger(&cpu);
        cpu.step();
    }

    cpu
}

fn show_debugger(cpu: &CPU) {
    // Creates the cursive root - required for every application.
    let mut siv = Cursive::new();

    // Creates a dialog with a single "Quit" button
    let next_ins = cpu.next_instruction();
    let list = cpu.program.op_code_list(Some(cpu.instruction_address));
    let stack = pretty_text_stack(&cpu);
    let env = cpu.environment();
    let env_display = format!("{:#?}", env.frames);

    let inner_view = LinearLayout::horizontal()
        .child(TextView::new(list))
        // .child(TextView::new(next_ins.as_str()))
        .child(TextView::new(env_display.as_str()))
        .child(TextView::new(stack.as_str()))
        ;
    let constants = format!("{:?}", cpu.program.constant_pool);
    let call_stack_view = format!("{} call stack frames", cpu.call_stack.len());
    let vert_view = LinearLayout::vertical()
        .child(TextView::new(constants))
        .child(TextView::new(call_stack_view))
        .child(inner_view);
    siv.add_layer(Dialog::around(vert_view)
                  .title("Cursive")
                  .button("Next (n)", |s| s.quit()));

    siv.add_global_callback('n', |s| s.quit());
    // Starts the event loop.
    siv.run();
}
