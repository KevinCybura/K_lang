use crate::lexer::Token;
use crate::lexer::Token::*;
use std::collections::HashMap;

#[derive(PartialEq, Clone, Debug)]
pub enum ASTNode {
    ExternNode(Prototype),
    FunctionNode(Function),
}
use ASTNode::*;

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
use Expression::*;

pub type ParsingResult = Result<(Vec<ASTNode>, Vec<Token>), String>;

enum PartParsingResult<T> {
    Good(T, Vec<Token>),
    NotComplete,
    Bad(String),
}
use PartParsingResult::*;

fn error<T>(message: &str) -> PartParsingResult<T> {
    use PartParsingResult::*;
    Bad(message.to_string())
}

pub struct ParserSettings {
    operator_precedence: HashMap<String, i32>,
}

pub fn default_parser_settings() -> ParserSettings {
    let mut operator_precedence = HashMap::new();
    operator_precedence.insert("<".to_string(), 10);
    operator_precedence.insert("+".to_string(), 20);
    operator_precedence.insert("-".to_string(), 20);
    operator_precedence.insert("*".to_string(), 40);
    ParserSettings {
        operator_precedence: operator_precedence,
    }
}

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

fn parse_extern(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
) -> PartParsingResult<ASTNode> {
    tokens.pop();
    let mut parsed_tokens = vec![Token::Extern];
    let prototype = parse_try!(parse_prototype, tokens, settings, parsed_tokens);
    Good(ExternNode(prototype), parsed_tokens)
}

fn parse_function(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
) -> PartParsingResult<ASTNode> {
    tokens.pop();
    let mut parsed_tokens = vec![Token::Def];
    let prototype = parse_try!(parse_prototype, tokens, settings, parsed_tokens);
    let body = parse_try!(parse_expr, tokens, settings, parsed_tokens);

    Good(FunctionNode(Function { prototype, body }), parsed_tokens)
}

fn parse_prototype(
    tokens: &mut Vec<Token>,
    _settings: &mut ParserSettings,
) -> PartParsingResult<Prototype> {
    let mut parsed_tokens = Vec::new();

    let name = expect_token!(
        [Ident(name), Ident(name.clone()), name] <= tokens,
        parsed_tokens,
        "expected function name in prototype"
    );

    expect_token!(
        [OpeningParenthesis, OpeningParenthesis, ()] <= tokens,
        parsed_tokens,
        "expected '(' in prototype"
    );

    let mut args = Vec::new();
    loop {
        expect_token!([ Ident(arg), Ident(arg.clone()), args.push(arg.clone());
                      Comma, Comma, continue;
                      ClosingParenthesis, ClosingParenthesis, break
        ] <= tokens, parsed_tokens, "expected ')' in prototype");
    }

    Good(Prototype { name, args }, parsed_tokens)
}

fn parse_expression(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
) -> PartParsingResult<ASTNode> {
    let mut parsed_tokens = Vec::new();
    let expression = parse_try!(parse_expr, tokens, settings, parsed_tokens);
    let prototype = Prototype {
        name: "".to_string(),
        args: vec![],
    };
    let lambda = Function {
        prototype,
        body: expression,
    };
    Good(FunctionNode(lambda), parsed_tokens)
}

fn parse_primary_expr(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
) -> PartParsingResult<Expression> {
    match tokens.last() {
        Some(&Ident(_)) => parse_ident_expr(tokens, settings),
        Some(&Numeric(_)) => parse_literal_expr(tokens, settings),
        Some(&OpeningParenthesis) => parse_parenthesis_expr(tokens, settings),
        None => return NotComplete,
        _ => error("unknow token when expecting an expression"),
    }
}

fn parse_ident_expr(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
) -> PartParsingResult<Expression> {
    let mut parsed_tokens = Vec::new();

    let name = expect_token!(
        [Ident(name), Ident(name.clone()), name] <= tokens,
        parsed_tokens,
        "identifier expect"
    );

    expect_token! (
        [ OpeningParenthesis, OpeningParenthesis, () ]
        else { return Good(VariableExpr(name), parsed_tokens) }
        <= tokens, parsed_tokens);

    let mut args = Vec::new();
    loop {
        expect_token!(
        [ ClosingParenthesis, ClosingParenthesis, break;
          Comma, Comma, continue]
          else { args.push(parse_try!(parse_expr, tokens, settings, parsed_tokens)); }
          <= tokens, parsed_tokens
        );
    }

    Good(CallExpr(name, args), parsed_tokens)
}

fn parse_literal_expr(
    tokens: &mut Vec<Token>,
    _settings: &mut ParserSettings,
) -> PartParsingResult<Expression> {
    let mut parsed_tokens = Vec::new();

    let value = expect_token!(
        [Numeric(val), Numeric(val), val] <= tokens,
        parsed_tokens,
        "Literal expected"
    );

    Good(LiteralEpxr(value), parsed_tokens)
}

fn parse_parenthesis_expr(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
) -> PartParsingResult<Expression> {
    tokens.pop();
    let mut parsed_tokens = vec![OpeningParenthesis];

    let expr = parse_try!(parse_expr, tokens, settings, parsed_tokens);

    expect_token!(
        [ClosingParenthesis, ClosingParenthesis, ()] <= tokens,
        parsed_tokens,
        "')' expected"
    );

    Good(expr, parsed_tokens)
}

fn parse_expr(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
) -> PartParsingResult<Expression> {
    let mut parsed_tokens = Vec::new();
    let lhs = parse_try!(parse_primary_expr, tokens, settings, parsed_tokens);
    let expr = parse_try!(parse_binary_expr, tokens, settings, parsed_tokens, 0, &lhs);
    Good(expr, parsed_tokens)
}

fn parse_binary_expr(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    expr_precedence: i32,
    lhs: &Expression,
) -> PartParsingResult<Expression> {
    // start with LHS value
    let mut result = lhs.clone();
    let mut parsed_tokens = Vec::new();
    loop {
        let (operator, precedence) = match tokens.last() {
            Some(Operator(operator)) => match settings.operator_precedence.get(operator) {
                Some(pr) if *pr >= expr_precedence => (operator.clone(), *pr),
                None => return error("Unknown operator found"),
                _ => break,
            },
            _ => break,
        };

        tokens.pop();
        parsed_tokens.push(Operator(operator.clone()));

        let mut rhs = parse_try!(parse_primary_expr, tokens, settings, parsed_tokens);

        loop {
            let binary_rhs = match tokens.last().map(|i| i.clone()) {
                Some(Operator(ref op)) => match settings.operator_precedence.get(op).map(|i| *i) {
                    Some(pr) if pr > precedence => {
                        parse_try!(parse_binary_expr, tokens, settings, parsed_tokens, pr, &rhs)
                    }
                    None => return error("Unkown operator found"),
                    _ => break,
                },
                _ => break,
            };
            rhs = binary_rhs;
        }
        result = BinaryExpr(operator, Box::new(result), Box::new(rhs));
    }
    Good(result, parsed_tokens)
}
