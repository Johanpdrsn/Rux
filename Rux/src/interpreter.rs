use crate::compiler::Compiler;
use crate::vm::VM;
pub struct Interpreter {
    vm: VM,
}

impl Interpreter {
    pub fn new() -> Self {
        Self { vm: VM::new() }
    }

    pub fn interpret(&mut self, source: &str) {
        let mut compiler = Compiler::from_source(source);
        let function = compiler.compile();

        if !compiler.had_error {
            self.vm.run_main(&function).unwrap();
        }
    }
}
