use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::{hash::Hash, hash::Hasher};

use crate::ast::{
    Assign, Binary, Block, Class, ClassDecl, Error, Expr, Expression, FunDecl, Function, Grouping,
    IVisitorExpr, IVisitorStmt, If, Literal, Logical, Print, Return, Stmt, Unary, Var, VarDecl,
    While,
};
use crate::operators::Operator;

#[derive(Debug, PartialEq, Clone)]
pub struct Environment {
    symbol_table: HashMap<String, Expr>,
    parent: Option<usize>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            symbol_table: HashMap::new(),
            parent: None,
        }
    }

    pub fn new_with_parent(parent: usize) -> Self {
        Environment {
            symbol_table: HashMap::new(),
            parent: Some(parent),
        }
    }

    pub fn define(&mut self, name: &str, value: Expr) {
        self.symbol_table.insert(name.to_string(), value);
    }

    pub fn retrieve(&self, name: &str) -> Option<Expr> {
        self.symbol_table.get(name).cloned()
    }
}

#[derive(Debug, PartialEq)]
pub struct Interpreter {
    environments: Vec<Environment>,
    locals: HashMap<u64, usize>,
    globals: HashMap<String, Expr>,
    actual_env_number: usize,
}
impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environments: vec![Environment::new()],
            locals: HashMap::new(),
            globals: HashMap::new(),
            actual_env_number: 0,
        }
    }

    pub fn new_environment(&mut self) {
        let env = Environment::new_with_parent(self.actual_env_number);
        self.environments.push(env);
        self.actual_env_number = self.environments.len() - 1;
    }

    pub fn new_environment_with_parent(&mut self, parent_env_number: usize) {
        let env = Environment::new_with_parent(parent_env_number);
        self.environments.push(env);
        self.actual_env_number = self.environments.len() - 1;
    }

    pub fn get_env_number(&self) -> usize {
        self.actual_env_number
    }

    pub fn set_env_number(&mut self, env_number: usize) {
        self.actual_env_number = env_number;
    }

    pub fn drop_environment(&mut self) {
        self.environments.pop();
        self.actual_env_number -= 1;
    }

    pub fn define_symbol(&mut self, name: &str, value: Expr) {
        self.environments
            .get_mut(self.actual_env_number)
            .unwrap()
            .define(name, value);
    }

    pub fn get_symbol_at(&self, pos: usize, name: &str) -> Result<Option<Expr>, Error> {
        let env = self
            .environments
            .iter()
            .nth(self.actual_env_number - pos) // BAD INDEX
            .unwrap();
        let symbol = env.retrieve(name);
        if symbol.is_some() {
            return Ok(symbol);
        }
        Ok(None)
    }

    /*pub fn get_symbol(&self, name: &str) -> Result<Option<Expr>, Error> {
        for env in self.environments.iter().rev() {
            let symbol = env.retrieve(name);
            if symbol.is_some() {
                return Ok(symbol);
            }
        }
        Ok(None)
    }*/

    pub fn assign_symbol_at(&mut self, pos: usize, name: &str, value: Expr) {
        let env = self
            .environments
            .iter_mut()
            .nth(self.actual_env_number - pos)
            .unwrap();
        env.define(name, value);
    }

    /*pub fn assign_symbol(&mut self, name: &str, value: Expr) {
        for env in self.environments.iter_mut().rev() {
            let symbol = env.retrieve(name);
            if symbol.is_some() {
                env.define(name, value);
                break;
            }
        }
    }*/

    pub fn check_symbol(&self, name: &str, _env: usize) -> bool {
        let rev = self.environments.iter().rev();
        for env in rev {
            let symbol = env.retrieve(name);
            if symbol.is_some() {
                return true;
            }
        }
        false
    }

    fn lookup_symbol(&self, name: &str, expr: Expr) -> Result<Option<Expr>, Error> {
        let hash = Self::calculate_hash(&expr);
        let distance = self.locals.get(&hash);

        match distance {
            Some(distance) => self.get_symbol_at(*distance, name),
            None => Ok(self.globals.get(name).cloned()),
        }
    }

    pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }

    pub fn resolve(&mut self, expr: &mut Expr, depth: usize) {
        let hash = Self::calculate_hash(expr);
        self.locals.insert(hash, depth);
    }

    pub fn execute_block_context(
        &mut self,
        stmts: &Vec<Stmt>,
        context: usize,
    ) -> Result<Option<Stmt>, Error> {
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
}

impl IVisitorStmt<Result<Option<Stmt>, Error>> for Interpreter {
    fn visit_expr(&mut self, stmt: &Stmt) -> Result<Option<Stmt>, Error> {
        if let Stmt::Expression(Expression { expr }) = stmt {
            match expr.accept(self) {
                Ok(_) => Ok(None),
                Err(e) => Err(e),
            }
        } else {
            Err(Error::new("Invalid statement".to_string()))
        }
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
                let accepted_expr = expr.accept(self)?;
                self.define_symbol(name.as_str(), accepted_expr.unwrap());
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
            if let Some(Expr::Literal(Literal::Bool(b))) = eval_condition {
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
            while let Some(Expr::Literal(Literal::Bool(true))) = accepted_cond {
                accepted_cond = condition.accept(self).unwrap();
                _ = body.accept(self)
            }
        }
        Ok(None)
    }

    fn visit_block(&mut self, stmt: &Stmt) -> Result<Option<Stmt>, Error> {
        if let Stmt::Block(Block { stmts }) = stmt {
            self.new_environment();
            self.execute_block_context(stmts, self.get_env_number())?;
            self.drop_environment();
            Ok(None)
        } else {
            Err(Error::new("Invalid statement".to_string()))
        }
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
                _ => value.accept(self).unwrap().unwrap(),
            };

            Ok(Some(Stmt::Return(Return {
                keyword: keyword.clone(),
                value: val,
            })))
        } else {
            Err(Error::new("Invalid statement".to_string()))
        }
    }

    fn visit_class(&mut self, stmt: &Stmt) -> Result<Option<Stmt>, Error> {
        if let Stmt::ClassDecl(ClassDecl {
            name,
            methods: _methods,
        }) = stmt
        {
            let class: Class = Class {
                name: name.lexeme.clone(),
                //methods,
            };
            self.define_symbol(&name.lexeme.as_str(), Expr::Class(class));

            Ok(None)
        } else {
            Err(Error::new("Invalid statement".to_string()))
        }
    }
}

impl IVisitorExpr<Result<Option<Expr>, Error>> for Interpreter {
    fn visit_literal(&mut self, expr: &Expr) -> Result<Option<Expr>, Error> {
        Ok(Some(expr.clone()))
    }

    fn visit_unary(&mut self, expr: &Expr) -> Result<Option<Expr>, Error> {
        if let Expr::Unary(Unary { operator, right }) = expr {
            let accepted_right = right.accept(self).unwrap().unwrap();
            operator.clone().unary(accepted_right)
        } else {
            Err(Error::new("Invalid expression".to_string()))
        }
    }

    fn visit_binary(&mut self, expr: &Expr) -> Result<Option<Expr>, Error> {
        if let Expr::Binary(Binary {
            left,
            operator,
            right,
        }) = expr
        {
            let accepted_left = left.accept(self).unwrap().unwrap();
            let accepted_right = right.accept(self).unwrap().unwrap();
            operator.clone().binary(accepted_left, accepted_right)
        } else {
            Err(Error::new("Invalid expression".to_string()))
        }
    }

    fn visit_grouping(&mut self, expr: &Expr) -> Result<Option<Expr>, Error> {
        if let Expr::Grouping(Grouping { group }) = expr {
            group.accept(self)
        } else {
            Err(Error::new("Invalid expression".to_string()))
        }
    }

    fn visit_var(&mut self, expr: &Expr) -> Result<Option<Expr>, Error> {
        if let Expr::Var(Var::Token(name)) = expr {
            self.lookup_symbol(name.lexeme.as_str(), expr.clone())
        } else {
            Err(Error::new("Invalid expression".to_string()))
        }
    }

    fn visit_assign(&mut self, expr: &Expr) -> Result<Option<Expr>, Error> {
        if let Expr::Assign(Assign { var, expr }) = expr {
            let Var::Token(token) = var;
            let var_name: String = token.lexeme.to_owned();

            if self.check_symbol(&var_name, self.actual_env_number) {
                let accepted_expr = expr.accept(self).unwrap().unwrap();

                let hash = Self::calculate_hash(&accepted_expr);
                let distance = self.locals.get(&hash);

                match distance {
                    Some(distance) => {
                        self.assign_symbol_at(*distance, var_name.as_str(), accepted_expr.clone())
                    }
                    None => {
                        self.globals.insert(var_name, accepted_expr.clone());
                    }
                }

                Ok(Some(accepted_expr))
            } else {
                Err(Error::new(format!("Undefined variable '{}'.", var_name)))
            }
        } else {
            Err(Error::new("Invalid expression".to_string()))
        }
    }

    fn visit_logical(&mut self, expr: &Expr) -> Result<Option<Expr>, Error> {
        if let Expr::Logical(Logical {
            left,
            operator,
            right,
        }) = expr
        {
            let left_accepted = left.accept(self).unwrap();
            let accepted = match operator {
                Operator::Or => {
                    if let Some(Expr::Literal(Literal::Bool(false))) = left_accepted {
                        left_accepted
                    } else {
                        right.accept(self).unwrap()
                    }
                }
                _ => {
                    if let Some(Expr::Literal(Literal::Bool(false))) = left_accepted {
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

    fn visit_call(self: &mut Interpreter, expr: &Expr) -> Result<Option<Expr>, Error> {
        if let Expr::Call(call) = expr {
            let callee_accepted = call.callee.accept(self).unwrap().unwrap();
            let args: Vec<Expr> = call
                .arguments
                .iter()
                .map(|arg| arg.accept(self).unwrap().unwrap())
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
                        Ok(Some(fun.execute_call(self, args)))
                    }
                }
                Expr::Class(class) => {
                    let Class { name: _name } = &class;
                    Ok(Some(class.execute_call(self, args)))
                }
                _ => Err(Error::new("Invalid call".to_string())),
            }
        } else {
            Err(Error::new("Invalid statement".to_string()))
        }
    }
}
