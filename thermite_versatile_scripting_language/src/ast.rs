// ast.rs

// Define different types of binary operators
#[derive(Debug)]
pub enum BinaryOperator {
    Add,
    Subtract,
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
}

// Define different types of unary operators
#[derive(Debug)]
pub enum UnaryOperator {
    Negate,
}

// Define control flow structures (e.g., If and While statements)
#[derive(Debug)]
pub enum ControlFlow {
    If {
        condition: Box<Expr>,
        body: Vec<Expr>,
    },
    While {
        condition: Box<Expr>,
        body: Vec<Expr>,
    },
}

// Define the main expression types
#[derive(Debug)]
pub enum Expr {
    Literal(i32),
    Identifier(String),
    BinaryOperation {
        left: Box<Expr>,
        operator: BinaryOperator,
        right: Box<Expr>,
    },
    UnaryOperation {
        operator: UnaryOperator,
        operand: Box<Expr>,
    },
    ControlFlow(ControlFlow),
}
