use crate::ast::{
    Assign, Binary, Block, Call, ClassDecl, Error, Expr, Expression, FunDecl, Get, Grouping, If,
    Literal, Logical, Print, Return, Set, Stmt, Super, This, Unary, Var, VarDecl, While,
};
use crate::lexer::{Token, TokenLiteral, TokenType};
use crate::operators::Operator;

#[derive(Debug, Clone)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<Result<Stmt, Error>> {
        self.program()
    }

    fn program(&mut self) -> Vec<Result<Stmt, Error>> {
        let mut program = vec![];
        while !self.is_at_end() {
            let stmt = self.declaration();
            match stmt {
                Ok(stmt) => program.push(Ok(stmt)),
                Err(Error { msg: message }) => return vec![Err(Error { msg: message })],
            }
        }

        program
    }

    fn declaration(&mut self) -> Result<Stmt, Error> {
        if self.ismatch(&[TokenType::Var])? {
            self.var_decl()
        } else if self.ismatch(&[TokenType::Fun])? {
            self.fun_decl("function")
        } else if self.ismatch(&[TokenType::Class])? {
            self.class_decl()
        } else {
            self.statement()
        }
        //TODO catch error & synchronize
    }

    fn var_decl(&mut self) -> Result<Stmt, Error> {
        let name = self
            .consume(TokenType::Identifier, "Expect variable name.")?
            .lexeme
            .to_owned();

        let mut value = Expr::Literal(Literal::Nil);
        if self.ismatch(&[TokenType::Equal])? {
            value = self.expression()?;
        }

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )?;
        Ok(Stmt::VarDecl(VarDecl {
            name,
            expr: Box::new(value),
        }))
    }

    fn fun_decl(&mut self, kind: &str) -> Result<Stmt, Error> {
        let name = self
            .consume(TokenType::Identifier, &format!("Expect {} name.", kind))?
            .lexeme
            .clone();

        self.consume(
            TokenType::LeftParen,
            &format!("Expect '(' after {} name.", kind),
        )?;

        let mut parameters: Vec<Token> = vec![];
        if !self.check(&TokenType::RightParen) {
            loop {
                if parameters.len() >= 255 {
                    return Err(Error {
                        msg: format!("Cannot have more than 255 arguments."),
                    });
                }
                let token = self.consume(TokenType::Identifier, "Expect parameter name.")?;
                parameters.push(token.to_owned());
                if !self.ismatch(&[TokenType::Comma])? {
                    break;
                }
            }
        }

        self.consume(TokenType::RightParen, "Expect ')' after parameters.")?;

        self.consume(
            TokenType::LeftBrace,
            &format!("Expect '{{' before {} body.", kind),
        )?;

        let body: Vec<Stmt> = self.block()?;
        Ok(Stmt::FunDecl(FunDecl {
            name: name,
            parameters,
            body,
        }))
    }

    fn class_decl(&mut self) -> Result<Stmt, Error> {
        let name = self
            .consume(TokenType::Identifier, "Expect class name.")?
            .clone();

        let mut superclass: Option<Box<Expr>> = None;
        if self.ismatch(&[TokenType::Less])? {
            self.consume(TokenType::Identifier, "Expect superclass name.")?;
            superclass = Some(Box::new(Expr::Var(Var::Token(self.previous()?))));
        }
        self.consume(TokenType::LeftBrace, "Expect '{' before class body.")?;

        let mut methods: Vec<Stmt> = vec![];
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            methods.push(self.fun_decl("method")?.into());
        }

        self.consume(TokenType::RightBrace, "Expect '}' after class body.")?;
        Ok(Stmt::ClassDecl(ClassDecl {
            name: name,
            methods,
            superclass,
        }))
    }

    fn statement(&mut self) -> Result<Stmt, Error> {
        if self.ismatch(&[TokenType::Print])? {
            self.print_stmt()
        } else if self.ismatch(&[TokenType::LeftBrace])? {
            Ok(Stmt::Block(Block {
                stmts: self.block()?,
            }))
        } else if self.ismatch(&[TokenType::If])? {
            self.if_stmt()
        } else if self.ismatch(&[TokenType::While])? {
            self.while_stmt()
        } else if self.ismatch(&[TokenType::For])? {
            self.for_stmt()
        } else if self.ismatch(&[TokenType::Return])? {
            self.return_stmt()
        } else {
            self.expr_stmt()
        }
    }

    fn if_stmt(&mut self) -> Result<Stmt, Error> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition: Expr = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after if condition")?;
        let true_branch = self.statement()?;
        let mut false_branch = Stmt::Expression(Expression {
            expr: Box::new(Expr::Literal(Literal::Nil)),
        });
        if self.ismatch(&[TokenType::Else])? {
            false_branch = self.statement()?;
        }

        Ok(Stmt::If(If {
            condition: Box::new(condition),
            branch_true: Box::new(true_branch),
            branch_false: Box::new(false_branch),
        }))
    }

    fn return_stmt(&mut self) -> Result<Stmt, Error> {
        let keyword: Token = self.previous()?;
        let mut value: Option<Expr> = None;
        if !self.check(&TokenType::Semicolon) {
            value = Some(self.expression()?);
        }

        self.consume(TokenType::Semicolon, "Expect ';' after return value.")?;
        let return_value = value.unwrap_or_else(|| Expr::Literal(Literal::Nil));
        Ok(Stmt::Return(Return {
            keyword,
            value: return_value,
        }))
    }

    fn for_stmt(&mut self) -> Result<Stmt, Error> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let initializer: Option<Stmt> = if self.ismatch(&[TokenType::Semicolon])? {
            None
        } else if self.ismatch(&[TokenType::Var])? {
            Some(self.var_decl()?)
        } else {
            Some(self.expr_stmt()?)
        };

        let condition: Option<Expr> = if !self.check(&TokenType::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenType::Semicolon, "Expect ';' after loop condition.")?;

        let increment: Option<Expr> = if !self.check(&TokenType::RightParen) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenType::RightParen, "Expect ';' after loop condition.")?;

        let mut body: Stmt = self.statement()?;

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
            Expr::Literal(Literal::Bool(true))
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

        Ok(body)
    }

    fn while_stmt(&mut self) -> Result<Stmt, Error> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition: Expr = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after if condition")?;
        let body = self.statement()?;

        Ok(Stmt::While(While {
            condition: Box::new(condition),
            body: Box::new(body),
        }))
    }

    fn block(&mut self) -> Result<Vec<Stmt>, Error> {
        let mut statements: Vec<Stmt> = vec![];

        while !self.ismatch(&[TokenType::RightBrace])? && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.check(&TokenType::RightBrace);
        Ok(statements)
    }

    fn expr_stmt(&mut self) -> Result<Stmt, Error> {
        let value: Expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Expression(Expression {
            expr: Box::new(value),
        }))
    }

    fn print_stmt(&mut self) -> Result<Stmt, Error> {
        let value: Expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Print(Print {
            expr: Box::new(value),
        }))
    }

    fn expression(&mut self) -> Result<Expr, Error> {
        self.assigment()
    }

    fn assigment(&mut self) -> Result<Expr, Error> {
        let expr: Expr = self.or()?;

        if self.ismatch(&[TokenType::Equal])? {
            let value: Expr = self.assigment()?;

            match expr {
                Expr::Var(var) => Ok(Expr::Assign(Assign {
                    var,
                    expr: Box::new(value),
                })),
                Expr::Get(Get { object, name }) => Ok(Expr::Set(Set {
                    object,
                    name,
                    value: Box::new(value),
                })),
                _ => Err(Error {
                    msg: format!("Error at '=': Invalid assignment target.",),
                }),
            }
        } else {
            Ok(expr)
        }
    }

    fn or(&mut self) -> Result<Expr, Error> {
        let mut expr: Expr = self.and()?;
        while self.ismatch(&[TokenType::Or])? {
            let operator: Operator = Operator::from_token(&self.previous()?);
            let right: Expr = self.and()?;
            expr = Expr::Logical(Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, Error> {
        let mut expr: Expr = self.equality()?;
        while self.ismatch(&[TokenType::And])? {
            let operator: Operator = Operator::from_token(&self.previous()?);
            let right: Expr = self.and()?;
            expr = Expr::Logical(Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, Error> {
        let mut expr: Expr = self.comparison()?;

        while self.ismatch(&[TokenType::BangEqual, TokenType::EqualEqual])? {
            let operator: Operator = Operator::from_token(&self.previous()?);
            let right: Expr = self.comparison()?;
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, Error> {
        let mut expr: Expr = self.term()?;

        while self.ismatch(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ])? {
            let operator: Operator = Operator::from_token(&self.previous()?);
            let right: Expr = self.term()?;
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, Error> {
        let mut expr: Expr = self.factor()?;

        while self.ismatch(&[TokenType::Minus, TokenType::Plus])? {
            let operator: Operator = Operator::from_token(&self.previous()?);
            let right: Expr = self.factor()?;
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, Error> {
        let mut expr: Expr = self.unary()?;

        while self.ismatch(&[TokenType::Slash, TokenType::Star])? {
            let operator: Operator = Operator::from_token(&self.previous()?);
            let right: Expr = self.unary()?;
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, Error> {
        if self.ismatch(&[TokenType::Bang, TokenType::Minus])? {
            let operator: Operator = Operator::from_token(&self.previous()?);
            let right: Expr = self.factor()?;
            let expr = Expr::Unary(Unary {
                operator,
                right: Box::new(right),
            });

            return Ok(expr);
        }

        self.call()
    }

    fn call(&mut self) -> Result<Expr, Error> {
        let mut expr: Expr = self.primary()?;

        loop {
            if self.ismatch(&[TokenType::LeftParen])? {
                expr = self.finish_call(expr)?;
            } else if self.ismatch(&[TokenType::Dot])? {
                let name =
                    self.consume(TokenType::Identifier, "Expect property name after '.'.")?;
                expr = Expr::Get(Get {
                    object: Box::new(expr),
                    name: name.clone(),
                });
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, Error> {
        let mut arguments: Vec<Expr> = vec![];
        if !self.check(&TokenType::RightParen) {
            loop {
                if arguments.len() >= 255 {
                    Err(Error {
                        msg: format!("Cannot have more than 255 arguments."),
                    })?;
                }
                arguments.push(self.expression()?);
                if !self.ismatch(&[TokenType::Comma])? {
                    break;
                }
            }
        }

        let paren = self.consume(TokenType::RightParen, "Expect ')' after arguments.")?;
        Ok(Expr::Call(Call {
            callee: Box::new(callee),
            paren: paren.clone(),
            arguments,
        }))
    }

    fn primary(&mut self) -> Result<Expr, Error> {
        if self.ismatch(&[TokenType::False])? {
            Ok(Expr::Literal(Literal::Bool(false)))
        } else if self.ismatch(&[TokenType::True])? {
            Ok(Expr::Literal(Literal::Bool(true)))
        } else if self.ismatch(&[TokenType::Nil])? {
            Ok(Expr::Literal(Literal::Nil))
        } else if self.ismatch(&[TokenType::String, TokenType::Number])? {
            let expr: Expr = match self.previous()?.literal.as_ref().unwrap() {
                TokenLiteral::Number(n) => Expr::Literal(Literal::Number(*n)),
                TokenLiteral::Str(s) => Expr::Literal(Literal::Str(s.to_owned())),
            };
            Ok(expr)
        } else if self.ismatch(&[TokenType::LeftParen])? {
            let expr: Expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expected ')' after expression.")?;

            Ok(Expr::Grouping(Grouping {
                group: Box::new(expr),
            }))
        } else if self.ismatch(&[TokenType::This])? {
            Ok(Expr::This(This {
                keyword: self.previous()?,
            }))
        } else if self.ismatch(&[TokenType::Super])? {
            let keyword = self.previous()?;
            self.consume(TokenType::Dot, "Expect '.' after 'super'.")?;
            let method = self.consume(TokenType::Identifier, "Expect superclass method name.")?;
            Ok(Expr::Super(Super {
                keyword,
                method: method.clone(),
            }))
        } else if self.ismatch(&[TokenType::Identifier])? {
            Ok(Expr::Var(Var::Token(self.previous()?)))
        } else {
            Err(Error {
                msg: "Invalid type".to_string(),
            })
        }
    }

    fn ismatch(&mut self, tokens: &[TokenType]) -> Result<bool, Error> {
        for token_type in tokens {
            if self.check(token_type) {
                self.advance()?;
                return Ok(true);
            }
        }

        Ok(false)
    }

    fn check(&mut self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            false;
        }
        self.peek().token_type == *token_type
    }

    fn advance(&mut self) -> Result<Token, Error> {
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

    fn previous(&mut self) -> Result<Token, Error> {
        match self.tokens.get(self.current - 1) {
            Some(token) => Ok(token.clone()),
            None => Err(Error {
                msg: format!("No previous token"),
            }),
        }
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, Error> {
        if self.check(&token_type) {
            self.advance()
        } else {
            Err(Error {
                msg: format!("{} in {:?}", message, self.peek()),
            })
        }
    }
}
