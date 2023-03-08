use crate::chunk::OpCode;
use crate::value::Value;
use crate::vm::VM;
mod chunk;
mod stack;
mod value;
mod vm;
fn main() {
    let mut vm: VM = VM::new();

    vm.chunk.emit_constant(Value::Number(5.0));
    vm.chunk.emit_constant(Value::Number(6.0));

    vm.chunk.write(OpCode::Add, 123);
    vm.chunk.write(OpCode::Return, 123);
    println!("{:?}", vm.run());
}
