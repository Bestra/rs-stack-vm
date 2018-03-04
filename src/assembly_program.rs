use instruction::OpCode;
use value::Value;

pub struct AssemblyProgram {
    pub op_codes: Vec<OpCode>,
    pub constant_pool: Vec<Value>,
}
