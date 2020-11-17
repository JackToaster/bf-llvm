use inkwell::OptimizationLevel;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::Module;
use inkwell::targets::{InitializationConfig, Target};
use std::error::Error;

use crate::parser::ast::ASTBranch;

use super::codegen::CodeGen;

pub type Compiled = unsafe extern "C" fn() -> ();

pub fn compile(ast: ASTBranch) -> Result<(), Box<dyn Error>>{
    let context = Context::create();
    let module = context.create_module("bf");
    let execution_engine = module.create_jit_execution_engine(OptimizationLevel::None)?;
    let codegen = CodeGen {
        context: &context,
        module,
        builder: context.create_builder(),
    };

    codegen.gen_test();
    
    unsafe {
        let bf: JitFunction<Compiled> = execution_engine.get_function("bf").ok().ok_or("Could not compile code")?;
        println!("Calling function: {:?}", bf);
        bf.call();
        println!("Called function");
    }

    Ok(())
}