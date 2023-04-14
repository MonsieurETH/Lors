use crate::lexer::{Token, TokenType};
use crate::parser::Type;
use crate::Interpreter;

#[derive(Debug, Clone)]
pub enum Operator {
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Minus,
    Plus,
    Slash,
    Star,
    Or,
    And,
}

impl Operator {
    pub fn from_token(token: &Token) -> Operator {
        match token.token_type {
            TokenType::Minus => Operator::Minus,
            TokenType::Plus => Operator::Plus,
            TokenType::Slash => Operator::Slash,
            TokenType::Star => Operator::Star,
            TokenType::Bang => Operator::Bang,
            TokenType::BangEqual => Operator::BangEqual,
            TokenType::Equal => Operator::Equal,
            TokenType::EqualEqual => Operator::EqualEqual,
            TokenType::Greater => Operator::Greater,
            TokenType::GreaterEqual => Operator::GreaterEqual,
            TokenType::Less => Operator::Less,
            TokenType::LessEqual => Operator::LessEqual,
            TokenType::Or => Operator::Or,
            TokenType::And => Operator::And,
            _ => panic!("Unknown binary operation"),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Operator::Minus => String::from("-"),
            Operator::Plus => String::from("+"),
            Operator::Slash => String::from("/"),
            Operator::Star => String::from("*"),
            Operator::Bang => String::from("!"),
            Operator::BangEqual => String::from("!="),
            Operator::Equal => String::from("="),
            Operator::EqualEqual => String::from("=="),
            Operator::Greater => String::from(">"),
            Operator::GreaterEqual => String::from(">="),
            Operator::Less => String::from("<"),
            Operator::LessEqual => String::from("<="),
            Operator::Or => String::from("or"),
            Operator::And => String::from("and"),
        }
    }

    pub fn unary(self, right: Expr) -> Expr {
        match self {
            Operator::Minus => self.minus(right),
            Operator::Bang => self.negation(right),
            _ => panic!("Unknown unary operation"),
        }
    }

    fn minus(self, expr: Expr) -> Expr {
        match expr {
            Expr::Value(Type::Number(n)) => Expr::Value(Type::Number(-n)),
            _ => panic!("Operand must be a number"),
        }
    }

    fn negation(self, expr: Expr) -> Expr {
        match expr {
            Expr::Value(Type::Bool(b)) => Expr::Value(Type::Bool(!b)),
            _ => panic!("Operand must be a bool"),
        }
    }

    pub fn binary(self, left: Expr, right: Expr) -> Expr {
        match self {
            Operator::Plus => self.addition(left, right),
            Operator::Minus => self.subtraction(left, right),
            Operator::Star => self.multiplication(left, right),
            Operator::Slash => self.division(left, right),
            Operator::EqualEqual => self.equal_equal(left, right),
            Operator::BangEqual => self.bang_equal(left, right),
            Operator::Greater => self.greater_than(left, right),
            Operator::GreaterEqual => self.greater_than_or_equal(left, right),
            Operator::Less => self.less_than(left, right),
            Operator::LessEqual => self.less_than_or_equal(left, right),
            _ => panic!("Unknown binary operator"),
        }
    }

    pub fn logical(self, left: Expr, right: Expr) -> Expr {
        match self {
            Operator::Or => self.logical_or(left, right),
            Operator::And => self.logical_and(left, right),
            _ => panic!("Unknown logical operator"),
        }
    }

    fn logical_or(self, left: Expr, right: Expr) -> Expr {
        left
    }

    fn logical_and(self, left: Expr, right: Expr) -> Expr {
        right
    }

    fn addition(self, left: Expr, right: Expr) -> Expr {
        match (left, right) {
            (Expr::Value(Type::Number(l)), Expr::Value(Type::Number(r))) => {
                Expr::Value(Type::Number(l + r))
            }
            (Expr::Value(Type::Str(l)), Expr::Value(Type::Str(r))) => {
                Expr::Value(Type::Str(l + &r))
            }
            _ => panic!("Operands must be two numbers or two strings"),
        }
    }

    fn subtraction(self, left: Expr, right: Expr) -> Expr {
        match (left, right) {
            (Expr::Value(Type::Number(l)), Expr::Value(Type::Number(r))) => {
                Expr::Value(Type::Number(l - r))
            }
            _ => panic!("Operands must be numbers"),
        }
    }

    fn multiplication(self, left: Expr, right: Expr) -> Expr {
        match (left, right) {
            (Expr::Value(Type::Number(l)), Expr::Value(Type::Number(r))) => {
                Expr::Value(Type::Number(l * r))
            }
            _ => panic!("Operands must be numbers"),
        }
    }

    fn division(self, left: Expr, right: Expr) -> Expr {
        match (left, right) {
            (Expr::Value(Type::Number(l)), Expr::Value(Type::Number(r))) => {
                Expr::Value(Type::Number(l / r))
            }
            _ => panic!("Operands must be numbers"),
        }
    }

    fn equal_equal(self, left: Expr, right: Expr) -> Expr {
        match (left, right) {
            (Expr::Value(Type::Bool(l)), Expr::Value(Type::Bool(r))) => {
                Expr::Value(Type::Bool(l == r))
            }
            (Expr::Value(Type::Number(l)), Expr::Value(Type::Number(r))) => {
                Expr::Value(Type::Bool(l == r))
            }
            (Expr::Value(Type::Str(l)), Expr::Value(Type::Str(r))) => {
                Expr::Value(Type::Bool(l == r))
            }
            _ => panic!("Operands must be of the same type"),
        }
    }

    fn bang_equal(self, left: Expr, right: Expr) -> Expr {
        match (left, right) {
            (Expr::Value(Type::Bool(l)), Expr::Value(Type::Bool(r))) => {
                Expr::Value(Type::Bool(l != r))
            }
            (Expr::Value(Type::Number(l)), Expr::Value(Type::Number(r))) => {
                Expr::Value(Type::Bool(l != r))
            }
            (Expr::Value(Type::Str(l)), Expr::Value(Type::Str(r))) => {
                Expr::Value(Type::Bool(l != r))
            }
            _ => panic!("Operands must be of the same type"),
        }
    }

    fn greater_than(self, left: Expr, right: Expr) -> Expr {
        match (left, right) {
            (Expr::Value(Type::Number(l)), Expr::Value(Type::Number(r))) => {
                Expr::Value(Type::Bool(l > r))
            }
            _ => panic!("Operands must be numbers"),
        }
    }

    fn greater_than_or_equal(self, left: Expr, right: Expr) -> Expr {
        match (left, right) {
            (Expr::Value(Type::Number(l)), Expr::Value(Type::Number(r))) => {
                Expr::Value(Type::Bool(l >= r))
            }
            _ => panic!("Operands must be numbers"),
        }
    }

    fn less_than(self, left: Expr, right: Expr) -> Expr {
        match (left, right) {
            (Expr::Value(Type::Number(l)), Expr::Value(Type::Number(r))) => {
                Expr::Value(Type::Bool(l < r))
            }
            _ => panic!("Operands must be numbers"),
        }
    }

    fn less_than_or_equal(self, left: Expr, right: Expr) -> Expr {
        match (left, right) {
            (Expr::Value(Type::Number(l)), Expr::Value(Type::Number(r))) => {
                Expr::Value(Type::Bool(l <= r))
            }
            _ => panic!("Operands must be numbers"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Print(Box<Expr>),
    Expression(Box<Expr>),
    VarDecl(String, Box<Expr>),
    Block(Vec<Stmt>),
    If(Box<Expr>, Box<Stmt>, Box<Stmt>),
    While(Box<Expr>, Box<Stmt>),
}

impl Stmt {
    pub fn accept<'a, T: IVisitorStmt<'a, U>, U>(&'a self, visitor: &mut T) -> U {
        match self {
            Stmt::Expression(_) => visitor.visit_stmt_expr(&self),
            Stmt::Print(_) => visitor.visit_stmt_print(&self),
            Stmt::VarDecl(_, _) => visitor.visit_var_decl(&self),
            Stmt::Block(_) => visitor.visit_block(&self),
            Stmt::If(_, _, _) => visitor.visit_if(&self),
            Stmt::While(_, _) => visitor.visit_while(&self),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Var(Box<Token>),
    Value(Type),
    Assign(Box<Token>, Box<Expr>),
    Grouping(Box<Expr>),
    Unary {
        operator: Operator,
        right: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Operator,
        right: Box<Expr>,
    },
    Logical {
        left: Box<Expr>,
        operator: Operator,
        right: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        paren: Box<Token>,
        arguments: Vec<Expr>,
    },
}

impl Expr {
    pub fn accept<'a, T: IVisitorExpr<'a, U>, U>(&'a self, visitor: &mut T) -> U {
        match self {
            Expr::Var(_) => visitor.visit_var(&self),
            Expr::Value(_) => visitor.visit_value(&self),
            Expr::Unary {
                operator: _,
                right: _,
            } => visitor.visit_unary(&self),
            Expr::Binary {
                left: _,
                operator: _,
                right: _,
            } => visitor.visit_binary(&self),
            Expr::Grouping(_) => visitor.visit_grouping(&self),
            Expr::Assign(_, _) => visitor.visit_assign(&self),
            Expr::Logical {
                left: _,
                operator: _,
                right: _,
            } => visitor.visit_logical(&self),
            Expr::Call {
                callee,
                paren,
                arguments,
            } => visitor.visit_call(&self),
        }
    }
}

pub trait Callable {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Expr>) -> Expr;
    //fn name(&self) -> &str;
    //fn bind(&self, instance: &Instance) -> Self;
    //fn is_method(&self) -> bool;
    //fn is_initializer(&self) -> bool;
}

pub trait IVisitorExpr<'a, T> {
    fn visit_var(&mut self, expr: &'a Expr) -> T;
    fn visit_value(&mut self, expr: &'a Expr) -> T;
    fn visit_unary(&mut self, expr: &'a Expr) -> T;
    fn visit_binary(&mut self, expr: &'a Expr) -> T;
    fn visit_grouping(&mut self, expr: &'a Expr) -> T;
    fn visit_assign(&mut self, expr: &'a Expr) -> T;
    fn visit_logical(&mut self, expr: &'a Expr) -> T;
    fn visit_call(&mut self, expr: &'a Expr) -> T;
}

pub trait IVisitorStmt<'a, T> {
    fn visit_stmt_expr(&mut self, stmt: &'a Stmt) -> T;
    fn visit_stmt_print(&mut self, stmt: &'a Stmt) -> T;
    fn visit_var_decl(&mut self, stmt: &'a Stmt) -> T;
    fn visit_block(&mut self, stmt: &'a Stmt) -> T;
    fn visit_if(&mut self, stmt: &'a Stmt) -> T;
    fn visit_while(&mut self, stmt: &'a Stmt) -> T;

    fn execute_block(&mut self, stmts: &Vec<Stmt>);
}
