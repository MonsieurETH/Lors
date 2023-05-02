use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use crate::ast::{
    Assign, Binary, Block, Class, ClassDecl, Error, Expr, Expression, FunDecl, Function, Get,
    Grouping, IVisitorExpr, IVisitorStmt, If, Literal, Logical, Print, Return, Set, Stmt, This,
    Unary, Var, VarDecl, While,
};
use crate::operators::Operator;

#[derive(Debug, PartialEq, Clone, PartialOrd, Ord, Eq)]
pub struct Environment {
    symbol_table: BTreeMap<String, Expr>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            symbol_table: BTreeMap::new(),
            enclosing: None,
        }
    }

    pub fn new_with_enclosing(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
        Environment {
            symbol_table: BTreeMap::new(),
            enclosing,
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
    environments: Option<Rc<RefCell<Environment>>>,
    locals: BTreeMap<Expr, usize>,
    globals: BTreeMap<String, Expr>,
    counter: usize,
}

pub struct EnvironmentIterator<'a> {
    interpreter: &'a Interpreter,
    pos: usize,
}

impl<'a> Iterator for EnvironmentIterator<'a> {
    type Item = Rc<RefCell<Environment>>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut env = self.interpreter.get_actual_env();
        if self.pos == 0 {
            return env;
        }
        for _ in 0..self.pos {
            env = env?.borrow().enclosing.as_ref().map(|e| Rc::clone(e));
        }

        self.pos -= 1;

        match env {
            None => None,
            Some(e) => Some(e),
        }
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environments: Some(Rc::new(RefCell::new(Environment::new()))),
            locals: BTreeMap::new(),
            globals: BTreeMap::new(),
            counter: 1,
            //actual_env: None,
        }
    }

    pub fn iterator(&self) -> EnvironmentIterator {
        EnvironmentIterator {
            interpreter: self,
            pos: self.counter - 1,
        }
    }

    pub fn new_environment(&mut self) {
        let env = Environment::new_with_enclosing(self.get_actual_env());
        self.environments = Some(Rc::new(RefCell::new(env)));
        self.counter += 1;
    }

    pub fn new_environment_with_enclosing(&mut self, enclosing: Option<Rc<RefCell<Environment>>>) {
        let env = Environment::new_with_enclosing(enclosing);
        self.environments = Some(Rc::new(RefCell::new(env)));
        self.counter += 1;
    }

    pub fn get_actual_env(&self) -> Option<Rc<RefCell<Environment>>> {
        Some(Rc::clone(self.environments.as_ref().unwrap()))
    }

    pub fn set_environment(&mut self, env: Option<Rc<RefCell<Environment>>>) {
        self.environments = env;
    }

    pub fn drop_environment(&mut self) {
        let enclosing = Some(Rc::clone(
            self.environments
                .as_ref()
                .unwrap()
                .borrow()
                .enclosing
                .as_ref()
                .unwrap(),
        ));
        self.environments = enclosing;
    }

    pub fn define_symbol(&mut self, name: &str, value: Expr) {
        self.environments
            .as_ref()
            .unwrap()
            .borrow_mut()
            .define(name, value);
    }

    pub fn get_symbol_at(&self, mut pos: usize, name: &str) -> Result<Option<Expr>, Error> {
        //if pos < 0 {
        //    return Err(Error::new("Invalid position".to_string()));
        //}
        for env in self.iterator() {
            if pos == 0 {
                let symbol = env.borrow().retrieve(name);
                if symbol.is_some() {
                    return Ok(symbol);
                }
            }
            pos -= 1;
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

    pub fn assign_symbol_at(
        &mut self,
        mut pos: usize,
        name: &str,
        value: Expr,
    ) -> Result<Option<Expr>, Error> {
        if pos < 0 {
            return Err(Error::new("Invalid position".to_string()));
        }
        for env in self.iterator() {
            if pos == 0 {
                env.borrow_mut().define(name, value);
                break;
            }
            pos -= 1;
        }

        Ok(None)
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

    pub fn check_symbol(&self, name: &str) -> bool {
        for env in self.iterator() {
            let symbol = env.borrow().retrieve(name);
            if symbol.is_some() {
                return true;
            }
        }
        false
    }

    fn lookup_symbol(&self, name: &str, expr: Expr) -> Result<Option<Expr>, Error> {
        //let hash = Self::calculate_hash(&expr);
        let distance = self.locals.get(&expr);

        match distance {
            Some(distance) => self.get_symbol_at(*distance, name),
            None => Ok(self.globals.get(name).cloned()),
        }
    }

    //pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    //    let mut s = DefaultHasher::new();
    //    t.hash(&mut s);
    //    s.finish()
    //}

    pub fn resolve(&mut self, expr: &Expr, depth: usize) {
        //let hash = Self::calculate_hash(expr);
        self.locals.insert(expr.clone(), depth);
    }

    pub fn execute_block(
        &mut self,
        stmts: &Vec<Stmt>,
        env: Option<Rc<RefCell<Environment>>>,
    ) -> Result<Option<Stmt>, Error> {
        let actual_env = self.get_actual_env();
        self.set_environment(env);

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

        self.set_environment(actual_env);

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
            self.execute_block(stmts, self.get_actual_env())?;
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
            let call: Function = Function::from_stmt(stmt.clone(), self.get_actual_env());
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
        if let Stmt::ClassDecl(ClassDecl { name, methods }) = stmt {
            let mut meths = BTreeMap::new();
            for method in methods {
                let fun: Function = Function::from_stmt(method.clone(), self.get_actual_env());
                meths.insert(fun.name.clone(), fun);
            }
            let class: Class = Class {
                name: name.lexeme.clone(),
                methods: meths,
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

            if self.check_symbol(&var_name) {
                let accepted_expr = expr.accept(self).unwrap().unwrap();

                //let hash = Self::calculate_hash(&accepted_expr);
                let distance = self.locals.get(&accepted_expr);

                match distance {
                    Some(distance) => {
                        self.assign_symbol_at(*distance, var_name.as_str(), accepted_expr.clone())?;
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
                Expr::Class(class) => Ok(Some(class.execute_call())), //Ok(Some(class.execute_call(self, args))),
                _ => Err(Error::new("Invalid call".to_string())),
            }
        } else {
            Err(Error::new("Invalid statement".to_string()))
        }
    }

    fn visit_get(&mut self, expr: &Expr) -> Result<Option<Expr>, Error> {
        if let Expr::Get(Get { object, name }) = expr {
            let accepted_object = object.accept(self).unwrap().unwrap();
            match accepted_object {
                Expr::Instance(instance) => Ok(Some(instance.get_field(name.lexeme.as_str())?)),
                _ => Err(Error::new("Only instances have properties.".to_string())),
            }
        } else {
            Err(Error::new("Invalid statement".to_string()))
        }
    }

    fn visit_set(&mut self, expr: &Expr) -> Result<Option<Expr>, Error> {
        if let Expr::Set(Set {
            object,
            name,
            value,
        }) = expr
        {
            let accepted_object = object.accept(self).unwrap().unwrap();
            match accepted_object {
                Expr::Instance(mut instance) => {
                    let value = value.accept(self).unwrap().unwrap();
                    instance.set_field(&name.lexeme, value);
                    Ok(None)
                }
                _ => Err(Error::new("Only instances have fields.".to_string())),
            }
        } else {
            Err(Error::new("Invalid statement".to_string()))
        }
    }

    fn visit_this(&mut self, expr: &Expr) -> Result<Option<Expr>, Error> {
        if let Expr::This(This { keyword }) = expr {
            self.lookup_symbol(&keyword.lexeme, expr.clone())?;
            Ok(None)
        } else {
            Err(Error::new("Invalid statement".to_string()))
        }
    }
}
