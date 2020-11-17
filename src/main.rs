use std::error::Error;

mod parser;
mod compiler;


fn main() -> Result<(), Box<dyn Error>> {
    let program = ">++++++++[<+++++++++>-]<.>++++[<+++++++>-]<+.+++++++..+++.>>++++++[<+++++++>-]<++.------------.>++++++[<+++++++++>-]<+.<.+++.------.--------.>>>++++[<++++++++>-]<+.";
    let (_remainder, parsed) = parser::parse::parse_bf(program)?;

    compiler::jit::compile(parsed)?;

    Ok(())
}
