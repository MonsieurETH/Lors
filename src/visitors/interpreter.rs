use std::collections::HashMap;

use crate::ast::{
    Assign, Binary, Block, Error, Expr, Expression, FunDecl, Function, Grouping, IVisitorExpr,
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

    pub fn new_environment(&mut self) -> usize {
        let env = Environment::new();
        self.environments.push(env);
        self.environments.len() - 1
    }

    pub fn get_env_number(&self) -> usize {
        self.actual_env_number
    }

    pub fn set_env_number(&mut self, env_number: usize) {
        self.actual_env_number = env_number;
    }

    pub fn destroy_environment(&mut self, pos: usize) {
        self.environments.remove(pos);
        //self.actual_env_number -= 1;
    }

    pub fn define_symbol(&mut self, name: &str, value: Expr) {
        self.environments
            .get_mut(self.actual_env_number)
            .unwrap()
            .define(name, value);
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

    pub fn check_symbol(&self, name: &str) -> bool {
        for env in self.environments.iter().rev() {
            let symbol = env.retrieve(name);
            if symbol.is_some() {
                true;
            }
        }
        false
    }
}

impl IVisitorStmt<Result<Option<Stmt>, Error>> for Interpreter {
    fn visit_expr(&mut self, stmt: &Stmt) -> Result<Option<Stmt>, Error> {
        if let Stmt::Expression(Expression { expr }) = stmt {
            match expr.accept(self) {
                Ok(_) => Ok(None),
                Err(e) => Err(e),
            }

            //Ok(None)
        } else {
            Err(Error::new("Invalid statement".to_string()))
        }
        //Ok(None)
    }

    fn visit_print(&mut self, stmt: &Stmt) -> Result<Option<Stmt>, Error> {
        match stmt {
            Stmt::Print(Print { expr }) => {
                let pv = expr.accept(self)?;
                println!("{:?}", pv);
                Ok(None)
            }
            _ => Err(Error::new("Invalid statement".to_string())),
        }
    }

    fn visit_var_decl(&mut self, stmt: &Stmt) -> Result<Option<Stmt>, Error> {
        match stmt {
            Stmt::VarDecl(VarDecl { name, expr }) => {
                let accepted_expr = expr.accept(self).unwrap();
                self.define_symbol(name.as_str(), accepted_expr);
                Ok(None)
            }
            _ => Err(Error::new("Invalid statement".to_string())),
        }
    }

    fn visit_if(&mut self, stmt: &Stmt) -> Result<Option<Stmt>, Error> {
        if let Stmt::If(If {
            condition,
            branch_true,
            branch_false,
        }) = stmt
        {
            let eval_condition = condition.accept(self).unwrap();
            if let Expr::Literal(Literal::Bool(b)) = eval_condition {
                if b {
                    _ = branch_true.accept(self)
                } else {
                    _ = branch_false.accept(self)
                }
            }
        }
        Ok(None)
    }

    fn visit_while(&mut self, stmt: &Stmt) -> Result<Option<Stmt>, Error> {
        if let Stmt::While(While { condition, body }) = stmt {
            let mut accepted_cond = condition.accept(self).unwrap();
            while let Expr::Literal(Literal::Bool(true)) = accepted_cond {
                accepted_cond = condition.accept(self).unwrap();
                _ = body.accept(self)
            }
        }
        Ok(None)
    }

    fn visit_block(&mut self, stmt: &Stmt) -> Result<Option<Stmt>, Error> {
        if let Stmt::Block(Block { stmts }) = stmt {
            let new_env = self.new_environment();
            self.execute_block(stmts, new_env)?;
            self.destroy_environment(new_env);
            Ok(None)
        } else {
            Err(Error::new("Invalid statement".to_string()))
        }
    }

    fn execute_block(&mut self, stmts: &Vec<Stmt>, context: usize) -> Result<Option<Stmt>, Error> {
        let actual_context = self.get_env_number();
        self.set_env_number(context);

        let mut result = None;
        for stmt in stmts {
            let accepted_stmt = stmt.accept(self).unwrap();
            match accepted_stmt {
                Some(s) => {
                    result = Some(s);
                    break;
                }
                None => continue,
            }
        }

        self.set_env_number(actual_context);

        Ok(result)
    }

    fn visit_fun_decl(&mut self, stmt: &Stmt) -> Result<Option<Stmt>, Error> {
        if let Stmt::FunDecl(FunDecl {
            name,
            parameters: _,
            body: _,
        }) = stmt
        {
            let call: Function = Function::from_stmt(stmt.clone(), self.get_env_number());
            self.define_symbol(name.as_str(), Expr::Function(call));
        }
        Ok(None)
    }

    fn visit_return(&mut self, stmt: &Stmt) -> Result<Option<Stmt>, Error> {
        if let Stmt::Return(Return { keyword, value }) = stmt {
            let val = match value {
                Expr::Literal(Literal::Nil) => Expr::Literal(Literal::Nil),
                _ => value.accept(self).unwrap(),
            };

            Ok(Some(Stmt::Return(Return {
                keyword: keyword.clone(),
                value: val,
            })))
        } else {
            Err(Error::new("Invalid statement".to_string()))
        }
    }
}

impl IVisitorExpr<Result<Expr, Error>> for Interpreter {
    fn visit_literal(&mut self, expr: &Expr) -> Result<Expr, Error> {
        Ok(expr.clone())
    }

    fn visit_unary(&mut self, expr: &Expr) -> Result<Expr, Error> {
        if let Expr::Unary(Unary { operator, right }) = expr {
            let accepted_right = right.accept(self).unwrap();
            Ok(operator.clone().unary(accepted_right))
        } else {
            Err(Error::new("Invalid expression".to_string()))
        }
    }

    fn visit_binary(&mut self, expr: &Expr) -> Result<Expr, Error> {
        if let Expr::Binary(Binary {
            left,
            operator,
            right,
        }) = expr
        {
            let accepted_left = left.accept(self).unwrap();
            let accepted_right = right.accept(self).unwrap();
            operator.clone().binary(accepted_left, accepted_right)
        } else {
            Err(Error::new("Invalid expression".to_string()))
        }
    }

    fn visit_grouping(&mut self, expr: &Expr) -> Result<Expr, Error> {
        if let Expr::Grouping(Grouping { group }) = expr {
            group.accept(self)
        } else {
            Err(Error::new("Invalid expression".to_string()))
        }
    }

    fn visit_var(&mut self, expr: &Expr) -> Result<Expr, Error> {
        if let Expr::Var(Var::Token(name)) = expr {
            match self.get_symbol(name.lexeme.as_str()) {
                Some(exp) => Ok(exp),
                None => Err(Error::new(format!(
                    "Symbol {} not found",
                    name.lexeme.as_str(),
                ))),
            }
        } else {
            Err(Error::new("Invalid expression".to_string()))
        }
    }

    fn visit_assign(&mut self, expr: &Expr) -> Result<Expr, Error> {
        if let Expr::Assign(Assign { var, expr }) = expr {
            let Var::Token(token) = var;
            let var_name: String = token.lexeme.to_owned();
            if self.check_symbol(&var_name) {
                let accepted_expr = expr.accept(self)?;
                self.assign_symbol(var_name.as_str(), accepted_expr.clone());
                Ok(accepted_expr)
            } else {
                Err(Error::new(format!("Undefined variable '{}'.", var_name)))
            }
        } else {
            Err(Error::new("Invalid expression".to_string()))
        }
    }

    fn visit_logical(&mut self, expr: &Expr) -> Result<Expr, Error> {
        if let Expr::Logical(Logical {
            left,
            operator,
            right,
        }) = expr
        {
            let left_accepted = left.accept(self).unwrap();
            let accepted = match operator {
                Operator::Or => {
                    if let Expr::Literal(Literal::Bool(false)) = left_accepted {
                        left_accepted
                    } else {
                        right.accept(self).unwrap()
                    }
                }
                _ => {
                    if let Expr::Literal(Literal::Bool(false)) = left_accepted {
                        left_accepted
                    } else {
                        right.accept(self).unwrap()
                    }
                }
            };

            Ok(accepted)
        } else {
            Err(Error::new("Invalid statement".to_string()))
        }
    }

    fn visit_call(self: &mut Interpreter, expr: &Expr) -> Result<Expr, Error> {
        if let Expr::Call(call) = expr {
            let callee_accepted = call.callee.accept(self).unwrap();
            let args: Vec<Expr> = call
                .arguments
                .iter()
                .map(|arg| arg.accept(self).unwrap())
                .collect();

            match callee_accepted {
                Expr::Function(fun) => {
                    let Function {
                        name,
                        parameters,
                        body: _,
                        context: _,
                    } = &fun;

                    if args.len() != parameters.len() {
                        Err(Error::new(format!(
                            "Invalid number of arguments (got {}, expected {}) in {} call",
                            args.len(),
                            parameters.len(),
                            name
                        )))
                    } else {
                        Ok(fun.execute_call(self, args))
                    }
                }
                _ => Err(Error::new("Invalid call".to_string())),
            }
        } else {
            Err(Error::new("Invalid statement".to_string()))
        }
    }
}
