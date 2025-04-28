#![forbid(unsafe_code)]

use std::{collections::HashMap, fmt::Display};

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Number(f64),
    Symbol(String),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(num) => write!(f, "{}", num),
            Self::Symbol(sym) => write!(f, "'{}", sym),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct Interpreter {
    stack: Vec<Value>,
    variables: HashMap<String, Value>,
}

enum Operation {
    Add,
    Sub,
    Mul,
    Div,
    Set,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            stack: Vec::new(),
            variables: HashMap::new(),
        }
    }

    fn apply(&mut self, op: Operation) {
        if self.stack.len() < 2 {
            panic!("Not enough values to apply operation");
        }

        match (op, self.stack.pop().unwrap(), self.stack.pop().unwrap()) {
            (Operation::Add, Value::Number(x), Value::Number(y)) => self.upd_stack(x + y),
            (Operation::Sub, Value::Number(x), Value::Number(y)) => self.upd_stack(x - y),
            (Operation::Mul, Value::Number(x), Value::Number(y)) => self.upd_stack(x * y),
            (Operation::Div, Value::Number(x), Value::Number(y)) => self.upd_stack(x / y),
            (Operation::Set, Value::Symbol(sym), val) => self.upd_variables(sym, val),
            _ => panic!("Invalid operation or types"),
        }
    }

    pub fn upd_stack(&mut self, num: f64) {
        self.stack.push(Value::Number(num));
    }

    pub fn upd_variables(&mut self, key: String, val: Value) {
        self.variables.insert(key, val);
    }

    pub fn eval(&mut self, expr: &str) {
        let arguments = expr.split_whitespace();
        for argument in arguments {
            match argument.parse::<f64>() {
                Ok(num) => self.stack.push(Value::Number(num)),
                Err(_) => match argument {
                    "+" => self.apply(Operation::Add),
                    "-" => self.apply(Operation::Sub),
                    "*" => self.apply(Operation::Mul),
                    "/" => self.apply(Operation::Div),
                    "set" => self.apply(Operation::Set),
                    s if s.starts_with("'") => self.stack.push(Value::Symbol(s[1..].to_string())),
                    s if s.starts_with("$") => {
                        let temp = self.variables.get(&s[1..]).unwrap();
                        self.stack.push(temp.clone())
                    }
                    s => panic!("Unknown symbol {}", s),
                },
            }
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}
