use core::fmt;
use std::{collections::HashMap, rc::Rc};

use anyhow::bail;

use crate::parse::{BinaryOp, Expr, Function, Literal, Module, Stmt};

struct Interpreter<'a> {
    ast: &'a Module,
    vars: HashMap<&'a str, NonVarLiteral>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum NonVarLiteral {
    Integer(i64),
    Bool(bool),
    String(String),
}

impl fmt::Display for NonVarLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NonVarLiteral::Integer(i) => f.write_str(&i.to_string()),
            NonVarLiteral::Bool(b) => f.write_str(&b.to_string()),
            NonVarLiteral::String(s) => f.write_str(s),
        }
    }
}

// Most (in)efficient interpreter ever
impl<'a> Interpreter<'a> {
    pub fn new(ast: &'a Module) -> Self {
        Interpreter {
            ast,
            vars: HashMap::new(),
        }
    }

    // Convert a Literal to a NonVarLiteral, which will never contain a variable
    pub fn to_nonvar_literal(&self, literal: &&Literal) -> anyhow::Result<NonVarLiteral> {
        match literal {
            Literal::Variable(name) => match self.vars.get(name.as_str()) {
                Some(var) => Ok(var.clone().clone()),
                None => bail!("Could not find variable {name}!"),
            },
            Literal::Integer(i) => Ok(NonVarLiteral::Integer(*i)),
            Literal::Bool(b) => Ok(NonVarLiteral::Bool(*b)),
            Literal::String(s) => Ok(NonVarLiteral::String(s.to_string())),
        }
    }

    pub fn compute_expr(&self, expr: &'a Expr) -> anyhow::Result<NonVarLiteral> {
        let value = match expr {
            Expr::Literal(literal) => self.to_nonvar_literal(&literal)?,
            Expr::Binary(literal, binary_op, expr) => {
                let value: Result<NonVarLiteral, anyhow::Error> = match (
                    self.to_nonvar_literal(&literal)?,
                    binary_op,
                    self.compute_expr(expr)?,
                ) {
                    (NonVarLiteral::Integer(l), BinaryOp::Add, NonVarLiteral::Integer(r)) => Ok(NonVarLiteral::Integer(l+r)),
                    (NonVarLiteral::Integer(l), BinaryOp::Sub, NonVarLiteral::Integer(r)) => Ok(NonVarLiteral::Integer(l-r)),
                    (NonVarLiteral::Integer(l), BinaryOp::Mul, NonVarLiteral::Integer(r)) => Ok(NonVarLiteral::Integer(l*r)),
                    (NonVarLiteral::Integer(l), BinaryOp::Div, NonVarLiteral::Integer(r)) => Ok(NonVarLiteral::Integer(l/r)),
                    _ => bail!("Invalid operation"),
                };

                value?
            },
        };

        Ok(value.clone())
    }

    pub fn interpret_stmt(&mut self, stmt: &'a Stmt) -> anyhow::Result<()> {
        match stmt {
            Stmt::FunctionCall(function_call) => {
                let function = match self
                    .ast
                    .functions
                    .iter()
                    .find(|f| f.name == function_call.name)
                {
                    Some(f) => f,
                    None => bail!("Function {} not found!", function_call.name),
                };

                self.interpret_fn(function, function_call.args.clone())?;
            }
            Stmt::SetVariable(set_variable) => {
                let value = self.compute_expr(&set_variable.value)?;
                self.vars.insert(&set_variable.name, value);
            }
        }

        Ok(())
    }

    pub fn interpret_fn(
        &mut self,
        function: &'a Function,
        input_args: Vec<Literal>,
    ) -> anyhow::Result<()> {
        if function.name == "print" {
            for arg in input_args {
                println!("{}", self.to_nonvar_literal(&&arg)?);
            }

            return Ok(());
        }

        // Add all input arguments to the global variable list
        for (i, expected_arg) in function.args.iter().enumerate() {
            match input_args.get(i) {
                Some(arg) => {
                    self.vars
                        .insert(expected_arg, self.to_nonvar_literal(&arg)?);
                }
                None => bail!("Expected arg {expected_arg} at index {i}"),
            }
        }

        for stmt in &function.body {
            self.interpret_stmt(stmt)?;
        }

        Ok(())
    }
}

pub fn interpet(mut ast: Module) -> anyhow::Result<()> {
    ast.functions.push(Function { name: String::from("print"), args: Vec::new(), body: Vec::new() });
    let mut interpreter = Interpreter::new(&ast);

    let main_function = match ast.functions.iter().find(|f| f.name == "main") {
        Some(f) => f,
        None => bail!("No main function found!"),
    };

    interpreter.interpret_fn(main_function, Vec::new())?;

    Ok(())
}
