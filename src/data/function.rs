use crate::{
    ast::Expression,
    interpreter::{Functions, Interpreter, Std, Variables},
};

use super::{unsizedset::{BaseSet, UnsizedSet}, Data};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Function {
    pub name: String,
    pub args: Vec<String>,
    pub expr: Expression,
    pub domain: UnsizedSet,
}

impl Function {
    pub fn new(name: String, args: Vec<String>, expr: Expression) -> Self {
        Self {
            name,
            args,
            expr,
            domain: UnsizedSet::from_base_set(BaseSet::Real),
        }
    }

    pub fn run(&self, variables: &Variables, functions: &Functions, std: &Std) -> Data {
        
        Interpreter::eval_expression(&self.expr, variables, functions, std)
    }
}

