use nom::{IResult, branch::alt, character::complete::char, character::complete::none_of, combinator::map, combinator::{cut, value}, multi::{many0, many1}, sequence::{preceded, terminated}};

use super::ast::*;

// Consumes a BF operation (not a loop)
fn operation<'a>(input: &'a str) -> IResult<&'a str, Operation> {
    let ptr_left = value(Operation::PtrLeft, char('<'));
    let ptr_right = value(Operation::PtrRight, char('>'));
    let inc = value(Operation::Increment, char('+'));
    let dec = value(Operation::Decrement, char('-'));
    let read = value(Operation::Read, char(','));
    let write = value(Operation::Write, char('.'));

    alt((ptr_left, ptr_right, inc, dec, read, write))(input)
}

// Consumes the next valid element (loop or operation). If there are invalid (comment) characters first, they are discarded.
const VALID_CHARS: &str = ",.<>+-[]";
fn ast_element<'a>(input: &'a str) -> IResult<&'a str, ASTElement> {
    let op_or_loop = alt((
        map(operation, ASTElement::Operation),
        map(ast_loop, ASTElement::Loop)
    ));

    preceded(many0(none_of(VALID_CHARS)), op_or_loop)(input)
}

// Consumes a loop (delimited by [])
fn ast_loop<'a>(input: &'a str) -> IResult<&'a str, ASTBranch> {
    let left_delimiter = char('[');
    // There can be whitespace/comments before a loop closing
    let right_delimiter = preceded(many0(none_of(VALID_CHARS)), char(']'));

    preceded(left_delimiter, terminated(ast_branch, right_delimiter))(input)
}

// Consumes a series of operations and loops. This generates the entire AST of the input.
fn ast_branch<'a>(input: &'a str) -> IResult<&'a str, ASTBranch> {
    cut(map(many1(ast_element), ASTBranch))(input)
}

// Consumes a series of operations and loops. This is the top level parser.
pub fn parse_bf<'a>(input: &'a str) -> IResult<&'a str, ASTBranch> {
    cut(map(many0(ast_element), ASTBranch))(input)
}

#[cfg(test)]
mod tests {
    use nom::error::{ErrorKind};

    use crate::parser::ast::{Operation, ASTElement, ASTBranch};

    use super::{ast_branch, ast_element, ast_loop, operation, parse_bf};

    #[test]
    fn parse_operators() {
        assert_eq!(operation("<<"), Ok(( "<", Operation::PtrLeft)));
        assert_eq!(operation("><"), Ok(( "<", Operation::PtrRight)));
        assert_eq!(operation("+<"), Ok(( "<", Operation::Increment)));
        assert_eq!(operation("-<"), Ok(( "<", Operation::Decrement)));
        assert_eq!(operation(",<"), Ok(( "<", Operation::Read)));
        assert_eq!(operation(".<"), Ok(( "<", Operation::Write)));
        assert_eq!(operation("a"), Err(nom::Err::Error(nom::error::Error { input: "a", code: ErrorKind::Char })));
    }

    #[test]
    fn parse_loop() {
        assert_eq!(ast_loop("[+]-"), Ok(("-", ASTBranch(vec![ASTElement::Operation(Operation::Increment)]))));
        // Comments don't matter
        assert_eq!(ast_loop("[a+b]-"), Ok(("-", ASTBranch(vec![ASTElement::Operation(Operation::Increment)]))));
        ast_loop("input").expect_err("Should have produced an error");
        ast_loop("[]").expect_err("Empty loops are not allowed");
        ast_loop("[+++").expect_err("Unmatched loops are not allowed");
    }

    #[test]
    fn parse_element() {
        assert_eq!(ast_element("foo++"), Ok(("+", ASTElement::Operation(Operation::Increment))));
        assert_eq!(ast_element("foo[+]-"), Ok(("-", ASTElement::Loop(ASTBranch(vec![ASTElement::Operation(Operation::Increment)])))));
        ast_element("foo").expect_err("no bf happening here...");
    }

    #[test]
    fn parse_ast() {
        // wow much ast
        assert_eq!(ast_branch("foo+[-]f"), Ok((
            "f", 
            ASTBranch(vec![
                ASTElement::Operation(Operation::Increment), 
                ASTElement::Loop(ASTBranch(vec![
                    ASTElement::Operation(Operation::Decrement)
                ]))
            ]))
        ));

        // Program and branch should be equal for non-empty programs
        assert_eq!(ast_branch("foo+[-]f"), parse_bf("foo+[-]f"));

        // Comments don't matter
        assert_eq!(parse_bf("foo+ffff[asdf- hello world]"), parse_bf("+[-]"));

        // Parse empty program
        assert_eq!(parse_bf("foo"), Ok(("foo", ASTBranch(vec![]))));
    }
}