use crate::ast::{Expr, IVisitorExpr, IVisitorStmt, Stmt};
use crate::parser::Type;

pub struct AstPrinter {}

impl<'a> IVisitorStmt<'a, String> for AstPrinter {
    fn visit_stmt_expr(&mut self, stmt: &'a Stmt) -> String {
        todo!()
    }

    fn visit_stmt_print(&mut self, stmt: &'a Stmt) -> String {
        todo!()
    }

    fn visit_var_decl(&mut self, stmt: &'a Stmt) -> String {
        todo!()
    }

    fn visit_block(&mut self, stmt: &'a Stmt) -> String {
        todo!()
    }

    fn execute_block(&mut self, stmts: &Vec<Stmt>) {
        todo!()
    }

    fn visit_if(&mut self, stmt: &'a Stmt) -> String {
        todo!()
    }

    fn visit_while(&mut self, stmt: &'a Stmt) -> String {
        todo!()
    }
}

impl<'a> IVisitorExpr<'a, String> for AstPrinter {
    fn visit_value(self: &mut AstPrinter, expr: &'a Expr) -> String {
        match expr {
            Expr::Value(Type::Str(str)) => str.to_owned(),
            Expr::Value(Type::Bool(b)) => {
                if *b {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
            Expr::Value(Type::Number(n)) => format!("{}", n),
            Expr::Value(Type::Nil) => "nil".to_string(),
            _ => panic!("Error"),
        }
    }

    fn visit_unary(self: &mut AstPrinter, expr: &'a Expr) -> String {
        if let Expr::Unary { operator, right } = expr {
            let ans = "(".to_owned() + &operator.to_string() + &right.accept(self);
            ans
        } else {
            panic!("ERROR")
        }
    }

    fn visit_binary(self: &mut AstPrinter, expr: &'a Expr) -> String {
        if let Expr::Binary {
            left,
            operator,
            right,
        } = expr
        {
            let ans = "(".to_owned()
                + &left.accept(self)
                + &operator.to_string()
                + &right.accept(self)
                + ")";
            ans.to_owned()
        } else {
            panic!("ERROR")
        }
    }

    fn visit_grouping(self: &mut AstPrinter, expr: &'a Expr) -> String {
        if let Expr::Grouping(grp) = expr {
            grp.accept(self).to_string()
        } else {
            panic!("ERROR")
        }
    }

    fn visit_var(&mut self, expr: &'a Expr) -> String {
        todo!()
    }

    fn visit_assign(&mut self, expr: &'a Expr) -> String {
        todo!()
    }

    fn visit_logical(&mut self, expr: &'a Expr) -> String {
        todo!()
    }

    fn visit_call(&mut self, expr: &'a Expr) -> String {
        todo!()
    }
}
