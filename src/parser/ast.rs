// The primitive BF operations
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Operation {
    Read,
    Write,
    Increment,
    Decrement,
    PtrLeft,
    PtrRight
}

// Contains an ordered list of child elements which can be operations or loops
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ASTBranch (pub Vec<ASTElement>);


#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ASTElement {
    Loop(ASTBranch),
    Operation(Operation)
}
