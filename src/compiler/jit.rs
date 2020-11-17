use inkwell::OptimizationLevel;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::Module;
use inkwell::targets::{InitializationConfig, Target};
use std::{error::Error, io::{self, Read, Write}};

use crate::parser::ast::ASTBranch;

use super::codegen::{BF_ARRAY_SIZE, CodeGen};


extern "C" fn bf_putchar(character: i32) -> () {
    print!("{}", character as u8 as char);
    io::stdout().flush();
    // println!("Printed character with id: {}", character)
}

extern "C" fn bf_getchar() -> i32 {
    let mut input = std::io::stdin();
    let mut buf = [0u8];

    if input.read(&mut buf).unwrap() == 0 {
        return 0xFF; // EOF
    }

    buf[0] as i32
}

pub type Compiled = unsafe extern "C" fn(
    unsafe extern "C" fn(character: i32),
    unsafe extern "C" fn() -> i32,
    *const i32
) -> ();

pub fn compile<'a>(ast: ASTBranch) -> Result<(), Box<dyn Error>>{
    let context = Context::create();
    let module = context.create_module("bf");
    let execution_engine = module.create_jit_execution_engine(OptimizationLevel::Aggressive)?;
    let codegen = CodeGen {
        context: &context,
        module,
        builder: context.create_builder(),
    };

    codegen.gen_bf(ast);

    let bf_array: [i32; BF_ARRAY_SIZE] = [0; BF_ARRAY_SIZE];

    unsafe {
        let compiled: JitFunction<Compiled> = execution_engine.get_function("bf").ok().ok_or("Could not compile code")?;
        compiled.call(bf_putchar, bf_getchar, bf_array.as_ptr());
    }

    Ok(())
}