#[derive(Debug, Clone)]
pub enum Expr {
    Assign {
        name: String,
        value: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        right: Box<Expr>,
        operator: String,
    },
    // Call {
    //     callee: Box<Expr>,
    //     arguments: Vec<Expr>,
    // },
    // Grouping {
    //     expression: Box<Expr>,
    // },
    Literal {
        value: usize,
    },
    // Logical {
    //     left: Box<Expr>,
    //     right: Box<Expr>,
    //     operator: String,
    // },
    // Unary {
    //     right: Box<Expr>,
    //     operator: String,
    // },
    Variable {
        name: String,
    },
}

#[derive(Debug, Clone)]
pub enum Statement {
    Block {
        statements: Vec<Statement>,
    },
    Expression {
        expression: Box<Expr>,
    },
    // Function(FunctionDeclaration),
    // If {
    //     condition: Box<Expr>,
    //     then_branch: Box<Statement>,
    //     else_branch: Option<Box<Statement>>,
    // },
    Print {
        expression: Box<Expr>,
    },
    // Return {
    //     value: Option<Box<Expr>>,
    // },
    // While {
    //     condition: Box<Expr>,
    //     body: Box<Statement>,
    // },
    Var { // initializes a variable
        name: String,
        initializer: Option<Box<Expr>>,
    },
}

pub struct Program {
    pub statements: Vec<Statement>,
}
