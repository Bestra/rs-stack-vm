extern crate stack_vm;

use stack_vm::runtime::run_program;

fn test_output(input: &str, expected: Vec<&str>) {
    match run_program(input.to_string(), false, false) {
        Some(o) => assert_eq!(o, expected),
        None => assert!(false, "no program output")
    }
}

#[test]
fn if_statement() {
    test_output("if (true) print \"t\"; end", vec!["t"]);
    test_output("if (false) print \"t\"; end", vec![]);
    test_output("if (false) print \"t\"; end print \"after\";", vec!["after"]);
    test_output("if (false) print \"t\"; else print \"f\"; end", vec!["f"]);
}

#[test]
fn logical_and_falls_through() {
    test_output("print true && \"t\";", vec!["t"]);
}

#[test]
fn logical_and_short_circuits() {
    test_output("print false && \"t\";", vec!["false"]);
}
