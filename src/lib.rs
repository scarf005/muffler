#[derive(Debug, PartialEq)]
pub struct Number(pub i32);

#[derive(Debug, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Number {
    pub fn new(s: &str) -> Self {
        Self(s.parse().unwrap())
    }
}

impl Op {
    pub fn new(s: &str) -> Self {
        match s {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            _ => panic!("Unknown operator: {s}"),
        }
    }
}

