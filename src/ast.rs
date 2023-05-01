use std::collections::BTreeMap;

use ordered_float::OrderedFloat;

use crate::lexer::Token;
use crate::operators::Operator;
use crate::visitors::interpreter::Interpreter;

macro_rules! define_ast {
    (pub enum $root:ident { $($n:ident: $t:ident $b:tt),* $(,)? }) => {
        #[derive(Clone, PartialEq, Debug, Hash)]
        pub enum $root {
            $($n($n)),*
        }

        $(
            #[derive(Clone, PartialEq, Debug, Hash)]
            pub $t $n $b
        )*
    }
}

define_ast!(
    pub enum Expr {
        Literal: enum {
            Bool(bool),
            Number(OrderedFloat<f64>),
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
            pub body: Vec<Stmt>,
            pub context: usize,
        },
        Instance: struct {
            pub class: Box<Class>,
            pub fields: BTreeMap<String, Expr>,
        },
        Class: struct {
            pub name: String,
            pub methods: BTreeMap<String, Function>,
        },
        Get: struct {
            pub object: Box<Expr>,
            pub name: Token,
        },
        Set: struct {
            pub object: Box<Expr>,
            pub name: Token,
            pub value: Box<Expr>,
        }

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
        Return: struct {
            pub keyword: Token,
            pub value: Expr,
        },
        ClassDecl: struct {
            pub name: Token,
            pub methods: Vec<Stmt>,
        }
    }
);

impl Stmt {
    pub fn accept<T: IVisitorStmt<U>, U>(&self, visitor: &mut T) -> U {
        match self {
            Stmt::Expression(_) => visitor.visit_expr(&self),
            Stmt::Print(_) => visitor.visit_print(&self),
            Stmt::VarDecl(_) => visitor.visit_var_decl(&self),
            Stmt::Block(_) => visitor.visit_block(&self),
            Stmt::If(_) => visitor.visit_if(&self),
            Stmt::While(_) => visitor.visit_while(&self),
            Stmt::FunDecl(_) => visitor.visit_fun_decl(&self),
            Stmt::Return(_) => visitor.visit_return(&self),
            Stmt::ClassDecl(_) => visitor.visit_class(&self),
        }
    }
}

impl Expr {
    pub fn accept<T: IVisitorExpr<U>, U>(&self, visitor: &mut T) -> U {
        match self {
            Expr::Var(_) => visitor.visit_var(&self),
            Expr::Literal(_) => visitor.visit_literal(&self),
            Expr::Unary(_) => visitor.visit_unary(&self),
            Expr::Binary(_) => visitor.visit_binary(&self),
            Expr::Grouping(_) => visitor.visit_grouping(&self),
            Expr::Assign(_) => visitor.visit_assign(&self),
            Expr::Logical(_) => visitor.visit_logical(&self),
            Expr::Call(_) => visitor.visit_call(&self),
            Expr::Get(_) => visitor.visit_get(&self),
            Expr::Set(_) => visitor.visit_set(&self),
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
                context: 0,
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
    pub fn execute_call(self, interpreter: &mut Interpreter, args: Vec<Expr>) -> Expr {
        let Function {
            name: _,
            parameters,
            body,
            context,
        } = self;

        interpreter.new_environment_with_parent(context);

        for (i, arg) in args.into_iter().enumerate() {
            let Var::Token(token) = parameters.get(i).unwrap();
            interpreter.define_symbol(token.lexeme.as_str(), arg);
        }
        //TODO globals here

        let res: Option<Stmt> = interpreter
            .execute_block(&body, interpreter.get_env_number())
            .unwrap();

        interpreter.drop_environment();
        match res {
            Some(Stmt::Return(Return { keyword: _, value })) => value,
            _ => Expr::Literal(Literal::Nil),
        }
    }

    pub fn from_stmt(value: crate::ast::Stmt, context: usize) -> Function {
        match value {
            Stmt::FunDecl(fun_decl) => Function {
                name: fun_decl.name,
                parameters: fun_decl
                    .parameters
                    .into_iter()
                    .map(|x| crate::ast::Var::Token(x))
                    .collect(),
                body: fun_decl.body,
                context,
            },
            _ => panic!("Expected function"),
        }
    }

    pub fn arity(&self) -> usize {
        self.parameters.len()
    }
}

impl Class {
    pub fn execute_call(self) -> Expr {
        let instance = Expr::Instance(Instance {
            class: Box::new(self),
            fields: BTreeMap::new(),
        });
        instance
    }

    pub fn find_method(&self, name: &str) -> Result<Option<Function>, Error> {
        Ok(self.methods.get(name).cloned())
    }

    pub fn arity(&self) -> usize {
        self.methods.len()
    }
}

impl Instance {
    pub fn get_field(&self, name: &str) -> Result<Expr, Error> {
        match self.fields.get(name) {
            Some(expr) => Ok(expr.clone()),
            None => {
                let method = self.class.find_method(name);
                match method {
                    Ok(Some(method)) => Ok(Expr::Function(method)),
                    _ => Err(Error {
                        msg: format!("Undefined property '{:?}'.", name),
                    }),
                }
            }
        }
    }

    pub fn set_field(&mut self, name: &str, value: Expr) {
        self.fields.insert(name.to_string(), value);
    }
}

pub trait IVisitorExpr<T> {
    fn visit_var(&mut self, expr: &Expr) -> T;
    fn visit_literal(&mut self, expr: &Expr) -> T;
    fn visit_unary(&mut self, expr: &Expr) -> T;
    fn visit_binary(&mut self, expr: &Expr) -> T;
    fn visit_grouping(&mut self, expr: &Expr) -> T;
    fn visit_assign(&mut self, expr: &Expr) -> T;
    fn visit_logical(&mut self, expr: &Expr) -> T;
    fn visit_call(&mut self, expr: &Expr) -> T;
    fn visit_get(&mut self, expr: &Expr) -> T;
    fn visit_set(&mut self, expr: &Expr) -> T;
}

pub trait IVisitorStmt<T> {
    fn visit_expr(&mut self, stmt: &Stmt) -> T;
    fn visit_print(&mut self, stmt: &Stmt) -> T;
    fn visit_var_decl(&mut self, stmt: &Stmt) -> T;
    fn visit_block(&mut self, stmt: &Stmt) -> T;
    fn visit_if(&mut self, stmt: &Stmt) -> T;
    fn visit_while(&mut self, stmt: &Stmt) -> T;
    fn visit_fun_decl(&mut self, stmt: &Stmt) -> T;
    fn visit_return(&mut self, stmt: &Stmt) -> T;
    fn visit_class(&mut self, stmt: &Stmt) -> T;
}

#[derive(Debug)]
pub struct Error {
    pub msg: String,
}

impl Error {
    pub fn new(msg: String) -> Self {
        Error { msg }
    }
}
