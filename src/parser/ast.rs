// use crate::lexer::Token;

#[derive(Debug)]
pub struct ProtoType {
    pub func_name: String,
    pub args: Vec<String>,
}

impl ProtoType {
    pub fn new(func_name: String, args: Vec<String>) -> Self {
        ProtoType { func_name, args }
    }
}

#[derive(Debug)]
pub enum AST {
    ExternNode(ProtoType),
}
// #[derive(PartialEq, Clone, Debug)]
// pub enum ASTNode {
//     ExternNode(Prototype),
//     FunctionNode(Function),
// }

// #[derive(PartialEq, Clone, Debug)]
// pub struct Function {
//     pub prototype: Prototype,
//     pub body: Expression,
// }

// #[derive(PartialEq, Clone, Debug)]
// pub struct Prototype {
//     pub name: String,
//     pub args: Vec<String>,
// }

// #[derive(PartialEq, Clone, Debug)]
// pub enum Expression {
//     LiteralEpxr(f64),
//     VariableExpr(String),
//     BinaryExpr(String, Box<Expression>, Box<Expression>),
//     CallExpr(String, Vec<Expression>),
// }

// pub enum PartParsingResult<T> {
//     Good(T, Vec<Token>),
//     NotComplete,
//     Bad(String),
// }

// pub fn error<T>(message: &str) -> PartParsingResult<T> {
//     PartParsingResult::Bad(message.to_string())
// }

// pub type ParsingResult = Result<(Vec<ASTNode>, Vec<Token>), String>;
