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
fn logical_assignment() {
    test_output("
var a = true && \"t\";
print a;", vec!["t"]);
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

#[test]
fn for_loop() {
    test_output("
for (var i = 0; i < 3; i = i + 1) print i;
", vec!["0", "1", "2"]);
}

#[test]
fn for_loop_block() {
    test_output("
for (var i = 0; i < 3; i = i + 1) { print i + 1; }
", vec!["1", "2", "3"]);
}

#[test]
fn for_loop_no_initializer() {
    test_output("
var i = 1;
for (; i < 3; i = i + 1) { print i; }
", vec!["1", "2"]);
}

#[test]
fn fun_print_arg() {
   test_output("
fun hello(a) {
  print \"hello \" + a;
}

hello(\"world\");
", vec!["hello world"]);
}

#[test]
fn fun_return() {
    test_output("
fun add1(a) {
  return a + 1;
}

print add1(1);
", vec!["2"]);
}

#[test]
fn fun_multiple_args() {
    test_output("
fun prints(x, y) {
  print x;
  print y;
}

prints(\"foo\", \"bar\");

", vec!["foo", "bar"]);
}

#[test]
fn fun_closures() {
    test_output("
fun makePoint(x, y) {
  fun closure(method) {
    if (method == \"x\") return x; end
    if (method == \"y\") return y; end
    print \"unknown method \" + method;
  }

  return closure;
}

var point = makePoint(2, 3);
print point(\"x\");
print point(\"y\");
", vec!["2", "3"]);
}
