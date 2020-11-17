use inkwell::{IntPredicate, values::IntValue};

use crate::parser::ast::{ASTBranch, ASTElement, Operation};

use super::codegen::{BfGlobals, Generate};


impl<'ctx> Generate<'ctx> for ASTBranch {
    fn generate(&self, context: &'ctx inkwell::context::Context, builder: &inkwell::builder::Builder<'ctx>, globals: &BfGlobals) {
        for element in self.0.iter() {
            element.generate(context, builder, globals);
        }
    }
}


impl<'ctx> Generate<'ctx> for ASTElement {
    fn generate(&self, context: &'ctx inkwell::context::Context, builder: &inkwell::builder::Builder<'ctx>, globals: &BfGlobals) {
        match self {
            ASTElement::Loop(contents) => {
                let head = context.append_basic_block(globals.function, "");
                let body = context.append_basic_block(globals.function, "");
                let tail = context.append_basic_block(globals.function, "");   

                builder.build_unconditional_branch(head);
                builder.position_at_end(head);


                // Branch to skip loop body if the value is zero
                let branch_condition = builder.build_int_compare(
                    IntPredicate::EQ,
                    generate_get(context, builder, globals),
                    context.i8_type().const_int(0, false),
                    "",
                );

                builder.build_conditional_branch(branch_condition, tail, body);


                // Go to the body and fill it with the loop contents
                builder.position_at_end(body);
                contents.generate(context, builder, globals);
                // Branch from loop body end to loop start.
                builder.build_unconditional_branch(head);

                // Put the builder back at the end
                builder.position_at_end(tail);
            }
            ASTElement::Operation(op) => {op.generate(context, builder, globals)}
        }
    }
}


impl<'ctx> Generate<'ctx> for Operation {
    fn generate(&self, context: &'ctx inkwell::context::Context, builder: &inkwell::builder::Builder<'ctx>, globals: &BfGlobals) {
        match self {
            Operation::Read => {
                let res = builder.build_call(globals.getchar, &[], "").try_as_basic_value().left().unwrap().into_int_value();
                generate_set(context, builder, globals, res);
            }
            Operation::Write => {
                let current = generate_get(context, builder, globals);
                builder.build_call(globals.putchar, &[current.into()], "");
            }
            Operation::Increment => {
                let current = generate_get(context, builder, globals);
                let incremented = builder.build_int_add(current, context.i32_type().const_int(1, false), "");
                generate_set(context, builder, globals, incremented);
            }
            Operation::Decrement => {
                let current = generate_get(context, builder, globals);
                let incremented = builder.build_int_sub(current, context.i32_type().const_int(1, false), "");
                generate_set(context, builder, globals, incremented);
            }
            Operation::PtrLeft => {
                let ptr_value = builder.build_load(globals.array_ptr, "").into_int_value();
                let incremented = builder.build_int_sub(ptr_value, context.i32_type().const_int(1, false), "");
                builder.build_store(globals.array_ptr, incremented);
            }
            Operation::PtrRight => {
                let ptr_value = builder.build_load(globals.array_ptr, "").into_int_value();
                let incremented = builder.build_int_add(ptr_value, context.i32_type().const_int(1, false), "");
                builder.build_store(globals.array_ptr, incremented);
            }
        }
    }
}


fn generate_set<'ctx>(context: &'ctx inkwell::context::Context, builder: &inkwell::builder::Builder<'ctx>, globals: &BfGlobals, value: IntValue) {
    let array_ptr_value = builder.build_load(globals.array_ptr, "");
    let value_ptr = unsafe {
        builder.build_in_bounds_gep(
            globals.array,
            &[
                context.i32_type().const_int(0, false),
                array_ptr_value.into_int_value(),
            ],
            "",
        )
    };

    builder.build_store(value_ptr, value);
}


fn generate_get<'ctx>(context: &'ctx inkwell::context::Context, builder: &inkwell::builder::Builder<'ctx>, globals: &BfGlobals<'ctx>) -> IntValue<'ctx> {
    let array_ptr_value = builder.build_load(globals.array_ptr, "");
    let value_ptr = unsafe {
        builder.build_in_bounds_gep(
            globals.array,
            &[
                context.i32_type().const_int(0, false),
                array_ptr_value.into_int_value(),
            ],
            "",
        )
    };

    builder.build_load(value_ptr, "").into_int_value()
}
