//expression     → equality ;
//equality       → comparison ( ( "!=" | "==" ) comparison )* ;
//comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
//term           → factor ( ( "-" | "+" ) factor )* ;
//factor         → unary ( ( "/" | "*" ) unary )* ;
//unary          → ( "!" | "-" ) unary
//               | primary ;
//primary        → NUMBER | STRING | "true" | "false" | "nil"
//               | "(" expression ")" ;

//program        → statement* Eof ;
//statement      → exprStmt | printStmt ;
//exprStmt       → expression ";" ;
//printStms      → print expression ";" ;

//expression     → assignment ;
//assignment     → IDENTIFIER "=" assignment
//               | equality ;

//statement      → exprStmt
//              | printStmt
//              | block ;
//block          → "{" declaration* "}" ;

//statement      → exprStmt
//               | ifStmt
//               | printStmt
//               | block ;
//ifStmt         → "if" "(" expression ")" statement
//               ( "else" statement )? ;

//expression     → assignment ;
//assignment     → IDENTIFIER "=" assignment
//                  | logic_or ;
//logic_or       → logic_and ( "or" logic_and )* ;
//logic_and      → equality ( "and" equality )* ;

use crate::ast::{
    Assign, Binary, Block, Expr, Expression, Grouping, If, Logical, Operator, Print, Stmt, Unary,
    Value, Var, VarDecl, While,
};
use crate::lexer::{Literal, Token, TokenType};

#[derive(Debug, Clone)]
pub enum Type {
    Bool(bool),
    Number(f64),
    Str(String),
    Nil,
}

impl PartialEq for Type {
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }

    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Bool(_), Self::Bool(_)) => true,
            (Self::Number(_), Self::Number(_)) => true,
            (Self::Str(_), Self::Str(_)) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        self.program()
    }

    fn program(&mut self) -> Vec<Stmt> {
        let mut program = vec![];
        while !self.is_at_end() {
            let stmt = self.declaration();
            program.push(stmt);
        }

        program
    }

    fn declaration(&mut self) -> Stmt {
        if self.ismatch(&[TokenType::Var]) {
            self.var_decl()
        } else {
            self.statement()
        }
        //TODO catch error & synchronize
    }

    fn var_decl(&mut self) -> Stmt {
        let mut name: String = "".to_string();
        {
            name = self
                .consume(TokenType::Identifier, "Expect variable name.")
                .lexeme
                .to_owned();
        }
        let mut value = Expr::Value(Value { ty: Type::Nil });
        if self.ismatch(&[TokenType::Equal]) {
            value = self.expression();
        }
        {
            self.consume(
                TokenType::Semicolon,
                "Expect ';' after variable declaration.",
            );
        }
        Stmt::VarDecl(VarDecl {
            name,
            expr: Box::new(value),
        })
    }

    fn statement(&mut self) -> Stmt {
        if self.ismatch(&[TokenType::Print]) {
            self.print_stmt()
        } else if self.ismatch(&[TokenType::LeftBrace]) {
            Stmt::Block(Block {
                stmts: self.block(),
            })
        } else if self.ismatch(&[TokenType::If]) {
            self.if_stmt()
        } else if self.ismatch(&[TokenType::While]) {
            self.while_stmt()
        } else if self.ismatch(&[TokenType::For]) {
            self.for_stmt()
        } else {
            self.expr_stmt()
        }
    }

    fn if_stmt(&mut self) -> Stmt {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.");
        let condition: Expr = self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after if condition");
        let true_branch = self.statement();
        let mut false_branch = Stmt::Expression(Expression {
            expr: Box::new(Expr::Value(Value { ty: Type::Nil })),
        }); // TODO too hacky!
        if self.ismatch(&[TokenType::Else]) {
            false_branch = self.statement();
        }

        Stmt::If(If {
            condition: Box::new(condition),
            branch_true: Box::new(true_branch),
            branch_false: Box::new(false_branch),
        })
    }

    fn for_stmt(&mut self) -> Stmt {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.");
        let initializer: Option<Stmt> = if self.ismatch(&[TokenType::Semicolon]) {
            None
        } else if self.ismatch(&[TokenType::Var]) {
            Some(self.var_decl())
        } else {
            Some(self.expr_stmt())
        };

        let condition: Option<Expr> = if !self.check(&TokenType::Semicolon) {
            Some(self.expression())
        } else {
            None
        };
        self.consume(TokenType::Semicolon, "Expect ';' after loop condition.");

        let increment: Option<Expr> = if !self.check(&TokenType::RightParen) {
            Some(self.expression())
        } else {
            None
        };
        self.consume(TokenType::RightParen, "Expect ';' after loop condition.");

        let mut body: Stmt = self.statement();

        body = if let Some(inc) = increment {
            Stmt::Block(Block {
                stmts: vec![
                    body,
                    Stmt::Expression(Expression {
                        expr: Box::new(inc),
                    }),
                ],
            })
        } else {
            body
        };

        let cond = if let Some(cond) = condition {
            cond
        } else {
            Expr::Value(Value {
                ty: Type::Bool(true),
            })
        };
        body = Stmt::While(While {
            condition: Box::new(cond),
            body: Box::new(body),
        });

        body = if let Some(init) = initializer {
            Stmt::Block(Block {
                stmts: vec![init, body],
            })
        } else {
            body
        };

        body
    }

    fn while_stmt(&mut self) -> Stmt {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.");
        let condition: Expr = self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after if condition");
        let body = self.statement();

        Stmt::While(While {
            condition: Box::new(condition),
            body: Box::new(body),
        })
    }

    fn block(&mut self) -> Vec<Stmt> {
        let mut statements: Vec<Stmt> = vec![];

        while !self.ismatch(&[TokenType::RightBrace]) && !self.is_at_end() {
            statements.push(self.declaration())
        }

        self.check(&TokenType::RightBrace);
        statements
    }

    fn expr_stmt(&mut self) -> Stmt {
        let value: Expr = self.expression();
        self.consume(TokenType::Semicolon, "Expect ';' after value.");
        Stmt::Expression(Expression {
            expr: Box::new(value),
        })
    }

    fn print_stmt(&mut self) -> Stmt {
        let value: Expr = self.expression();
        self.consume(TokenType::Semicolon, "Expect ';' after value.");
        Stmt::Print(Print {
            expr: Box::new(value),
        })
    }

    fn expression(&mut self) -> Expr {
        self.assigment()
    }

    fn assigment(&mut self) -> Expr {
        //let expr: Expr = self.equality();
        let expr: Expr = self.or();

        if self.ismatch(&[TokenType::Equal]) {
            let value: Expr = self.assigment();

            match expr {
                Expr::Var(Var { var }) => Expr::Assign(Assign {
                    var,
                    expr: Box::new(value),
                }),
                _ => panic!("Invalid assignment target:"),
            }
        } else {
            expr
        }
    }

    fn or(&mut self) -> Expr {
        let mut expr: Expr = self.and();
        while self.ismatch(&[TokenType::Or]) {
            let operator: Operator = Operator::from_token(self.previous());
            let right: Expr = self.and();
            expr = Expr::Logical(Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }
        expr
    }

    fn and(&mut self) -> Expr {
        let mut expr: Expr = self.equality();
        while self.ismatch(&[TokenType::And]) {
            let operator: Operator = Operator::from_token(self.previous());
            let right: Expr = self.and();
            expr = Expr::Logical(Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }
        expr
    }

    fn equality(&mut self) -> Expr {
        let mut expr: Expr = self.comparison();

        while self.ismatch(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator: Operator = Operator::from_token(self.previous());
            let right: Expr = self.comparison();
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr: Expr = self.term();

        while self.ismatch(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator: Operator = Operator::from_token(self.previous());
            let right: Expr = self.term();
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr: Expr = self.factor();

        while self.ismatch(&[TokenType::Minus, TokenType::Plus]) {
            let operator: Operator = Operator::from_token(self.previous());
            let right: Expr = self.factor();
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr: Expr = self.unary();

        while self.ismatch(&[TokenType::Slash, TokenType::Star]) {
            let operator: Operator = Operator::from_token(self.previous());
            let right: Expr = self.unary();
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.ismatch(&[TokenType::Bang, TokenType::Minus]) {
            let operator: Operator = Operator::from_token(self.previous());
            let right: Expr = self.factor();
            let expr = Expr::Unary(Unary {
                operator,
                right: Box::new(right),
            });

            return expr;
        }

        self.call()
    }

    fn call(&mut self) -> Expr {
        let mut expr: Expr = self.primary();

        loop {
            if self.ismatch(&[TokenType::LeftParen]) {
                expr = self.finish_call(expr);
            } else {
                break;
            }
        }

        expr
    }

    fn finish_call(&mut self, callee: Expr) -> Expr {
        let mut arguments: Vec<Expr> = vec![];
        if !self.check(&TokenType::RightParen) {
            loop {
                if arguments.len() >= 255 {
                    panic!("Cannot have more than 255 arguments.");
                }
                arguments.push(self.expression());
                if self.ismatch(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        let paren = self.consume(TokenType::RightParen, "Expect ')' after arguments.");
        callee
        //Expr::Call {
        //    callee: Box::new(callee),
        //    paren: Box::new(paren.clone()), // TODO: Fix this
        //    arguments,
        //}
    }

    fn primary(&mut self) -> Expr {
        if self.ismatch(&[TokenType::False]) {
            Expr::Value(Value {
                ty: Type::Bool(false),
            })
        } else if self.ismatch(&[TokenType::True]) {
            Expr::Value(Value {
                ty: Type::Bool(true),
            })
        } else if self.ismatch(&[TokenType::Nil]) {
            Expr::Value(Value { ty: Type::Nil })
        } else if self.ismatch(&[TokenType::String, TokenType::Number]) {
            let expr: Expr = match self.previous().literal.as_ref().unwrap() {
                Literal::Number(n) => Expr::Value(Value {
                    ty: Type::Number(*n),
                }),
                Literal::Str(s) => Expr::Value(Value {
                    ty: Type::Str(s.to_owned()),
                }),
                _ => panic!("Invalid type"),
            };
            expr
        } else if self.ismatch(&[TokenType::LeftParen]) {
            let expr: Expr = self.expression();
            self.consume(TokenType::RightParen, "Expected ')' after expression.");

            Expr::Grouping(Grouping {
                group: Box::new(expr),
            })
        } else if self.ismatch(&[TokenType::Identifier]) {
            Expr::Var(Var {
                var: Box::new(self.previous().to_owned()),
            })
        } else {
            panic!("Invalid type")
        }
    }

    fn ismatch(&mut self, tokens: &[TokenType]) -> bool {
        for token_type in tokens {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&mut self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            false;
        }
        self.peek().token_type == *token_type
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&mut self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&mut self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    fn previous(&mut self) -> &Token {
        self.tokens.get(self.current - 1).unwrap()
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> &Token {
        if self.check(&token_type) {
            return self.advance();
        }

        panic!("{} in {:?}", message, self.peek())
    }
}
