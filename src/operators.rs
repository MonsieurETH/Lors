use crate::{
    ast::{Error, Expr, Literal},
    lexer::{Token, TokenType},
};

#[derive(Debug, Clone, PartialEq)]
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

    /*pub fn to_string(&self) -> String {
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
    }*/

    pub fn unary(self, right: Expr) -> Expr {
        match self {
            Operator::Minus => self.minus(right),
            Operator::Bang => self.negation(right),
            _ => panic!("Unknown unary operation"),
        }
    }

    fn minus(self, expr: Expr) -> Expr {
        match expr {
            Expr::Literal(Literal::Number(n)) => Expr::Literal(Literal::Number(n)),
            _ => panic!("Operand must be a number"),
        }
    }

    fn negation(self, expr: Expr) -> Expr {
        match expr {
            Expr::Literal(Literal::Bool(b)) => Expr::Literal(Literal::Bool(b)),
            _ => panic!("Operand must be a bool"),
        }
    }

    pub fn binary(self, left: Expr, right: Expr) -> Result<Expr, Error> {
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

    /*pub fn logical(self, left: Expr, right: Expr) -> Result<Expr, Error> {
        match self {
            Operator::Or => self.logical_or(left, right),
            Operator::And => self.logical_and(left, right),
            _ => Err(Error::new("Unknown logical operator".to_string())),
        }
    }

    fn logical_or(self, left: Expr, right: Expr) -> Result<Expr, Error> {
        Ok(left)
    }

    fn logical_and(self, left: Expr, right: Expr) -> Result<Expr, Error> {
        Ok(right)
    }*/

    fn addition(self, left: Expr, right: Expr) -> Result<Expr, Error> {
        match (left, right) {
            (Expr::Literal(Literal::Number(l)), Expr::Literal(Literal::Number(r))) => {
                Ok(Expr::Literal(Literal::Number(l + r)))
            }
            (Expr::Literal(Literal::Str(l)), Expr::Literal(Literal::Str(r))) => {
                Ok(Expr::Literal(Literal::Str(l + &r)))
            }
            _ => Err(Error::new(
                "Operands must be two numbers or two strings.".to_string(),
            )),
        }
    }

    fn subtraction(self, left: Expr, right: Expr) -> Result<Expr, Error> {
        match (left, right) {
            (Expr::Literal(Literal::Number(l)), Expr::Literal(Literal::Number(r))) => {
                Ok(Expr::Literal(Literal::Number(l - r)))
            }
            _ => Err(Error::new("Operands must be numbers.".to_string())),
        }
    }

    fn multiplication(self, left: Expr, right: Expr) -> Result<Expr, Error> {
        match (left, right) {
            (Expr::Literal(Literal::Number(l)), Expr::Literal(Literal::Number(r))) => {
                Ok(Expr::Literal(Literal::Number(l * r)))
            }
            _ => Err(Error::new("Operands must be numbers.".to_string())),
        }
    }

    fn division(self, left: Expr, right: Expr) -> Result<Expr, Error> {
        match (left, right) {
            (Expr::Literal(Literal::Number(l)), Expr::Literal(Literal::Number(r))) => {
                Ok(Expr::Literal(Literal::Number(l / r)))
            }
            _ => Err(Error::new("Operands must be numbers.".to_string())),
        }
    }

    fn equal_equal(self, left: Expr, right: Expr) -> Result<Expr, Error> {
        match (left, right) {
            (Expr::Literal(Literal::Bool(l)), Expr::Literal(Literal::Bool(r))) => {
                Ok(Expr::Literal(Literal::Bool(l == r)))
            }
            (Expr::Literal(Literal::Number(l)), Expr::Literal(Literal::Number(r))) => {
                Ok(Expr::Literal(Literal::Bool(l == r)))
            }
            (Expr::Literal(Literal::Str(l)), Expr::Literal(Literal::Str(r))) => {
                Ok(Expr::Literal(Literal::Bool(l == r)))
            }
            _ => Err(Error::new("Operands must be of the same type".to_string())),
        }
    }

    fn bang_equal(self, left: Expr, right: Expr) -> Result<Expr, Error> {
        match (left, right) {
            (Expr::Literal(Literal::Bool(l)), Expr::Literal(Literal::Bool(r))) => {
                Ok(Expr::Literal(Literal::Bool(l != r)))
            }
            (Expr::Literal(Literal::Number(l)), Expr::Literal(Literal::Number(r))) => {
                Ok(Expr::Literal(Literal::Bool(l != r)))
            }
            (Expr::Literal(Literal::Str(l)), Expr::Literal(Literal::Str(r))) => {
                Ok(Expr::Literal(Literal::Bool(l != r)))
            }
            _ => Err(Error::new("Operands must be of the same type".to_string())),
        }
    }

    fn greater_than(self, left: Expr, right: Expr) -> Result<Expr, Error> {
        match (left, right) {
            (Expr::Literal(Literal::Number(l)), Expr::Literal(Literal::Number(r))) => {
                Ok(Expr::Literal(Literal::Bool(l > r)))
            }
            _ => Err(Error::new("Operands must be numbers".to_string())),
        }
    }

    fn greater_than_or_equal(self, left: Expr, right: Expr) -> Result<Expr, Error> {
        match (left, right) {
            (Expr::Literal(Literal::Number(l)), Expr::Literal(Literal::Number(r))) => {
                Ok(Expr::Literal(Literal::Bool(l >= r)))
            }
            _ => Err(Error::new("Operands must be numbers".to_string())),
        }
    }

    fn less_than(self, left: Expr, right: Expr) -> Result<Expr, Error> {
        match (left, right) {
            (Expr::Literal(Literal::Number(l)), Expr::Literal(Literal::Number(r))) => {
                Ok(Expr::Literal(Literal::Bool(l < r)))
            }
            _ => Err(Error::new("Operands must be numbers".to_string())),
        }
    }

    fn less_than_or_equal(self, left: Expr, right: Expr) -> Result<Expr, Error> {
        match (left, right) {
            (Expr::Literal(Literal::Number(l)), Expr::Literal(Literal::Number(r))) => {
                Ok(Expr::Literal(Literal::Bool(l <= r)))
            }
            _ => Err(Error::new("Operands must be numbers".to_string())),
        }
    }
}
