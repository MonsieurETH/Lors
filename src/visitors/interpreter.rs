use std::collections::HashMap;

use crate::ast::{Callable, Expr, IVisitorExpr, IVisitorStmt, Operator, Stmt};
use crate::parser::Type;

#[derive(Debug)]
pub struct Environment {
    enclosing: Option<Box<Environment>>,
    pub symbol_table: HashMap<String, Box<Expr>>,
}

impl Clone for Environment {
    fn clone(&self) -> Self {
        Self {
            enclosing: self.enclosing.clone(),
            symbol_table: self.symbol_table.clone(),
        }
    }
}
pub struct Interpreter {
    pub env: Environment,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let symbol_table = HashMap::new();
        let env: Environment = Environment {
            enclosing: None,
            symbol_table,
        };
        Interpreter { env }
    }

    pub fn get(&self, symbol: &str) -> Option<Box<Expr>> {
        let mut current_env = &self.env;
        while let Some(expr) = current_env.symbol_table.get(symbol) {
            return Some(expr.clone());
        }
        while let Some(enclosing) = &current_env.enclosing {
            current_env = enclosing.as_ref();
            if let Some(expr) = current_env.symbol_table.get(symbol) {
                return Some(expr.clone());
            }
        }
        None
    }

    pub fn define(&mut self, symbol: String, expr: Box<Expr>) {
        self.env.symbol_table.insert(symbol, expr);
    }

    // This function is pretty awful, and separating 'define' & 'assign'
    // looks like a bad idea in the long term. (check & rewrite)"
    pub fn assign(&mut self, symbol: String, expr: Box<Expr>) {
        let mut current_env = &mut self.env;
        let str = symbol.as_str();
        loop {
            if current_env.symbol_table.contains_key(str) {
                current_env
                    .symbol_table
                    .insert(symbol.clone(), expr.clone());
                return;
            }

            if let Some(enclosing) = &mut current_env.enclosing {
                current_env = enclosing.as_mut();
            } else {
                break;
            }
        }
    }
}

impl<'a> IVisitorStmt<'a, ()> for Interpreter {
    fn visit_stmt_expr(&mut self, stmt: &'a Stmt) {
        if let Stmt::Expression(expr) = stmt {
            expr.accept(self);
        } else {
            panic!("ERROR")
        }
    }

    fn visit_stmt_print(&mut self, stmt: &'a Stmt) {
        if let Stmt::Print(expr) = stmt {
            println!("{:?}", expr.accept(self));
        } else {
            panic!("ERROR")
        }
    }

    fn visit_var_decl(&mut self, stmt: &'a Stmt) {
        if let Stmt::VarDecl(name, expr) = stmt {
            let accepted_expr = expr.accept(self);
            self.define(name.to_owned(), Box::new(accepted_expr));
        } else {
            panic!("ERROR")
        }
    }

    fn visit_block(&mut self, stmt: &'a Stmt) {
        if let Stmt::Block(statements) = stmt {
            self.execute_block(statements);
        } else {
            panic!("ERROR")
        }
    }

    fn visit_if(&mut self, stmt: &'a Stmt) {
        if let Stmt::If(condition, true_branch, false_branch) = stmt {
            let eval_condition = condition.accept(self);
            if let Expr::Value(Type::Bool(b)) = eval_condition {
                if b {
                    true_branch.accept(self)
                } else {
                    false_branch.accept(self)
                }
            }
        }
    }

    fn visit_while(&mut self, stmt: &'a Stmt) {
        if let Stmt::While(condition, body) = stmt {
            let mut accepted_cond = condition.accept(self);
            while let Expr::Value(Type::Bool(true)) = accepted_cond {
                accepted_cond = condition.accept(self);
                body.accept(self)
            }
        }
    }

    fn execute_block(&mut self, stmts: &Vec<Stmt>) {
        // TODO Another function with terrible code. Need to review and rewrite it.
        let symbol_table = HashMap::new();
        let mut new: Environment = Environment {
            enclosing: Some(Box::new(self.env.clone())),
            symbol_table,
        };
        _ = std::mem::swap(&mut self.env, &mut new);

        for stmt in stmts {
            stmt.accept(self)
        }

        _ = std::mem::swap(self.env.enclosing.clone().unwrap().as_mut(), &mut self.env);
    }
}

impl<'a> IVisitorExpr<'a, crate::ast::Expr> for Interpreter {
    fn visit_value(&mut self, expr: &'a Expr) -> crate::ast::Expr {
        expr.clone()
    }

    fn visit_unary(&mut self, expr: &'a Expr) -> crate::ast::Expr {
        if let Expr::Unary { operator, right } = expr {
            let accepted_right = right.accept(self);
            operator.clone().unary(accepted_right)
        } else {
            panic!("ERROR")
        }
    }

    fn visit_binary(&mut self, expr: &'a Expr) -> crate::ast::Expr {
        if let Expr::Binary {
            left,
            operator,
            right,
        } = expr
        {
            let accepted_left = left.accept(self);
            let accepted_right = right.accept(self);
            operator.clone().binary(accepted_left, accepted_right)
        } else {
            panic!("ERROR")
        }
    }

    fn visit_grouping(&mut self, expr: &'a Expr) -> crate::ast::Expr {
        if let Expr::Grouping(grp) = expr {
            grp.accept(self)
        } else {
            panic!("ERROR")
        }
    }

    fn visit_var(&mut self, expr: &'a Expr) -> crate::ast::Expr {
        if let Expr::Var(name) = expr {
            match self.get(name.lexeme.as_str()) {
                Some(exp) => *exp,
                None => panic!("Not found"),
            }
        } else {
            panic!("ERROR")
        }
    }

    fn visit_assign(&mut self, expr: &'a Expr) -> crate::ast::Expr {
        if let Expr::Assign(name, value) = expr {
            let accepted_value = value.accept(self);
            let var_name: String = name.lexeme.to_owned();
            self.assign(var_name, Box::new(accepted_value));
        }
        expr.clone()
    }

    fn visit_logical(&mut self, expr: &'a Expr) -> crate::ast::Expr {
        if let Expr::Logical {
            left,
            operator,
            right,
        } = expr
        {
            let left_accepted = left.accept(self);
            let accepted = match operator {
                Operator::Or => {
                    if let Expr::Value(Type::Bool(false)) = left_accepted {
                        left_accepted
                    } else {
                        right.accept(self)
                    }
                }
                _ => {
                    if let Expr::Value(Type::Bool(false)) = left_accepted {
                        left_accepted
                    } else {
                        right.accept(self)
                    }
                }
            };

            accepted
        } else {
            panic!("ERROR")
        }
    }

    //fn visit_call(&mut self, expr: &'a Expr) -> crate::ast::Expr {
    //    if let Expr::Call { callee, paren, arguments } = expr {
    //        let callee_accepted = callee.accept(self);
    //        let args = arguments.iter().map(|arg| arg.accept(self)).collect();
    //
    //        expr
    //        //Callable::call(self, args)
    //    } else {
    //        panic!("ERROR")
    //    }
}
