use std::collections::HashMap;

use crate::lexer::Token;
use crate::operators::Operator;
use crate::visitors::interpreter::{self, Environment, Interpreter};

macro_rules! define_ast {
    (pub enum $root:ident { $($n:ident: $t:ident $b:tt),* $(,)? }) => {
        #[derive(Clone, PartialEq, Debug)]
        pub enum $root {
            $($n($n)),*
        }

        $(
            #[derive(Clone, PartialEq, Debug)]
            pub $t $n $b
        )*
    }
}

define_ast!(
    pub enum Expr {
        Literal: enum {
            Bool(bool),
            Number(f64),
            Str(String),
            Nil,
        },
        Var: enum {
            Token(Token)
        },
        Assign: struct {
            pub var: Var,
            pub expr: Box<Expr>
        },
        Grouping: struct {
            pub group: Box<Expr>
        },
        Unary: struct {
            pub operator: Operator,
            pub right: Box<Expr>,
        },
        Binary: struct {
            pub left: Box<Expr>,
            pub operator: Operator,
            pub right: Box<Expr>,
        },
        Logical: struct {
            pub left: Box<Expr>,
            pub operator: Operator,
            pub right: Box<Expr>,
        },
        Call: struct {
            pub callee: Box<Expr>,
            pub paren: Token,
            pub arguments: Vec<Expr>,
        },
        Function: struct {
            pub name: String,
            pub parameters: Vec<Var>,
            pub body: Vec<Stmt>
        },
    }
);

define_ast!(
    pub enum Stmt {
        Print: struct {
            pub expr: Box<Expr>
        },
        Expression: struct {
            pub expr: Box<Expr>
        },
        VarDecl: struct {
            pub name: String,
            pub expr: Box<Expr>
        },
        FunDecl: struct {
            pub name: String,
            pub parameters: Vec<Token>,
            pub body: Vec<Stmt>
        },
        Block: struct {
            pub stmts: Vec<Stmt>
        },
        If: struct {
            pub condition: Box<Expr>,
            pub branch_true: Box<Stmt>,
            pub branch_false: Box<Stmt>
        },
        While: struct {
            pub condition: Box<Expr>,
            pub body: Box<Stmt>
        },
    }
);

impl Stmt {
    pub fn accept<'a, T: IVisitorStmt<'a, U>, U>(&'a self, visitor: &mut T) -> U {
        match self {
            Stmt::Expression(_) => visitor.visit_expr(&self),
            Stmt::Print(_) => visitor.visit_print(&self),
            Stmt::VarDecl(_) => visitor.visit_var_decl(&self),
            Stmt::Block(_) => visitor.visit_block(&self),
            Stmt::If(_) => visitor.visit_if(&self),
            Stmt::While(_) => visitor.visit_while(&self),
            Stmt::FunDecl(_) => visitor.visit_fun_decl(&self),
        }
    }
}

//#[derive(Debug, Clone)]

impl Expr {
    pub fn accept<'a, T: IVisitorExpr<'a, U>, U>(&'a self, visitor: &mut T) -> U {
        match self {
            Expr::Var(_) => visitor.visit_var(&self),
            Expr::Literal(_) => visitor.visit_literal(&self),
            Expr::Unary(_) => visitor.visit_unary(&self),
            Expr::Binary(_) => visitor.visit_binary(&self),
            Expr::Grouping(_) => visitor.visit_grouping(&self),
            Expr::Assign(_) => visitor.visit_assign(&self),
            Expr::Logical(_) => visitor.visit_logical(&self),
            Expr::Call(_) => visitor.visit_call(&self),
            //Expr::Function(_) => visitor.call_function(&self),
            _ => panic!("Invalid expression"),
        }
    }
}

impl From<crate::ast::Stmt> for crate::ast::Function {
    fn from(value: crate::ast::Stmt) -> Function {
        match value {
            Stmt::FunDecl(fun_decl) => Function {
                name: fun_decl.name,
                parameters: fun_decl
                    .parameters
                    .into_iter()
                    .map(|x| crate::ast::Var::Token(x))
                    .collect(),
                body: fun_decl.body,
            },
            _ => panic!("Expected function"),
        }
    }
}

impl From<crate::ast::Token> for crate::ast::Literal {
    fn from(value: crate::ast::Token) -> Self {
        Literal::Str(value.lexeme)
    }
}

impl Function {
    pub fn execute_call(self, interpreter: &mut Interpreter, args: Vec<Expr>) {
        let Function {
            name: _,
            parameters,
            body,
        } = self;
        let symbol_table = HashMap::new();
        let mut env: Environment = Environment {
            enclosing: interpreter.env.enclosing.clone(),
            symbol_table,
        };

        for (i, arg) in args.into_iter().enumerate() {
            let Var::Token(token) = parameters.get(i).unwrap();
            env.define(token.lexeme.clone(), Box::new(arg));

            //globals here
        }
        interpreter.execute_block(&body, env)
    }
}

pub trait IVisitorExpr<'a, T> {
    fn visit_var(&mut self, expr: &'a Expr) -> T;
    fn visit_literal(&mut self, expr: &'a Expr) -> T;
    fn visit_unary(&mut self, expr: &'a Expr) -> T;
    fn visit_binary(&mut self, expr: &'a Expr) -> T;
    fn visit_grouping(&mut self, expr: &'a Expr) -> T;
    fn visit_assign(&mut self, expr: &'a Expr) -> T;
    fn visit_logical(&mut self, expr: &'a Expr) -> T;
    fn visit_call(&mut self, expr: &'a Expr) -> T;
}

pub trait IVisitorStmt<'a, T> {
    fn visit_expr(&mut self, stmt: &'a Stmt) -> T;
    fn visit_print(&mut self, stmt: &'a Stmt) -> T;
    fn visit_var_decl(&mut self, stmt: &'a Stmt) -> T;
    fn visit_block(&mut self, stmt: &'a Stmt) -> T;
    fn visit_if(&mut self, stmt: &'a Stmt) -> T;
    fn visit_while(&mut self, stmt: &'a Stmt) -> T;
    fn visit_fun_decl(&mut self, stmt: &'a Stmt) -> T;

    fn execute_block(&mut self, stmts: &Vec<Stmt>, env: Environment);
}
