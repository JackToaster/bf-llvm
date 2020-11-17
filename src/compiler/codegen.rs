use std::io::{self, Read, Write};

use inkwell::{AddressSpace, builder::Builder, types::PointerType, values::PointerValue};
use inkwell::context::Context;
use inkwell::module::Module;




extern "C" fn bf_putchar(x: i32) -> () {
    print!("{}", x as u8 as char);
    io::stdout().flush();
}

extern "C" fn bf_getchar() -> i32 {
    let mut input = std::io::stdin();
    let mut buf = [0u8];

    if input.read(&mut buf).unwrap() == 0 {
        return 0xFF; // EOF
    }

    buf[0] as i32
}

const BF_ARRAY_SIZE: u32 = 30000;

pub(crate) struct CodeGen<'ctx> {
    pub(crate) context: &'ctx Context,
    pub(crate) module: Module<'ctx>,
    pub(crate) builder: Builder<'ctx>,
}

pub(crate) struct BfGlobals<'ctx> {
    pub(crate) putchar: PointerValue<'ctx>,
    pub(crate) getchar: PointerValue<'ctx>,
    pub(crate) array: PointerValue<'ctx>,
    pub(crate) array_ptr: PointerValue<'ctx>,
}

pub(crate) trait Generate<'ctx> {
    fn generate(&self, context: &'ctx Context, builder: &Builder<'ctx>, globals: &BfGlobals);
}

impl<'ctx> CodeGen<'ctx> {
    pub(crate) fn gen_test(&self) -> () {
        let i32_type = self.context.i32_type();

        let void_type = self.context.void_type();

        let putchar_type = void_type.fn_type(&[i32_type.into()], false).ptr_type(AddressSpace::Global);
        let getchar_type = i32_type.fn_type(&[], false).ptr_type(AddressSpace::Global);
        let array_type = i32_type.array_type(BF_ARRAY_SIZE).ptr_type(AddressSpace::Global);

        let fn_type = void_type.fn_type( 
            &[putchar_type.into(),
            getchar_type.into(),
            array_type.into()], 
            false
        );

        let function = self.module.add_function("bf", fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(basic_block);

        let putchar = function.get_nth_param(0).unwrap().into_pointer_value();
        let getchar = function.get_nth_param(1).unwrap().into_pointer_value();
        let array = function.get_nth_param(2).unwrap().into_pointer_value();
        
        let array_pointer = self.builder.build_alloca(i32_type, "");
        self.builder.build_store(array_pointer, i32_type.const_int(0, false));


        self.builder.build_return(None);
    }
}