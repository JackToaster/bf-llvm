use std::convert::TryInto;

use inkwell::{AddressSpace, builder::Builder, values::{FunctionValue, PointerValue}};
use inkwell::context::Context;
use inkwell::module::Module;

use crate::parser::ast::ASTBranch;

pub(crate) const BF_ARRAY_SIZE: usize = 30000;

pub(crate) struct CodeGen<'ctx> {
    pub(crate) context: &'ctx Context,
    pub(crate) module: Module<'ctx>,
    pub(crate) builder: Builder<'ctx>,
}

pub(crate) struct BfGlobals<'ctx> {
    pub(crate) function: FunctionValue<'ctx>,
    pub(crate) putchar: PointerValue<'ctx>,
    pub(crate) getchar: PointerValue<'ctx>,
    pub(crate) array: PointerValue<'ctx>,
    pub(crate) array_ptr: PointerValue<'ctx>,
}

pub(crate) trait Generate<'ctx> {
    fn generate(&self, context: &'ctx Context, builder: &Builder<'ctx>, globals: &BfGlobals);
}

impl<'ctx> CodeGen<'ctx> {
    pub(crate) fn gen_bf(&self, ast: ASTBranch) -> () {
        // Create needed types
        let i32_type = self.context.i32_type();
        let void_type = self.context.void_type();

        let putchar_type = void_type.fn_type(&[i32_type.into()], false).ptr_type(AddressSpace::Global);
        let getchar_type = i32_type.fn_type(&[], false).ptr_type(AddressSpace::Global);
        let array_type = i32_type.array_type(BF_ARRAY_SIZE.try_into().unwrap()).ptr_type(AddressSpace::Global);

        // Type of the main function (contains everything)
        let fn_type = void_type.fn_type( 
            &[putchar_type.into(),
            getchar_type.into(),
            array_type.into()], 
            false
        );

        // Create the main function and set up the builder to build in it
        let function = self.module.add_function("bf", fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(basic_block);

        // Get the external functions from the parameters
        let putchar = function.get_nth_param(0).unwrap().into_pointer_value();
        let getchar = function.get_nth_param(1).unwrap().into_pointer_value();
        let array = function.get_nth_param(2).unwrap().into_pointer_value();
        
        // Create the array pointer and initialize it
        let array_pointer = self.builder.build_alloca(i32_type, "");
        self.builder.build_store(array_pointer, i32_type.const_int(0, false));

        // Generate code from the parsed BF code.
        let globals = BfGlobals{
            function, putchar, getchar, array, array_ptr: array_pointer
        };

        ast.generate(&self.context, &self.builder, &globals);

        self.builder.build_return(None);

        self.module.print_to_file("module.ir");
    }
}