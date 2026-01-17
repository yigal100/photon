pub struct Ast {
    // AST fields
}

impl Ast {
    pub fn new() -> Self {
        Self {
            // Initialize fields
        }
    }
}

enum Expr {
    Int(i32),
    Bool(bool),

    Var(String),
    Let {
        name: String,
        val: Box<Expr>,
        body: Box<Expr>,
    },

    If {
        cond: Box<Expr>,
        then_: Box<Expr>,
        else_: Box<Expr>,
    },

    Call {
        func: String,
        arg: Box<Expr>,
    },

    Alloc {
        layout: StructLayout,
    },
    Load {
        base: Box<Expr>,
        offset: u32,
    },
    Store {
        base: Box<Expr>,
        offset: u32,
        val: Box<Expr>,
    },

    // Staging
    Quote(Box<Expr>),
    Splice(Box<Expr>),
    Lift(Box<Expr>),
}

use crate::semantic::StructLayout;
