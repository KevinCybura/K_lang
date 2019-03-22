use crate::parser::Token;

#[derive(PartialEq, Clone, Debug)]
pub enum ASTNode {
    ExternNode(Prototype),
    FunctionNode(Function),
}

#[derive(PartialEq, Clone, Debug)]
pub struct Function {
    pub prototype: Prototype,
    pub body: Expression,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Prototype {
    pub name: String,
    pub args: Vec<String>,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Expression {
    LiteralEpxr(f64),
    VariableExpr(String),
    BinaryExpr(String, Box<Expression>, Box<Expression>),
    CallExpr(String, Vec<Expression>),
}

pub type ParsingResult = Result<(Vec<ASTNode>, Vec<Token>), String>;

enum PartParsingResult<T> {
    Good(T, Vec<Token>),
    NotComplete,
    Bad(String),
}

fn error<T>(message: &str) -> PartParsingResult<T> {
    use PartParsingResult::*;
    Bad(message.to_string())
}

enum ParserSettings {}

pub fn parse(
    tokens: &[Token],
    parsed_tree: &[ASTNode],
    settings: &mut ParserSettings,
) -> ParsingResult {
    use PartParsingResult::*;
    let mut rest = tokens.to_vec();
    // use the vector as a stack.
    rest.reverse();

    let mut ast = parsed_tree.to_vec();

    while let Some(cur_token) = rest.last() {
        let result = match cur_token {
            Def => parse_function(&mut rest, settings),
            Extern => parse_extern(&mut rest, settings),
            Delimiter => {
                rest.pop();
                continue;
            }
            _ => parse_expression(&mut rest, settings),
        };

        match result {
            Good(ast_node, _) => ast.push(ast_node),
            NotComplete => break,
            Bad(message) => return Err(message),
        }
    }

    rest.reverse();
    Ok((ast, rest))
}
