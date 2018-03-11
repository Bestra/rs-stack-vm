use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub enum LogicalOperator {
    Or,
    And,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Star,
    Eq,
    BangEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
}

impl BinaryOperator {
    pub fn from_str(s: &str) -> BinaryOperator {
        use self::BinaryOperator::*;
        match s {
            "+" => Plus,
            "-" => Minus,
            "*" => Star,
            "==" => Eq,
            "!=" => BangEq,
            "<" => Lt,
            ">" => Gt,
            "<=" => LtEq,
            ">=" => GtEq,
            s => panic!("unable to create operator from {}", s)
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Assign {
        name: String,
        value: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        right: Box<Expr>,
        operator: BinaryOperator,
    },
    Call {
        callee: Box<Expr>,
        arguments: Vec<Expr>,
    },
    // Grouping {
    //     expression: Box<Expr>,
    // },
    Literal {
        value: usize,
    },
    Logical {
        left: Box<Expr>,
        right: Box<Expr>,
        operator: LogicalOperator,
    },
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
    Program {
        statements: Vec<Statement>,
    },

    Block {
        statements: Vec<Statement>,
    },
    Expression {
        expression: Box<Expr>,
    },
    // Function(FunctionDeclaration),
    If {
        condition: Box<Expr>,
        then_branch: Box<Statement>,
        else_branch: Option<Box<Statement>>,
    },
    Print {
        expression: Box<Expr>,
    },
    // Return {
    //     value: Option<Box<Expr>>,
    // },
    While {
        condition: Box<Expr>,
        body: Box<Statement>,
    },
    Var { // initializes a variable
        name: String,
        initializer: Option<Box<Expr>>,
    },
}

type GraphNode = (usize, String);
type GraphEdge = (GraphNode, GraphNode);
type GraphAccum = (Vec<GraphEdge>, usize);

pub fn print_ast(program: &Statement) -> Result<String, &str> {
    if let &Statement::Program { ref statements } = program {
        let (edges, _) = statements.iter().fold(
            (Vec::new(), 0),
            |acc, s| print_statement(acc, s, (0, "Program".to_string()))
        );

        let mut nodes: HashSet<String> = HashSet::new();
        for &((pi, ref pl),(ci, ref cl)) in edges.iter() {
            let parent = format!("{} [label=\"{}\"]", pi, pl);
            let child = format!("{} [label=\"{}\"]", ci, cl);
            nodes.insert(parent);
            nodes.insert(child);
        }

        let mut node_list: Vec<String> = nodes.into_iter().collect();
        let mut res: Vec<String> = edges.into_iter().map(|((pi, _),(ci, _))| format!("{}->{}", pi, ci)).collect();
        node_list.append(&mut res);
        let out = format!("digraph G {{ \n {} \n }}", node_list.join(";\n"));
        Ok(out)
    } else {
        Err("print_ast must take a program as its starting node")
    }
}

fn print_statement(accum: GraphAccum, node: &Statement, parent_node: GraphNode) -> GraphAccum {
    use ast::Statement::*;
    let (mut edges, parent_idx) = accum;

    let idx = parent_idx + 1;

    match node {
        &Program {
            statements: ref _statements,
        } => { panic!("shouldn't hit a program node")},

        &Block {
            ref statements,
        } => {

            edges.push((parent_node.clone(), (idx, "Block".to_string())));
            statements.iter().fold(
                (edges, idx),
                |acc, s| print_statement(acc, &s, (idx, "Block".to_string()))
            )
        },
        &Expression {
            ref expression,
        } => {

            edges.push((parent_node.clone(), (idx, "Expression".to_string())));
            print_expr((edges, idx), expression, (idx, "Expression".to_string()))
        },
        // // Function(FunctionDeclaration),
        &If {
            ref condition,
            ref then_branch,
            ref else_branch,
        } => {
            let p = (idx, "If".to_string());
            edges.push((parent_node.clone(), p.clone()));
            let (e1, i1) = print_expr((edges, idx), &*condition, p.clone());
            let (e2, i2) = print_statement((e1, i1), &*then_branch, p.clone());
            match else_branch {
                &Some(ref e) => print_statement((e2, i2), &*e, p.clone()),
                &None => (e2, i2),
            }
        },
        &Print {
            ref expression,
        } => {
            let p = (idx, "Print".to_string());
            edges.push((parent_node.clone(), p.clone()));
            print_expr((edges, idx), expression, p.clone())
        },
        // // Return {
        // //     value: Option<Box<Expr>>,
        // // },
        &While {
            ref condition,
            ref body,
        } => {
            let p = (idx, "While".to_string());
            edges.push((parent_node.clone(), p.clone()));
            let (e1, i1) = print_expr((edges, idx), condition, p.clone());
            print_statement((e1, i1), body, p.clone())
        },
        &Var { // initializes a variable
            ref name,
            ref initializer,
        } => {
            let p = (idx, format!("Var {}", name));
            edges.push((parent_node.clone(), p.clone()));
            match initializer {
                &Some(ref e) => print_expr((edges, idx), &*e, p.clone()),
                &None => (edges, idx),
            }
        },
    }
}

fn print_expr(accum: GraphAccum, node: &Expr, parent_node: GraphNode) -> GraphAccum {
    use ast::Expr::*;
    let (mut edges, parent_idx) = accum;
    let idx = parent_idx + 1;
    match node {
        &Assign {
            ref name,
            ref value,
        } => {
            let p = (idx, format!("Assign {}", name));
            edges.push((parent_node.clone(), p.clone()));
            print_expr((edges, idx), value, p.clone())
        },
        &Binary {
            ref left,
            ref right,
            ref operator,
        } => {
            let p = (idx, format!("Binary {:?}", operator));
            edges.push((parent_node.clone(), p.clone()));
            let (e1, i1) = print_expr((edges, idx), &*left, p.clone());
            print_expr((e1, i1), &*right, p.clone())
        },
        &Call {
            callee: ref _callee,
            arguments: ref _args,
        } => {
            let p = (idx, format!("Call"));
            edges.push((parent_node.clone(), p.clone()));
            (edges, idx)
        },
        // // Grouping {
        // //     expression: Box<Expr>,
        // // },
        &Literal {
            ref value,
        } => {
            edges.push((parent_node.clone(), (idx, format!("Value {}", value))));
            (edges, idx)
        },
        &Logical {
            ref left,
            ref right,
            ref operator,
        } => {
            let p = (idx, format!("Logical {:?}", operator));
            edges.push((parent_node.clone(), p.clone()));
            let (e1, i1) = print_expr((edges, idx), &*left, p.clone());
            print_expr((e1, i1), &*right, p.clone())
        },
        // // Unary {
        // //     right: Box<Expr>,
        // //     operator: String,
        // // },
        &Variable {
            ref name,
        } => {
            edges.push((parent_node.clone(), (idx, format!("Variable {}", name))));
            (edges, idx)
        },
    }
}

