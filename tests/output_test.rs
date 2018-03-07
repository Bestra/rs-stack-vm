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
fn logical_gt() {
    test_output("print 1 > 2;", vec!["false"]);
    test_output("print 2 > 2;", vec!["false"]);
    test_output("print 3 > 2;", vec!["true"]);
}

#[test]
fn logical_lt() {
    test_output("print 1 < 2;", vec!["true"]);
    test_output("print 2 < 2;", vec!["false"]);
    test_output("print 3 < 2;", vec!["false"]);
}

#[test]
fn logical_and_falls_through() {
    test_output("print true && \"t\";", vec!["t"]);
}

#[test]
fn logical_and_short_circuits() {
    test_output("print false && \"t\";", vec!["false"]);
}

#[test]
fn while_loop() {
    test_output("
  var i = 0;
  while (i < 3) {
    print i;
    i = i + 1;
  }
", vec!["0", "1", "2"]);
}