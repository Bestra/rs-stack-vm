use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::fmt;
use std::rc::Rc;
use std::cmp::Ordering;

use function::FunctionDefinition;

#[derive(Debug,Clone)]
pub enum Value {
    Number(i32),
    Bool(bool),
    String(String),
    Fn(Rc<FunctionDefinition>),
    // Float(f64),
}

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        use value::Value::*;
        match (self, other) {
            (&Number(i), &Number(j)) => i == j,
            (&Bool(i), &Bool(j)) => i == j,
            (&String(ref i), &String(ref j)) => i == j,
            (&Fn(ref i), &Fn(ref j)) => Rc::ptr_eq(i, j),
            _ => false,
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        use value::Value::*;
        match (self, other) {
            (&Number(i), &Number(j)) => Some(i.cmp(&j)),
            (&Bool(i), &Bool(j)) => Some(i.cmp(&j)),
            (&String(ref i), &String(ref j)) => Some(i.cmp(j)),
            _ => None,
        }
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, other: Value) -> Value {
        match (&self, &other) {
            (&Value::Number(a), &Value::Number(b)) => Value::Number(a + b),
            (&Value::String(ref a), &Value::String(ref b)) => Value::String(format!("{}{}", a, b)),
            _ => panic!("Unable to add types {:?} and {:?}", self, other)
        }
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, other: Value) -> Value {
            match (&self, &other) {
            (&Value::Number(a), &Value::Number(b)) => Value::Number(a - b),
            _ => panic!("Unable to subtract types {:?} and {:?}", self, other)
        }
    }
}


impl Mul for Value {
    type Output = Value;

    fn mul(self, other: Value) -> Value {
        match (&self, &other) {
            (&Value::Number(a), &Value::Number(b)) => Value::Number(a * b),
            _ => panic!("Unable to multiply types {:?} and {:?}", self, other)
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Value::Number(a) => write!(f, "{}", a),
            &Value::String(ref a) => write!(f, "{}", a),
            &Value::Bool(a) => write!(f, "{}", a),
            &Value::Fn(ref _a) => write!(f, "Function definition"),
        }
    }
}
