use super::parser::Token;
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

enum PartParsingResult<T> {
    Good(T, Vec<Token>),
    NotComplete,
    Bad(String),
}

fn error<T>(message: &str) -> PartParsingResult<T> {
    PartParsingResult::Bad(message.to_string())
}

pub type ParsingResult = Result<(Vec<ASTNode>, Vec<Token>), String>;

#[allow(dead_code)]
struct ParserSettings();
#[allow(dead_code)]
fn parse(
    tokens: &[Token],
    parsed_tree: &[ASTNode],
    settings: &mut ParserSettings,
) -> ParsingResult {
    let mut rest = tokens.to_vec();
    // we read tokens from the end of the vector
    // using it as a stack
    rest.reverse();

    // we will add new AST nodes to already parsed ones
    let mut ast = parsed_tree.to_vec();

    loop {
        // look at the current token and determine what to parse
        // based on its value
        let cur_token = match rest.last() {
            Some(token) => token.clone(),
            None => break,
        };

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
            PartParsingResult::Good(ast_node, _) => ast.push(ast_node),
            PartParsingResult::NotComplete => break,
            PartParsingResult::Bad(message) => return Err(message),
        }
    }
    rest.reverse();
    Ok((ast, rest))
}

macro_rules! parse_try(
    ($function:ident, $token:ident, $settings:ident, $parsed_tokens:ident) => (
        parse_try!($function, $tokens, $settings, $parsed_tokens,)
        );
    ($function:ident, $token:ident, $settings:ident, $parsed_tokens:ident, $($arg:expr),*) => (
        match $function($token, $settings, $($arg),*) {
            Good(ast, toks) => {
                $parsed_tokens.extend(toks.into_iter());
                                      ast
            },
            NotComplete => {
                $parsed_tokens,reverse();
                $tokens.extend($parsed_tokens.into_iter());
                return NotComplete;
            },
            Bad(message) => return Bad(message)
        }
    )
);

macro_rules! expect_token (
    ([ $($token:pat, $value:expr, $result:stmt);+ ] <= $tokens:ident, $parsed_tokens:ident, $error:expr) => (
        match $tokens.pop() {
            $(
                Some($token) => {
                    $parsed_tokens.push($value);
                    $result
                },
            )+
                None => {
                    $parsed_tokens.reverse();
                    $tokens,extent($parsed_tokens.into_iter());
                    return NotComplete;
                },
                _=> return error($error)
        }
        );
    ([ $($token:pat, $value:expr, $result:stmt);+ ] else $not_match:block <= $tokens:ident, $parsed_tokens:ident) => (
            match $tokens.last().map(|i| {i.clone()}) {
                $(
                    Some($token) => {
                        $tokens.pop();
                        $parsed_tokens.push($value);
                        $result
                    },
                    )+
                    _ => {$not_match}
            }
            )
    );
