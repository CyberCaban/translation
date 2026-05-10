use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

use crate::lexer::AType;

use super::ParseError;

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub vars_decls: Vec<VarsDecl>,
    pub body: Vec<Statement>,
}

impl Program {
    pub fn verify_invariants(&self) -> Result<(), ParseError> {
        let mut variables: HashMap<String, VarState> = HashMap::new();

        for declaration in &self.vars_decls {
            for var in declaration.iter() {
                if variables
                    .insert(
                        var.name.clone(),
                        VarState {
                            ttype: var.ttype.clone(),
                            initialized: false,
                        },
                    )
                    .is_some()
                {
                    return Err(ParseError::at(
                        format!("duplicate variable declaration for `{}`", var.name),
                        var.line,
                        var.column,
                    ));
                }
            }
        }

        self.verify_statements(&self.body, &mut variables)
    }

    fn verify_statements(
        &self,
        statements: &[Statement],
        variables: &mut HashMap<String, VarState>,
    ) -> Result<(), ParseError> {
        for statement in statements {
            self.verify_statement(statement, variables)?;
        }

        Ok(())
    }

    fn verify_statement(
        &self,
        statement: &Statement,
        variables: &mut HashMap<String, VarState>,
    ) -> Result<(), ParseError> {
        match statement {
            Statement::Assignment {
                var,
                value,
                line,
                column,
            } => {
                let expected_type = variables.get(var).ok_or_else(|| {
                    ParseError::at(format!("undeclared variable `{}`", var), *line, *column)
                })?.ttype.clone();
                let actual = self.infer_expr(value, variables)?;

                self.ensure_same_type(&expected_type, &actual.ty, actual.line, actual.column, "assignment")?;

                if let Some(symbol) = variables.get_mut(var) {
                    symbol.initialized = true;
                }

                Ok(())
            }
            Statement::If {
                condition,
                then_branch,
                ..
            } => {
                self.verify_comparison(condition, variables)?;
                let mut branch_variables = variables.clone();
                self.verify_statements(then_branch, &mut branch_variables)
            }
            Statement::Print { value, .. } => {
                self.infer_expr(value, variables)?;
                Ok(())
            }
        }
    }

    fn verify_comparison(
        &self,
        comparison: &Comparison,
        variables: &HashMap<String, VarState>,
    ) -> Result<(), ParseError> {
        match comparison {
            Comparison::Equal {
                left,
                right,
                line,
                column,
            }
            | Comparison::NotEqual {
                left,
                right,
                line,
                column,
            }
            | Comparison::Less {
                left,
                right,
                line,
                column,
            }
            | Comparison::LessEqual {
                left,
                right,
                line,
                column,
            }
            | Comparison::Greater {
                left,
                right,
                line,
                column,
            }
            | Comparison::GreaterEqual {
                left,
                right,
                line,
                column,
            } => {
                let left_type = self.infer_expr(left, variables)?;
                let right_type = self.infer_expr(right, variables)?;
                self.ensure_same_type(&left_type.ty, &right_type.ty, *line, *column, "comparison")
            }
            Comparison::Expression(expr) => {
                self.infer_expr(expr, variables)?;
                Ok(())
            }
        }
    }

    fn infer_expr(
        &self,
        expr: &Expr,
        variables: &HashMap<String, VarState>,
    ) -> Result<TypedValue, ParseError> {
        match expr {
            Expr::Plus {
                left,
                term,
                line,
                column,
            }
            | Expr::Minus {
                left,
                term,
                line,
                column,
            } => {
                let left_type = self.infer_expr(left, variables)?;
                let right_type = self.infer_term(term, variables)?;
                self.ensure_same_type(
                    &left_type.ty,
                    &right_type.ty,
                    *line,
                    *column,
                    "arithmetic operation",
                )?;
                Ok(TypedValue::new(left_type.ty, *line, *column))
            }
            Expr::Term(term) => self.infer_term(term, variables),
        }
    }

    fn infer_term(
        &self,
        term: &Term,
        variables: &HashMap<String, VarState>,
    ) -> Result<TypedValue, ParseError> {
        match term {
            Term::Multiply {
                left,
                factor,
                line,
                column,
            }
            | Term::Divide {
                left,
                factor,
                line,
                column,
            } => {
                let left_type = self.infer_term(left, variables)?;
                let right_type = self.infer_factor(factor, variables)?;
                self.ensure_same_type(
                    &left_type.ty,
                    &right_type.ty,
                    *line,
                    *column,
                    "arithmetic operation",
                )?;
                Ok(TypedValue::new(left_type.ty, *line, *column))
            }
            Term::Factor(factor) => self.infer_factor(factor, variables),
        }
    }

    fn infer_factor(
        &self,
        factor: &Factor,
        variables: &HashMap<String, VarState>,
    ) -> Result<TypedValue, ParseError> {
        match factor {
            Factor::Variable { name, line, column } => {
                let symbol = variables.get(name).ok_or_else(|| {
                    ParseError::at(format!("undeclared variable `{}`", name), *line, *column)
                })?;

                if !symbol.initialized {
                    return Err(ParseError::at(
                        format!("use of uninitialized variable `{}`", name),
                        *line,
                        *column,
                    ));
                }

                Ok(TypedValue::new(symbol.ttype.clone(), *line, *column))
            }
            Factor::IntLiteral { line, column, .. } => {
                Ok(TypedValue::new(AType::Int, *line, *column))
            }
            Factor::FloatLiteral { line, column, .. } => {
                Ok(TypedValue::new(AType::Float, *line, *column))
            }
            Factor::Paren { expr, line, column } => {
                let value = self.infer_expr(expr, variables)?;
                Ok(TypedValue::new(value.ty, *line, *column))
            }
            Factor::Cast {
                target,
                expr,
                line,
                column,
            } => {
                self.infer_expr(expr, variables)?;
                Ok(TypedValue::new(target.clone(), *line, *column))
            }
        }
    }

    fn ensure_same_type(
        &self,
        expected: &AType,
        actual: &AType,
        line: usize,
        column: usize,
        context: &str,
    ) -> Result<(), ParseError> {
        if expected == actual {
            return Ok(());
        }

        Err(ParseError::at(
            format!(
                "type mismatch in {}: expected `{:?}`, found `{:?}`",
                context, expected, actual
            ),
            line,
            column,
        ))
    }
}

#[derive(Debug, Clone, PartialEq)]
struct TypedValue {
    ty: AType,
    line: usize,
    column: usize,
}

impl TypedValue {
    fn new(ty: AType, line: usize, column: usize) -> Self {
        Self { ty, line, column }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct VarState {
    ttype: AType,
    initialized: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Var {
    pub ttype: AType,
    pub name: String,
    pub line: usize,
    pub column: usize,
}
impl Var {
    pub fn new(ttype: AType, name: String, line: usize, column: usize) -> Self {
        Self {
            ttype,
            name,
            line,
            column,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VarsDecl(Vec<Var>);
impl VarsDecl {
    pub fn new(vars: Vec<Var>) -> Self {
        Self(vars)
    }
}
impl Deref for VarsDecl {
    type Target = Vec<Var>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for VarsDecl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Assignment {
        var: String,
        value: Expr,
        line: usize,
        column: usize,
    },
    If {
        condition: Comparison,
        then_branch: Vec<Statement>,
        line: usize,
        column: usize,
    },
    Print {
        value: Expr,
        line: usize,
        column: usize,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Comparison {
    Equal {
        left: Expr,
        right: Expr,
        line: usize,
        column: usize,
    },
    NotEqual {
        left: Expr,
        right: Expr,
        line: usize,
        column: usize,
    },
    Less {
        left: Expr,
        right: Expr,
        line: usize,
        column: usize,
    },
    LessEqual {
        left: Expr,
        right: Expr,
        line: usize,
        column: usize,
    },
    Greater {
        left: Expr,
        right: Expr,
        line: usize,
        column: usize,
    },
    GreaterEqual {
        left: Expr,
        right: Expr,
        line: usize,
        column: usize,
    },
    Expression(Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Plus {
        left: Box<Expr>,
        term: Term,
        line: usize,
        column: usize,
    },
    Minus {
        left: Box<Expr>,
        term: Term,
        line: usize,
        column: usize,
    },
    Term(Term),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Term {
    Multiply {
        left: Box<Term>,
        factor: Factor,
        line: usize,
        column: usize,
    },
    Divide {
        left: Box<Term>,
        factor: Factor,
        line: usize,
        column: usize,
    },
    Factor(Factor),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Factor {
    Variable {
        name: String,
        line: usize,
        column: usize,
    },
    IntLiteral {
        value: i128,
        line: usize,
        column: usize,
    },
    FloatLiteral {
        value: f64,
        line: usize,
        column: usize,
    },
    Paren {
        expr: Box<Expr>,
        line: usize,
        column: usize,
    },
    Cast {
        target: AType,
        expr: Box<Expr>,
        line: usize,
        column: usize,
    },
}
