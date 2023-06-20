
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Bool(bool),
    Nil,
    Number(f64),
}

impl Value {
    pub fn is_nil(&self) -> bool {
        match self {
            Value::Nil => true,
            _ => false,
        }
    }

    pub fn is_bool(&self) -> bool {
        match self {
            Value::Bool(_) => true,
            _ => false,
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            Value::Number(_) => true,
            _ => false,
        }
    }

    pub fn as_bool(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            _ => false,
        }
    }

    pub fn as_number(&self) -> f64 {
        match self {
            Value::Number(n) => *n,
            _ => 0.0,
        }
    }

    pub fn from_f64(n: f64) -> Self {
        Value::Number(n)
    }

    pub fn from_bool(b: bool) -> Self {
        Value::Bool(b)
    }

    pub fn is_falsey(&self) -> bool {
        match self {
            Value::Nil => true,
            Value::Bool(b) => !*b,
            _ => false,
        }
    }
}
