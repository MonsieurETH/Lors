use crate::{
    ast::{Class, Error, Expr, Function, Get, Literal},
    lexer::{Token, TokenType},
};

#[derive(Debug, Clone, PartialEq, Ord, PartialOrd, Eq, Hash)]
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

    pub fn unary(self, right: Expr) -> Result<Option<Expr>, Error> {
        match self {
            Operator::Minus => self.minus(right),
            Operator::Bang => self.negation(right),
            _ => Err(Error::new("Unknown unary operation".to_string())),
        }
    }

    fn minus(self, expr: Expr) -> Result<Option<Expr>, Error> {
        match expr {
            Expr::Literal(Literal::Number(n)) => Ok(Some(Expr::Literal(Literal::Number(-n)))),
            _ => Err(Error::new("Operand must be a number.".to_string())),
        }
    }

    fn negation(self, expr: Expr) -> Result<Option<Expr>, Error> {
        match expr {
            Expr::Literal(Literal::Bool(b)) => Ok(Some(Expr::Literal(Literal::Bool(!b)))),
            _ => Err(Error::new("Operand must be a bool".to_string())),
        }
    }

    pub fn binary(self, left: Expr, right: Expr) -> Result<Option<Expr>, Error> {
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

    /*pub fn logical(self, left: Expr, right: Expr) -> Result<Option<Expr>, Error> {
        match self {
            Operator::Or => self.logical_or(left, right),
            Operator::And => self.logical_and(left, right),
            _ => Err(Error::new("Unknown logical operator".to_string())),
        }
    }

    fn logical_or(self, left: Expr, right: Expr) -> Result<Option<Expr>, Error> {
        Ok(left)
    }

    fn logical_and(self, left: Expr, right: Expr) -> Result<Option<Expr>, Error> {
        Ok(right)
    }*/

    fn addition(self, left: Expr, right: Expr) -> Result<Option<Expr>, Error> {
        match (left, right) {
            (Expr::Literal(Literal::Number(l)), Expr::Literal(Literal::Number(r))) => {
                Ok(Some(Expr::Literal(Literal::Number(l + r))))
            }
            (Expr::Literal(Literal::Str(l)), Expr::Literal(Literal::Str(r))) => {
                Ok(Some(Expr::Literal(Literal::Str(l + &r))))
            }
            _ => Err(Error::new(
                "Operands must be two numbers or two strings.".to_string(),
            )),
        }
    }

    fn subtraction(self, left: Expr, right: Expr) -> Result<Option<Expr>, Error> {
        match (left, right) {
            (Expr::Literal(Literal::Number(l)), Expr::Literal(Literal::Number(r))) => {
                Ok(Some(Expr::Literal(Literal::Number(l - r))))
            }
            _ => Err(Error::new("Operands must be numbers.".to_string())),
        }
    }

    fn multiplication(self, left: Expr, right: Expr) -> Result<Option<Expr>, Error> {
        match (left, right) {
            (Expr::Literal(Literal::Number(l)), Expr::Literal(Literal::Number(r))) => {
                Ok(Some(Expr::Literal(Literal::Number(l * r))))
            }
            _ => Err(Error::new("Operands must be numbers.".to_string())),
        }
    }

    fn division(self, left: Expr, right: Expr) -> Result<Option<Expr>, Error> {
        match (left, right) {
            (Expr::Literal(Literal::Number(l)), Expr::Literal(Literal::Number(r))) => {
                Ok(Some(Expr::Literal(Literal::Number(l / r))))
            }
            _ => Err(Error::new("Operands must be numbers.".to_string())),
        }
    }

    fn equal_equal(self, left: Expr, right: Expr) -> Result<Option<Expr>, Error> {
        match (left, right) {
            (
                Expr::Function(Function {
                    name: name1,
                    parameters: parameters1,
                    ..
                }),
                Expr::Function(Function {
                    name: name2,
                    parameters: parameters2,
                    ..
                }),
            ) => Ok(Some(Expr::Literal(Literal::Bool(
                name1 == name2 && parameters1 == parameters2,
            )))),
            (Expr::Class(Class { name: name1, .. }), Expr::Class(Class { name: name2, .. })) => {
                Ok(Some(Expr::Literal(Literal::Bool(name1 == name2))))
            }
            (Expr::Literal(Literal::Bool(l)), Expr::Literal(Literal::Bool(r))) => {
                Ok(Some(Expr::Literal(Literal::Bool(l == r))))
            }
            (Expr::Literal(Literal::Number(l)), Expr::Literal(Literal::Number(r))) => {
                Ok(Some(Expr::Literal(Literal::Bool(l == r))))
            }
            (Expr::Literal(Literal::Str(l)), Expr::Literal(Literal::Str(r))) => {
                Ok(Some(Expr::Literal(Literal::Bool(l == r))))
            }
            (Expr::Literal(Literal::Nil), Expr::Literal(Literal::Nil)) => {
                Ok(Some(Expr::Literal(Literal::Bool(true))))
            }
            (Expr::Literal(Literal::Nil), _) => Ok(Some(Expr::Literal(Literal::Bool(false)))),
            (_, Expr::Literal(Literal::Nil)) => Ok(Some(Expr::Literal(Literal::Bool(false)))),

            _ => Err(Error::new("Operands must be of the same type".to_string())),
        }
    }

    fn bang_equal(self, left: Expr, right: Expr) -> Result<Option<Expr>, Error> {
        match (left, right) {
            (Expr::Literal(Literal::Bool(l)), Expr::Literal(Literal::Bool(r))) => {
                Ok(Some(Expr::Literal(Literal::Bool(l != r))))
            }
            (Expr::Literal(Literal::Number(l)), Expr::Literal(Literal::Number(r))) => {
                Ok(Some(Expr::Literal(Literal::Bool(l != r))))
            }
            (Expr::Literal(Literal::Str(l)), Expr::Literal(Literal::Str(r))) => {
                Ok(Some(Expr::Literal(Literal::Bool(l != r))))
            }
            _ => Err(Error::new("Operands must be of the same type".to_string())),
        }
    }

    fn greater_than(self, left: Expr, right: Expr) -> Result<Option<Expr>, Error> {
        match (left, right) {
            (Expr::Literal(Literal::Number(l)), Expr::Literal(Literal::Number(r))) => {
                Ok(Some(Expr::Literal(Literal::Bool(l > r))))
            }
            _ => Err(Error::new("Operands must be numbers.".to_string())),
        }
    }

    fn greater_than_or_equal(self, left: Expr, right: Expr) -> Result<Option<Expr>, Error> {
        match (left, right) {
            (Expr::Literal(Literal::Number(l)), Expr::Literal(Literal::Number(r))) => {
                Ok(Some(Expr::Literal(Literal::Bool(l >= r))))
            }
            _ => Err(Error::new("Operands must be numbers.".to_string())),
        }
    }

    fn less_than(self, left: Expr, right: Expr) -> Result<Option<Expr>, Error> {
        match (left, right) {
            (Expr::Literal(Literal::Number(l)), Expr::Literal(Literal::Number(r))) => {
                Ok(Some(Expr::Literal(Literal::Bool(l < r))))
            }
            _ => Err(Error::new("Operands must be numbers.".to_string())),
        }
    }

    fn less_than_or_equal(self, left: Expr, right: Expr) -> Result<Option<Expr>, Error> {
        match (left, right) {
            (Expr::Literal(Literal::Number(l)), Expr::Literal(Literal::Number(r))) => {
                Ok(Some(Expr::Literal(Literal::Bool(l <= r))))
            }
            _ => Err(Error::new("Operands must be numbers.".to_string())),
        }
    }
}
