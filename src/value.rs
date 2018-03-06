use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::fmt;

#[derive(PartialEq,PartialOrd,Debug,Clone)]
pub enum Value {
    Number(i32),
    Bool(bool),
    String(String),
    // Float(f64),
}

impl Add for Value {
    type Output = Value;

    fn add(self, other: Value) -> Value {
        match (&self, &other) {
            (&Value::Number(a), &Value::Number(b)) => Value::Number(a + b),
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
        }
    }
}
