pub mod chunk;
pub mod value;
fn main() {
    let mut a = chunk::Chunk::new();
    let con = a.add_constant(value::Value::Number(1.2));
    let b = a.read_constant(con);
    a.write(chunk::OpCode::OpConstant, 123);
    // a.write(b, 123);
    a.write(chunk::OpCode::OpReturn, 123);
    a.disassemble("test");
}
