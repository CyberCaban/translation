use std::collections::HashMap;

use anyhow::{bail, Result};

use crate::{
    lexer::{AType, Lexer},
    parser::{Comparison, Expr, Factor, Parser, Program, Statement, Term},
};

pub struct Interpreter {
    program: Program,
}

#[derive(Debug, Clone, PartialEq)]
enum RuntimeValue {
    Int(i128),
    Float(f64),
}

impl std::fmt::Display for RuntimeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeValue::Int(value) => write!(f, "{}", value),
            RuntimeValue::Float(value) => write!(f, "{}", value),
        }
    }
}

impl Interpreter {
    pub fn new(input: &str) -> Result<Interpreter> {
        let mut lexer = Lexer::new();
        let tokens = lexer.lex(input)?;
        let mut parser = Parser::new(tokens);
        let program = parser.parse_program()?;
        program.verify_invariants()?;
        Ok(Self { program })
    }

    pub fn interpret(&self) -> Result<()> {
        let mut variables = self.init_variables();

        for stmt in &self.program.body {
            self.execute_statement(stmt, &mut variables)?;
        }

        Ok(())
    }

    fn init_variables(&self) -> HashMap<String, RuntimeValue> {
        let mut variables = HashMap::new();

        for declaration in &self.program.vars_decls {
            for var in declaration.iter() {
                let value = match var.ttype {
                    AType::Int => RuntimeValue::Int(0),
                    AType::Float => RuntimeValue::Float(0.0),
                    _ => unreachable!(),
                };
                variables.insert(var.name.clone(), value);

            }
        }

        variables
    }

    fn execute_statement(
        &self,
        statement: &Statement,
        variables: &mut HashMap<String, RuntimeValue>,
    ) -> Result<()> {
        match statement {
            Statement::Assignment { var, value, .. } => {
                let evaluated = self.evaluate_expr(value, variables)?;
                variables.insert(var.clone(), evaluated);
            }
            Statement::Print { value, .. } => {
                let evaluated = self.evaluate_expr(value, variables)?;
                println!("{}", evaluated);
            }
            Statement::If {
                condition,
                then_branch,
                ..
            } => {
                if self.evaluate_comparison(condition, variables)? {
                    for nested in then_branch {
                        self.execute_statement(nested, variables)?;
                    }
                }
            }
        }

        Ok(())
    }

    fn evaluate_expr(
        &self,
        expr: &Expr,
        variables: &HashMap<String, RuntimeValue>,
    ) -> Result<RuntimeValue> {
        match expr {
            Expr::Plus { left, term, .. } => {
                let lhs = self.evaluate_expr(left, variables)?;
                let rhs = self.evaluate_term(term, variables)?;
                self.add_values(lhs, rhs)
            }
            Expr::Minus { left, term, .. } => {
                let lhs = self.evaluate_expr(left, variables)?;
                let rhs = self.evaluate_term(term, variables)?;
                self.sub_values(lhs, rhs)
            }
            Expr::Term(term) => self.evaluate_term(term, variables),
        }
    }

    fn evaluate_term(
        &self,
        term: &Term,
        variables: &HashMap<String, RuntimeValue>,
    ) -> Result<RuntimeValue> {
        match term {
            Term::Multiply { left, factor, .. } => {
                let lhs = self.evaluate_term(left, variables)?;
                let rhs = self.evaluate_factor(factor, variables)?;
                self.mul_values(lhs, rhs)
            }
            Term::Divide { left, factor, .. } => {
                let lhs = self.evaluate_term(left, variables)?;
                let rhs = self.evaluate_factor(factor, variables)?;
                self.div_values(lhs, rhs)
            }
            Term::Factor(factor) => self.evaluate_factor(factor, variables),
        }
    }

    fn evaluate_factor(
        &self,
        factor: &Factor,
        variables: &HashMap<String, RuntimeValue>,
    ) -> Result<RuntimeValue> {
        match factor {
            Factor::Variable { name, .. } => variables
                .get(name)
                .cloned()
                .ok_or_else(|| anyhow::anyhow!("use of undeclared variable `{}`", name)),
            Factor::IntLiteral { value, .. } => Ok(RuntimeValue::Int(*value)),
            Factor::FloatLiteral { value, .. } => Ok(RuntimeValue::Float(*value)),
            Factor::Paren { expr, .. } => self.evaluate_expr(expr, variables),
            Factor::Cast { target, expr, .. } => {
                let value = self.evaluate_expr(expr, variables)?;
                self.cast_value(value, target)
            }
        }
    }

    fn cast_value(&self, value: RuntimeValue, target: &AType) -> Result<RuntimeValue> {
        match (value, target) {
            (RuntimeValue::Int(v), AType::Int) => Ok(RuntimeValue::Int(v)),
            (RuntimeValue::Int(v), AType::Float) => Ok(RuntimeValue::Float(v as f64)),
            (RuntimeValue::Float(v), AType::Float) => Ok(RuntimeValue::Float(v)),
            (RuntimeValue::Float(v), AType::Int) => Ok(RuntimeValue::Int(v as i128)),
            (_, AType::UserDefined(name)) => bail!("unsupported cast target type `{}`", name),
        }
    }

    fn evaluate_comparison(
        &self,
        comparison: &Comparison,
        variables: &HashMap<String, RuntimeValue>,
    ) -> Result<bool> {
        match comparison {
            Comparison::Equal { left, right, .. } => {
                let lhs = self.evaluate_expr(left, variables)?;
                let rhs = self.evaluate_expr(right, variables)?;
                self.compare_values(lhs, rhs, |a, b| a == b)
            }
            Comparison::NotEqual { left, right, .. } => {
                let lhs = self.evaluate_expr(left, variables)?;
                let rhs = self.evaluate_expr(right, variables)?;
                self.compare_values(lhs, rhs, |a, b| a != b)
            }
            Comparison::Less { left, right, .. } => {
                let lhs = self.evaluate_expr(left, variables)?;
                let rhs = self.evaluate_expr(right, variables)?;
                self.compare_values(lhs, rhs, |a, b| a < b)
            }
            Comparison::LessEqual { left, right, .. } => {
                let lhs = self.evaluate_expr(left, variables)?;
                let rhs = self.evaluate_expr(right, variables)?;
                self.compare_values(lhs, rhs, |a, b| a <= b)
            }
            Comparison::Greater { left, right, .. } => {
                let lhs = self.evaluate_expr(left, variables)?;
                let rhs = self.evaluate_expr(right, variables)?;
                self.compare_values(lhs, rhs, |a, b| a > b)
            }
            Comparison::GreaterEqual { left, right, .. } => {
                let lhs = self.evaluate_expr(left, variables)?;
                let rhs = self.evaluate_expr(right, variables)?;
                self.compare_values(lhs, rhs, |a, b| a >= b)
            }
            Comparison::Expression(expr) => {
                let value = self.evaluate_expr(expr, variables)?;
                match value {
                    RuntimeValue::Int(v) => Ok(v != 0),
                    RuntimeValue::Float(v) => Ok(v != 0.0),
                }
            }
        }
    }

    fn add_values(&self, lhs: RuntimeValue, rhs: RuntimeValue) -> Result<RuntimeValue> {
        match (lhs, rhs) {
            (RuntimeValue::Int(a), RuntimeValue::Int(b)) => Ok(RuntimeValue::Int(a + b)),
            (RuntimeValue::Float(a), RuntimeValue::Float(b)) => Ok(RuntimeValue::Float(a + b)),
            _ => bail!("type mismatch during addition"),
        }
    }

    fn sub_values(&self, lhs: RuntimeValue, rhs: RuntimeValue) -> Result<RuntimeValue> {
        match (lhs, rhs) {
            (RuntimeValue::Int(a), RuntimeValue::Int(b)) => Ok(RuntimeValue::Int(a - b)),
            (RuntimeValue::Float(a), RuntimeValue::Float(b)) => Ok(RuntimeValue::Float(a - b)),
            _ => bail!("type mismatch during subtraction"),
        }
    }

    fn mul_values(&self, lhs: RuntimeValue, rhs: RuntimeValue) -> Result<RuntimeValue> {
        match (lhs, rhs) {
            (RuntimeValue::Int(a), RuntimeValue::Int(b)) => Ok(RuntimeValue::Int(a * b)),
            (RuntimeValue::Float(a), RuntimeValue::Float(b)) => Ok(RuntimeValue::Float(a * b)),
            _ => bail!("type mismatch during multiplication"),
        }
    }

    fn div_values(&self, lhs: RuntimeValue, rhs: RuntimeValue) -> Result<RuntimeValue> {
        match (lhs, rhs) {
            (RuntimeValue::Int(_), RuntimeValue::Int(0)) => bail!("division by zero"),
            (RuntimeValue::Float(_), RuntimeValue::Float(v)) if v == 0.0 => {
                bail!("division by zero")
            }
            (RuntimeValue::Int(a), RuntimeValue::Int(b)) => Ok(RuntimeValue::Int(a / b)),
            (RuntimeValue::Float(a), RuntimeValue::Float(b)) => Ok(RuntimeValue::Float(a / b)),
            _ => bail!("type mismatch during division"),
        }
    }

    fn compare_values<F>(&self, lhs: RuntimeValue, rhs: RuntimeValue, op: F) -> Result<bool>
    where
        F: FnOnce(f64, f64) -> bool,
    {
        match (lhs, rhs) {
            (RuntimeValue::Int(a), RuntimeValue::Int(b)) => Ok(op(a as f64, b as f64)),
            (RuntimeValue::Float(a), RuntimeValue::Float(b)) => Ok(op(a, b)),
            _ => bail!("type mismatch during comparison"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_variables(input: &str) -> HashMap<String, RuntimeValue> {
        let interpreter = Interpreter::new(input).expect("interpreter init failed");
        let mut variables = interpreter.init_variables();

        for stmt in &interpreter.program.body {
            interpreter
                .execute_statement(stmt, &mut variables)
                .expect("execution failed");
        }

        variables
    }

    #[test]
    fn executes_if_then_single_statement() {
        let vars = run_variables("int x; begin x = 10; if x == 10 then begin x = 20;end; end");
        assert_eq!(vars.get("x"), Some(&RuntimeValue::Int(20)));
    }

    #[test]
    fn executes_if_then_block_statement() {
        let vars = run_variables(
            "int x, y; begin x = 10; if x <= 10 then begin y = 1; x = x + y; end; end",
        );
        assert_eq!(vars.get("x"), Some(&RuntimeValue::Int(11)));
        assert_eq!(vars.get("y"), Some(&RuntimeValue::Int(1)));
    }

    #[test]
    fn skips_then_branch_when_condition_false() {
        let vars = run_variables("int x; begin x = 10; if x > 10 then begin x = 99;end; end");
        assert_eq!(vars.get("x"), Some(&RuntimeValue::Int(10)));
    }
}
