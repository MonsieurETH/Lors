use core::panic;
use std::collections::HashMap;

use crate::ast::{
    Assign, Binary, Block, Expr, Expression, FunDecl, Function, Grouping, IVisitorExpr,
    IVisitorStmt, If, Literal, Logical, Print, Return, Stmt, Unary, Var, VarDecl, While,
};
use crate::operators::Operator;

#[derive(Debug, PartialEq)]
pub struct Environment {
    symbol_table: HashMap<String, Expr>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            symbol_table: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: &str, value: Expr) {
        self.symbol_table.insert(name.to_string(), value);
    }

    pub fn retrieve(&self, name: &str) -> Option<Expr> {
        self.symbol_table.get(name).cloned()
    }
}
pub struct Interpreter {
    environments: Vec<Environment>,
    actual_env_number: usize,
}
impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environments: vec![Environment::new()],
            actual_env_number: 0,
        }
    }

    pub fn new_environment(&mut self) {
        let env = Environment::new();
        self.environments.push(env);
        self.actual_env_number += 1;
    }

    pub fn get_env_number(&self) -> usize {
        self.actual_env_number
    }

    pub fn set_env_number(&mut self, env_number: usize) {
        self.actual_env_number = env_number;
    }

    //pub fn destroy_environment(&mut self) {
    //    self.environments.pop();
    //    self.actual_env_number -= 1;
    //}

    pub fn define_symbol(&mut self, name: &str, value: Expr) {
        self.environments.last_mut().unwrap().define(name, value);
    }

    pub fn get_symbol(&self, name: &str) -> Option<Expr> {
        for env in self.environments.iter().rev() {
            let symbol = env.retrieve(name);
            if symbol.is_some() {
                return symbol;
            }
        }
        None
    }

    pub fn assign_symbol(&mut self, name: &str, value: Expr) {
        for env in self.environments.iter_mut().rev() {
            let symbol = env.retrieve(name);
            if symbol.is_some() {
                env.define(name, value);
                break;
            }
        }
    }
}

impl IVisitorStmt<Option<Stmt>> for Interpreter {
    fn visit_expr(&mut self, stmt: &Stmt) -> Option<Stmt> {
        if let Stmt::Expression(Expression { expr }) = stmt {
            expr.accept(self);
        } else {
            panic!("ERROR")
        }
        None
    }

    fn visit_print(&mut self, stmt: &Stmt) -> Option<Stmt> {
        if let Stmt::Print(Print { expr }) = stmt {
            println!("{:?}", expr.accept(self));
        } else {
            panic!("ERROR")
        }
        None
    }

    fn visit_var_decl(&mut self, stmt: &Stmt) -> Option<Stmt> {
        match stmt {
            Stmt::VarDecl(VarDecl { name, expr }) => {
                let accepted_expr = expr.accept(self);
                self.define_symbol(name.as_str(), accepted_expr);
            }
            _ => panic!("ERROR"),
        }
        None
    }

    fn visit_if(&mut self, stmt: &Stmt) -> Option<Stmt> {
        if let Stmt::If(If {
            condition,
            branch_true,
            branch_false,
        }) = stmt
        {
            let eval_condition = condition.accept(self);
            if let Expr::Literal(Literal::Bool(b)) = eval_condition {
                if b {
                    _ = branch_true.accept(self)
                } else {
                    _ = branch_false.accept(self)
                }
            }
        }
        None
    }

    fn visit_while(&mut self, stmt: &Stmt) -> Option<Stmt> {
        if let Stmt::While(While { condition, body }) = stmt {
            let mut accepted_cond = condition.accept(self);
            while let Expr::Literal(Literal::Bool(true)) = accepted_cond {
                accepted_cond = condition.accept(self);
                _ = body.accept(self)
            }
        }
        None
    }

    fn visit_block(&mut self, stmt: &Stmt) -> Option<Stmt> {
        if let Stmt::Block(Block { stmts }) = stmt {
            self.execute_block(stmts, self.get_env_number());
            None
        } else {
            panic!("ERROR")
        }
    }

    fn execute_block(&mut self, stmts: &Vec<Stmt>, context: usize) -> Option<Stmt> {
        //let mut new = self.new_environment();
        //let old = context.clone();
        let actual_context = self.get_env_number();
        self.set_env_number(context);

        let mut result = None;
        for stmt in stmts {
            let accepted_stmt = stmt.accept(self);
            match accepted_stmt {
                Some(s) => {
                    result = Some(s);
                    break;
                }
                None => continue,
            }
        }

        self.set_env_number(actual_context);

        result
    }

    fn visit_fun_decl(&mut self, stmt: &Stmt) -> Option<Stmt> {
        if let Stmt::FunDecl(FunDecl {
            name,
            parameters: _,
            body: _,
        }) = stmt
        {
            let call: Function = Function::from_stmt(stmt.clone(), self.get_env_number());
            self.define_symbol(name.as_str(), Expr::Function(call));
        }
        None
    }

    fn visit_return(&mut self, stmt: &Stmt) -> Option<Stmt> {
        if let Stmt::Return(Return { keyword, value }) = stmt {
            let val = match value {
                Expr::Literal(Literal::Nil) => Expr::Literal(Literal::Nil),
                _ => value.accept(self),
            };

            Some(Stmt::Return(Return {
                keyword: keyword.clone(),
                value: val,
            }))
        } else {
            panic!("ERROR")
        }
    }
}

impl IVisitorExpr<crate::ast::Expr> for Interpreter {
    fn visit_literal(&mut self, expr: &Expr) -> crate::ast::Expr {
        expr.clone()
    }

    fn visit_unary(&mut self, expr: &Expr) -> crate::ast::Expr {
        if let Expr::Unary(Unary { operator, right }) = expr {
            let accepted_right = right.accept(self);
            operator.clone().unary(accepted_right)
        } else {
            panic!("ERROR")
        }
    }

    fn visit_binary(&mut self, expr: &Expr) -> crate::ast::Expr {
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

    fn visit_grouping(&mut self, expr: &Expr) -> crate::ast::Expr {
        if let Expr::Grouping(Grouping { group }) = expr {
            group.accept(self)
        } else {
            panic!("ERROR")
        }
    }

    fn visit_var(&mut self, expr: &Expr) -> crate::ast::Expr {
        if let Expr::Var(Var::Token(name)) = expr {
            match self.get_symbol(name.lexeme.as_str()) {
                Some(exp) => exp,
                None => panic!("Not found"),
            }
        } else {
            panic!("ERROR")
        }
    }

    fn visit_assign(&mut self, expr: &Expr) -> crate::ast::Expr {
        if let Expr::Assign(Assign { var, expr }) = expr {
            let accepted_expr = expr.accept(self);
            let Var::Token(token) = var;
            let var_name: String = token.lexeme.to_owned();
            self.assign_symbol(var_name.as_str(), accepted_expr);
        }
        expr.clone()
    }

    fn visit_logical(&mut self, expr: &Expr) -> crate::ast::Expr {
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

    fn visit_call(self: &mut Interpreter, expr: &Expr) -> crate::ast::Expr {
        if let Expr::Call(call) = expr {
            let callee_accepted = call.callee.accept(self);
            let args: Vec<Expr> = call.arguments.iter().map(|arg| arg.accept(self)).collect();

            match callee_accepted {
                Expr::Function(fun) => {
                    let Function {
                        name: _,
                        parameters,
                        body: _,
                        context: _,
                    } = &fun;

                    if args.len() != parameters.len() {
                        panic!("ERROR")
                    };

                    return fun.execute_call(self, args);
                }
                _ => panic!("ERROR"),
            }
        } else {
            panic!("ERROR")
        }
    }
}
