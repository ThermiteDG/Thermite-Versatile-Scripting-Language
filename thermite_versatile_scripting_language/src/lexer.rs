// lexer.rs

// Import necessary items from nom
extern crate nom;
use nom::IResult;


// Import AST definitions from the ast module
use crate::ast::{BinaryOperator, ControlFlow, Expr, UnaryOperator};

pub enum GenericDataType<T> {
    BigInt(i128),
    LittleInt(i64),
    Double(f64),
    Identifier(String),
    
}

// Define different token types
#[derive(Debug, PartialEq)]
pub enum Token {
    Number(i64),
    Identifier(String),
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulus,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    LogicalAnd,
    LogicalOr,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LeftShift,
    RightShift,
    LParen,
    RParen,
    If,
    Else,
    While,
}

// Define the lexer for numbers
named!(lex_number<&str, Token>,
    map_res!(
        recognize!(
            pair!(
                opt!(char!('-')),
                alt!(
                    tag!("0") | preceded!(one_of!("123456789"), many0!(digit))
                )
            )
        ),
        |s: &str| s.parse::<i64>()
    )
);

// Define the lexer for identifiers
named!(lex_identifier<&str, Token>,
    map!(recognize!(alphanumeric1), |s: &str| Token::Identifier(String::from(s)))
);

// Define the lexer for operators
named!(lex_operator<&str, Token>,
    alt!(
        // ... (omitted for brevity, see previous code)
    )
);

// Define the lexer for tokens by combining number, identifier, and operator lexers
named!(lex_token<&str, Token>,
    alt!(lex_number | lex_identifier | lex_operator)
);

// Define the parser for expressions as a sequence of tokens and build the AST
named!(pub parse_expr<&str, Vec<Expr>>,
    many0!(
        alt!(
            map!(lex_number, |n| Expr::Literal(n)) |
            map!(lex_identifier, |id| Expr::Identifier(match id {
                Token::Identifier(s) => s,
                _ => panic!("Expected identifier token"),
            })) |
            map!(lex_operator, |op| Expr::BinaryOperation {
                left: Box::new(Expr::Literal(0)),
                operator: operator_from_token(&op),
                right: Box::new(Expr::Literal(0)),
            }) |
            map!(lex_operator, |op| Expr::UnaryOperation {
                operator: unary_operator_from_token(&op),
                operand: Box::new(Expr::Literal(0)),
            }) |
            map!(lex_operator, |op| Expr::ControlFlow(match op {
                Token::If => ControlFlow::If {
                    condition: Box::new(Expr::Literal(0)),
                    body: vec![], // Dummy body
                },
                Token::Else => panic!("Unexpected 'else' token outside of 'if' statement"),
                Token::While => ControlFlow::While {
                    condition: Box::new(Expr::Literal(0)),
                    body: vec![], // Dummy body
                },
                _ => panic!("Unexpected token as control flow statement"),
            }))
        )
    )
);

// Helper functions to convert tokens to AST enums
fn operator_from_token(token: &Token) -> BinaryOperator {
    // ...
}

fn unary_operator_from_token(token: &Token) -> UnaryOperator {
    // ...
}
