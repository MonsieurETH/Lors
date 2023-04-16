use core::panic;
use std::collections::HashMap;

use crate::ast::{
    Assign, Binary, Block, Expr, Expression, FunDecl, Function, Grouping, IVisitorExpr,
    IVisitorStmt, If, Literal, Logical, Print, Stmt, Unary, Var, VarDecl, While,
};
use crate::operators::Operator;

#[derive(Debug)]
pub struct Environment {
    pub enclosing: Option<Box<Environment>>,
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

impl Environment {
    pub fn get(&self, symbol: &str) -> Option<Box<Expr>> {
        while let Some(expr) = self.symbol_table.get(symbol) {
            return Some(expr.clone());
        }
        let mut current_env = self;
        while let Some(enclosing) = current_env.enclosing.as_ref() {
            current_env = enclosing;
            if let Some(expr) = current_env.symbol_table.get(symbol) {
                return Some(expr.clone());
            }
        }
        None
    }

    pub fn define(&mut self, symbol: String, expr: Box<Expr>) {
        self.symbol_table.insert(symbol, expr);
    }

    // This function is pretty awful, and separating 'define' & 'assign'
    // looks like a bad idea in the long term. (check & rewrite)"
    pub fn assign(&mut self, symbol: String, expr: Box<Expr>) {
        let mut current_env = self;
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
}

impl<'a> IVisitorStmt<'a, ()> for Interpreter {
    fn visit_expr(&mut self, stmt: &'a Stmt) {
        if let Stmt::Expression(Expression { expr }) = stmt {
            expr.accept(self);
        } else {
            panic!("ERROR")
        }
    }

    fn visit_print(&mut self, stmt: &'a Stmt) {
        if let Stmt::Print(Print { expr }) = stmt {
            println!("{:?}", expr.accept(self));
        } else {
            panic!("ERROR")
        }
    }

    fn visit_var_decl(&mut self, stmt: &'a Stmt) {
        match stmt {
            Stmt::VarDecl(VarDecl { name, expr }) => {
                let accepted_expr = expr.accept(self);
                self.env.define(name.to_owned(), Box::new(accepted_expr));
            }
            _ => panic!("ERROR"),
        }
    }

    fn visit_if(&mut self, stmt: &'a Stmt) {
        if let Stmt::If(If {
            condition,
            branch_true,
            branch_false,
        }) = stmt
        {
            let eval_condition = condition.accept(self);
            if let Expr::Literal(Literal::Bool(b)) = eval_condition {
                if b {
                    branch_true.accept(self)
                } else {
                    branch_false.accept(self)
                }
            }
        }
    }

    fn visit_while(&mut self, stmt: &'a Stmt) {
        if let Stmt::While(While { condition, body }) = stmt {
            let mut accepted_cond = condition.accept(self);
            while let Expr::Literal(Literal::Bool(true)) = accepted_cond {
                accepted_cond = condition.accept(self);
                body.accept(self)
            }
        }
    }

    fn visit_block(&mut self, stmt: &'a Stmt) {
        if let Stmt::Block(Block { stmts }) = stmt {
            self.execute_block(stmts, self.env.clone());
        } else {
            panic!("ERROR")
        }
    }

    fn execute_block(&mut self, stmts: &Vec<Stmt>, env: Environment) {
        let symbol_table = HashMap::new();
        let mut new: Environment = Environment {
            enclosing: Some(Box::new(env)),
            symbol_table,
        };
        _ = std::mem::swap(&mut self.env, &mut new);

        for stmt in stmts {
            stmt.accept(self)
        }

        _ = std::mem::swap(self.env.enclosing.clone().unwrap().as_mut(), &mut self.env);
    }

    fn visit_fun_decl(&mut self, stmt: &'a Stmt) {
        if let Stmt::FunDecl(FunDecl {
            name,
            parameters: _,
            body: _,
        }) = stmt
        {
            let call: Function = Function::from(stmt.clone());
            self.env
                .define(name.clone(), Box::new(Expr::Function(call)));
        }
    }
}

impl<'a> IVisitorExpr<'a, crate::ast::Expr> for Interpreter {
    fn visit_literal(&mut self, expr: &'a Expr) -> crate::ast::Expr {
        expr.clone()
    }

    fn visit_unary(&mut self, expr: &'a Expr) -> crate::ast::Expr {
        if let Expr::Unary(Unary { operator, right }) = expr {
            let accepted_right = right.accept(self);
            operator.clone().unary(accepted_right)
        } else {
            panic!("ERROR")
        }
    }

    fn visit_binary(&mut self, expr: &'a Expr) -> crate::ast::Expr {
        if let Expr::Binary(Binary {
            left,
            operator,
            right,
        }) = expr
        {
            let accepted_left = left.accept(self);
            let accepted_right = right.accept(self);
            operator.clone().binary(accepted_left, accepted_right)
        } else {
            panic!("ERROR")
        }
    }

    fn visit_grouping(&mut self, expr: &'a Expr) -> crate::ast::Expr {
        if let Expr::Grouping(Grouping { group }) = expr {
            group.accept(self)
        } else {
            panic!("ERROR")
        }
    }

    fn visit_var(&mut self, expr: &'a Expr) -> crate::ast::Expr {
        if let Expr::Var(Var::Token(name)) = expr {
            match self.env.get(name.lexeme.as_str()) {
                Some(exp) => *exp,
                None => panic!("Not found"),
            }
        } else {
            panic!("ERROR")
        }
    }

    fn visit_assign(&mut self, expr: &'a Expr) -> crate::ast::Expr {
        if let Expr::Assign(Assign { var, expr }) = expr {
            let accepted_expr = expr.accept(self);
            let Var::Token(token) = var;
            let var_name: String = token.lexeme.to_owned();
            self.env.assign(var_name, Box::new(accepted_expr));
        }
        expr.clone()
    }

    fn visit_logical(&mut self, expr: &'a Expr) -> crate::ast::Expr {
        if let Expr::Logical(Logical {
            left,
            operator,
            right,
        }) = expr
        {
            let left_accepted = left.accept(self);
            let accepted = match operator {
                Operator::Or => {
                    if let Expr::Literal(Literal::Bool(false)) = left_accepted {
                        left_accepted
                    } else {
                        right.accept(self)
                    }
                }
                _ => {
                    if let Expr::Literal(Literal::Bool(false)) = left_accepted {
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

    fn visit_call(self: &mut Interpreter, expr: &'a Expr) -> crate::ast::Expr {
        if let Expr::Call(call) = expr {
            let callee_accepted = call.callee.accept(self);
            let args: Vec<Expr> = call.arguments.iter().map(|arg| arg.accept(self)).collect();

            match callee_accepted {
                Expr::Function(fun) => {
                    let Function {
                        name: _,
                        parameters,
                        body: _,
                    } = &fun;

                    if args.len() != parameters.len() {
                        panic!("ERROR")
                    };

                    fun.execute_call(self, args);
                    return Expr::Literal(Literal::Nil);
                }
                _ => panic!("ERROR"),
            }
        } else {
            panic!("ERROR")
        }
    }
}
