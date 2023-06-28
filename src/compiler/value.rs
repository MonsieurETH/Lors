use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap};
use std::hash::{Hash, Hasher};
use ordered_float::OrderedFloat;


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Bool(bool),
    Nil,
    Number(OrderedFloat<f64>),
    String(String),
    Hashmap(HashMap<Value, Value>),
}

impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Value::Bool(b) => b.hash(state),
            Value::Nil => ().hash(state),
            Value::Number(n) => n.hash(state),
            Value::String(s) => s.hash(state),
            Value::Hashmap(m) => {
                let mut hasher = DefaultHasher::new();
                for (k, v) in m {
                    k.hash(&mut hasher);
                    v.hash(&mut hasher);
                }
                hasher.finish().hash(state);
            }
        }
    }
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

    pub fn is_string(&self) -> bool {
        match self {
            Value::String(_) => true,
            _ => false,
        }
    }

    pub fn as_bool(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            _ => false,
        }
    }

    pub fn as_number(&self) -> OrderedFloat<f64> {
        match self {
            Value::Number(n) => *n,
            _ => OrderedFloat(0.0),
        }
    }

    pub fn as_string(&self) -> &str {
        match self {
            Value::String(s) => s,
            _ => "",
        }
    }

    pub fn from_f64(n: OrderedFloat<f64>) -> Self {
        Value::Number(n)
    }

    pub fn from_bool(b: bool) -> Self {
        Value::Bool(b)
    }

    pub fn from_string(s: String) -> Self {
        Value::String(s)
    }

    pub fn is_falsey(&self) -> bool {
        match self {
            Value::Nil => true,
            Value::Bool(b) => !*b,
            _ => false,
        }
    }
}
