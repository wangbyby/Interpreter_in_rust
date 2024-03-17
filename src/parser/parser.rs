use crate::ast::ast;
use crate::ast::ast::ASTNode;
use crate::mylexer::lexer;
use crate::token;
use crate::token::token::{Token, TokenType, TokenType::*};
use std::cell::OnceCell;
use std::collections::HashMap;
use std::iter::Peekable;

#[repr(u8)]
pub enum Pri {
    LOWEST,
    ASSIGN,      // 赋值
    EQUALS,      // ==, !=
    LESSGREATER, // < ,>
    SUM,         //+,-
    PRODUCT,     //*,/
    PREFIX,      // !,-
    CALL,        // func()
    INDEX,       // array[0], map[0]
}

macro_rules! get_pri {
    ($x:ident) => {
        Pri::$x as u8
    };
}

lazy_static! {
    static ref HASHMAP: HashMap<TokenType, u8> = {
        let mut mmap = HashMap::new();
        mmap.insert(EQ, get_pri!(EQUALS));
        mmap.insert(NotEQ, get_pri!(EQUALS));
        mmap.insert(LT, get_pri!(LESSGREATER));
        mmap.insert(GT, get_pri!(LESSGREATER));
        mmap.insert(PLUS, get_pri!(SUM));
        mmap.insert(MINUS, get_pri!(LESSGREATER));
        mmap.insert(SLASH, get_pri!(PRODUCT));
        mmap.insert(ASTERISK, get_pri!(PRODUCT));
        mmap.insert(LPAREN, get_pri!(CALL));
        mmap.insert(LBRACKET, get_pri!(INDEX));
        mmap.insert(ASSIGN, get_pri!(ASSIGN));
        mmap
    };
}

fn get_precedence(t: TokenType) -> u8 {
    HASHMAP.get(&t).map(|t| *t).unwrap_or(get_pri!(LOWEST))
}

macro_rules! ASTNode_None {
    () => {
        Box::new(ASTNode::None)
    };
}

type PrefixParserFn = fn(&mut Parser) -> Box<ast::ASTNode>;
type InfixParserFn = fn(&mut Parser, Box<ast::ASTNode>) -> Box<ast::ASTNode>;

pub struct Parser<'a> {
    l: Peekable<lexer::Lexer<'a>>,
    cur_token: Token,
    peek_token: Token,
    pub errors: Vec<String>,

    prefix_parser_fns: HashMap<TokenType, PrefixParserFn>,
    infix_parser_fns: HashMap<TokenType, InfixParserFn>,
}

impl<'a> Iterator for Parser<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

impl<'a> Parser<'a> {
    fn next_token(&mut self) -> Option<Token> {
        self.l.next()
    }
    fn peek_token(&mut self) -> Option<&Token> {
        self.l.peek()
    }

    fn parse_identifier(this: &mut Parser) -> Box<ast::ASTNode> {
        Box::new(ASTNode::Identifier(ast::Identifier {
            token: this.cur_token.clone(),
            value: this.cur_token.Literal.clone(),
        }))
    }

    fn parse_integer_literal(this: &mut Parser) -> Box<ast::ASTNode> {
        let mut lit = ast::IntegerLiteral::new(this.cur_token.clone());
        lit.value = match this.cur_token.Literal.parse::<i64>() {
            Ok(v) => v,
            Err(e) => {
                this.errors.push(e.to_string());
                return ASTNode_None!();
            }
        };
        Box::new(ASTNode::IntegerLiteral(lit))
    }
    fn parse_prefix_expression(this: &mut Parser) -> Box<ast::ASTNode> {
        let mut expression =
            ast::PrefixExpression::new(this.cur_token.clone(), this.cur_token.Literal.clone());

        this.next_token();

        expression.right = this.parse_expression(get_pri!(PREFIX));

        Box::new(ASTNode::PrefixExpression(expression))
    }
    fn parse_infix_expression(this: &mut Parser, left: Box<ast::ASTNode>) -> Box<ast::ASTNode> {
        let mut expression =
            ast::InfixExpression::new(this.cur_token.clone(), this.cur_token.Literal.clone());
        expression.left = left;
        let p = this.cur_precedence();
        this.next_token();
        expression.right = this.parse_expression(p);

        Box::new(ASTNode::InfixExpression(expression))
    }
    fn parse_boolean(this: &mut Parser) -> Box<ast::ASTNode> {
        Box::new(ASTNode::Boolean(ast::Boolean::new(
            this.cur_token.clone(),
            this.cur_token_is(True),
        )))
    }

    fn parse_group_expression(this: &mut Parser) -> Box<ast::ASTNode> {
        this.next_token();
        let exp = this.parse_expression(get_pri!(LOWEST));
        if !this.expect_peek(RPAREN) {
            return ASTNode_None!();
        }
        exp
    }

    fn parse_if_expression(this: &mut Parser) -> Box<ast::ASTNode> {
        let mut exp = ast::IfExpression::new(this.cur_token.clone());

        if !this.expect_peek(LPAREN) {
            return ASTNode_None!();
        }

        this.next_token();
        exp.condition = this.parse_expression(get_pri!(LOWEST));

        if !this.expect_peek(RPAREN) {
            return ASTNode_None!();
        }
        if !this.expect_peek(LBRACE) {
            return ASTNode_None!();
        }
        exp.consequence = this.parse_block_statement();

        if this.peek_token_is(Else) {
            this.next_token();

            if !this.expect_peek(LBRACE) {
                return ASTNode_None!();
            }
            exp.alternative = this.parse_block_statement();
        }
        Box::new(ASTNode::IfExpression(exp))
    }

    fn parse_func_literal(this: &mut Parser) -> Box<ast::ASTNode> {
        let mut lit = ast::FuncLiteral::new(this.cur_token.clone());
        if !this.expect_peek(LPAREN) {
            //少加一个!, 干...
            return ASTNode_None!();
        }
        lit.params = this.parse_func_params();

        if !this.expect_peek(LBRACE) {
            return ASTNode_None!();
        }
        lit.body = this.parse_block_statement();
        Box::new(ASTNode::FuncLiteral(lit))
    }

    fn parse_call_expression(this: &mut Parser, func: Box<ast::ASTNode>) -> Box<ast::ASTNode> {
        let mut call_expression = ast::CallExpression::new(this.cur_token.clone());
        call_expression.func = func;
        call_expression.args = this.parse_expression_list(RPAREN);
        Box::new(ASTNode::CallExpression(call_expression))
    }

    fn parse_string_literal(this: &mut Parser) -> Box<ast::ASTNode> {
        Box::new(ast::ASTNode::StringLiteral(
            this.cur_token.clone(),
            this.cur_token.Literal.clone(),
        ))
    }

    fn parse_array_literal(this: &mut Parser) -> Box<ast::ASTNode> {
        Box::new(ast::ASTNode::ArrayLiteral(
            this.cur_token.clone(),
            this.parse_expression_list(RBRACKET),
        ))
    }

    fn parse_index_expression(this: &mut Parser, left: Box<ast::ASTNode>) -> Box<ast::ASTNode> {
        let cur_token = this.cur_token.clone();

        this.next_token();
        let index = this.parse_expression(get_pri!(LOWEST));

        if !this.expect_peek(RBRACKET) {
            return ASTNode_None!();
        }
        Box::new(ast::ASTNode::IndexLiteral(cur_token, left, index))
    }

    fn parse_hash_helper(
        this: &mut Parser,
    ) -> Option<Vec<(std::boxed::Box<ast::ASTNode>, std::boxed::Box<ast::ASTNode>)>> {
        let mut hash = vec![];
        while !this.peek_token_is(RBRACE) {
            this.next_token();
            let key = this.parse_expression(get_pri!(LOWEST));

            if !this.expect_peek(COLON) {
                return None;
            }

            this.next_token();
            let value = this.parse_expression(get_pri!(LOWEST));

            hash.push((key, value));

            if !this.expect_peek(COMMA) && !this.peek_token_is(RBRACE) {
                return None;
            }
        }
        if !this.expect_peek(RBRACE) {
            return None;
        }
        Some(hash)
    }

    fn parse_hash_literal(this: &mut Parser) -> Box<ast::ASTNode> {
        let cur_token = this.cur_token.clone();

        if let Some(hash) = Parser::parse_hash_helper(this) {
            Box::new(ast::ASTNode::HashLiteral(cur_token, hash))
        } else {
            ASTNode_None!()
        }
    }

    fn parse_assign_expression(this: &mut Parser, left: Box<ast::ASTNode>) -> Box<ast::ASTNode> {
        let mut letstmt = ast::LetStatement::new();
        letstmt.token = this.cur_token.clone();

        match left.as_ref() {
            ast::ASTNode::Identifier(ref value) => letstmt.name = Box::new(value.clone()),
            _ => return ASTNode_None!(),
        }

        this.next_token();
        letstmt.value = this.parse_expression(get_pri!(LOWEST));

        if !this.expect_peek(SEMICOLON) {
            return ASTNode_None!();
        }

        Box::new(ast::ASTNode::LetStatement(letstmt))
    }

    fn parse_class_literal(this: &mut Parser) -> Box<ast::ASTNode> {
        let cur_token = this.cur_token.clone();
        this.next_token();

        if let Some(hash) = Parser::parse_hash_helper(this) {
            Box::new(ast::ASTNode::HashLiteral(cur_token, hash))
        } else {
            ASTNode_None!()
        }
    }

    pub fn new(l: lexer::Lexer) -> Parser {
        let mut p = Parser {
            l: l.peekable(),
            cur_token: Token:: default(),
            peek_token: Token::default(),
            errors: vec![],
            prefix_parser_fns: HashMap::new(),
            infix_parser_fns: HashMap::new(),
        };
        // 前缀
        p.prefix_parser_fns.insert(IDENT, Parser::parse_identifier);
        p.prefix_parser_fns
            .insert(INT, Parser::parse_integer_literal);
        p.prefix_parser_fns
            .insert(BANG, Parser::parse_prefix_expression);
        p.prefix_parser_fns
            .insert(MINUS, Parser::parse_prefix_expression);
        p.prefix_parser_fns.insert(True, Parser::parse_boolean);
        p.prefix_parser_fns.insert(False, Parser::parse_boolean);
        p.prefix_parser_fns
            .insert(LPAREN, Parser::parse_group_expression);
        p.prefix_parser_fns.insert(If, Parser::parse_if_expression);
        p.prefix_parser_fns
            .insert(Function, Parser::parse_func_literal);
        p.prefix_parser_fns
            .insert(Str, Parser::parse_string_literal);
        p.prefix_parser_fns
            .insert(LBRACKET, Parser::parse_array_literal);
        p.prefix_parser_fns
            .insert(LBRACE, Parser::parse_hash_literal);
        p.prefix_parser_fns
            .insert(Class, Parser::parse_class_literal);

        //中缀
        p.infix_parser_fns
            .insert(PLUS, Parser::parse_infix_expression);
        p.infix_parser_fns
            .insert(MINUS, Parser::parse_infix_expression);
        p.infix_parser_fns
            .insert(SLASH, Parser::parse_infix_expression);
        p.infix_parser_fns
            .insert(ASTERISK, Parser::parse_infix_expression);
        p.infix_parser_fns
            .insert(EQ, Parser::parse_infix_expression);
        p.infix_parser_fns
            .insert(NotEQ, Parser::parse_infix_expression);
        p.infix_parser_fns
            .insert(LT, Parser::parse_infix_expression);
        p.infix_parser_fns
            .insert(GT, Parser::parse_infix_expression);
        p.infix_parser_fns
            .insert(LPAREN, Parser::parse_call_expression);
        p.infix_parser_fns
            .insert(LBRACKET, Parser::parse_index_expression);
        p.infix_parser_fns
            .insert(ASSIGN, Parser::parse_assign_expression);

        p.next_token();
        p.next_token();
        p
    }

    //入口函数
    pub fn parse_program(&mut self) -> Box<ast::ASTNode> {
        let mut program = ast::Program::new();

        while self.cur_token.Type != EOF {
            if let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt);
            }

            self.next_token();
        }

        Box::new(ast::ASTNode::Program(program))
    }

    fn parse_statement(&mut self) -> Option<Box<ast::ASTNode>> {
        Some(match self.cur_token.Type {
            LET => {
                if let Some(letstmt) = self.parse_letstatement() {
                    Box::new(ASTNode::LetStatement(*letstmt))
                } else {
                    ASTNode_None!()
                }
            }
            RETURN => {
                if let Some(stmt) = self.parse_returnstatement() {
                    Box::new(ASTNode::ReturnStatement(*stmt))
                } else {
                    ASTNode_None!()
                }
            }
            _ => self.parse_expression_statement(),
        })
    }
    fn parse_expression_statement(&mut self) -> Box<ast::ASTNode> {
        let mut stmt = ast::ExpressionStatement::new(self.cur_token.clone());

        stmt.expression = self.parse_expression(get_pri!(LOWEST));

        if self.peek_token_is(SEMICOLON) {
            self.next_token();
        }
        Box::new(ast::ASTNode::ExpressionStatement(stmt))
    }

    fn parse_expression(&mut self, precedence: u8) -> Box<ast::ASTNode> {
        if let Some(prefix) = self.prefix_parser_fns.get_mut(&self.cur_token.Type) {
            let mut left_expr = prefix(self);
            while !self.peek_token_is(SEMICOLON) && precedence < self.peek_precedence() {
                if let Some(infix) = self.infix_parser_fns.get_mut(&self.peek_token.Type).cloned() {
                    //找了一天的错,干...
                    self.next_token();
                    left_expr = infix(self, left_expr);
                } else {
                    return left_expr;
                }
            }
            return left_expr;
        } else {
            self.no_prefix_parse_error(self.cur_token.Type);
            return ASTNode_None!();
        }
    }

    fn parse_expression_list(&mut self, end: TokenType) -> Vec<Box<ast::ASTNode>> {
        let mut list = vec![];
        if self.peek_token_is(end) {
            self.next_token();
            return list;
        }

        self.next_token();
        list.push(self.parse_expression(get_pri!(LOWEST)));

        while self.peek_token_is(COMMA) {
            self.next_token();
            self.next_token();
            list.push(self.parse_expression(get_pri!(LOWEST)));
        }

        if !self.expect_peek(end) {
            return vec![];
        }
        list
    }
    // fn parse_call_args(&mut self)->Vec<Box<ast::ASTNode>>{
    //     let mut args = vec![];
    //     if self.peek_token_is(RPAREN){
    //         self.next_token();
    //         return args;
    //     }
    //     self.next_token();
    //     args.push(self.parse_expression(get_pri!(LOWEST)));

    //     while self.peek_token_is(COMMA){
    //         self.next_token();
    //         self.next_token();
    //         args.push(self.parse_expression(get_pri!(LOWEST)));
    //     }
    //     if !self.expect_peek(RPAREN){
    //         return vec![];
    //     }
    //     args
    // }

    fn parse_func_params(&mut self) -> Vec<Option<Box<ast::Identifier>>> {
        let mut ids = vec![];
        if self.peek_token_is(RPAREN) {
            self.next_token();
            return ids;
        }
        self.next_token();

        let ident = ast::Identifier::new(self.cur_token.clone(), self.cur_token.Literal.clone());

        ids.push(Some(Box::new(ident)));

        while self.peek_token_is(COMMA) {
            self.next_token();
            self.next_token();

            let ident =
                ast::Identifier::new(self.cur_token.clone(), self.cur_token.Literal.clone());
            ids.push(Some(Box::new(ident)));
        }

        if !self.expect_peek(RPAREN) {
            return vec![];
        }
        ids
    }

    // { }
    fn parse_block_statement(&mut self) -> Box<ast::ASTNode> {
        let mut block = ast::BlockStatement::new(self.cur_token.clone());

        self.next_token();

        while !self.cur_token_is(RBRACE) {
            if let Some(value) = self.parse_statement() {
                block.statements.push(value);
            }
            self.next_token();
        }
        Box::new(ast::ASTNode::BlockStatement(block))
    }

    // let与return
    fn parse_letstatement(&mut self) -> Option<Box<ast::LetStatement>> {
        let mut stmt = ast::LetStatement::new();
        stmt.token = self.cur_token.clone();
        if !self.expect_peek(IDENT) {
            return None;
        }
        stmt.name = Box::new(ast::Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.Literal.clone(),
        });

        if !self.expect_peek(ASSIGN) {
            return None;
        }

        self.next_token();

        stmt.value = self.parse_expression(get_pri!(LOWEST));

        if self.peek_token_is(SEMICOLON) {
            self.next_token();
        }
        Some(Box::new(stmt))
    }

    fn parse_returnstatement(&mut self) -> Option<Box<ast::ReturnStatement>> {
        let mut restmt = ast::ReturnStatement::new(self.cur_token.clone());
        self.next_token();

        restmt.return_value = self.parse_expression(get_pri!(LOWEST));

        if self.peek_token_is(SEMICOLON) {
            self.next_token();
        }
        Some(Box::new(restmt))
    }

    //以下为辅助函数

    fn no_prefix_parse_error(&mut self, t: TokenType) {
        self.errors.push(String::from(format!(
            "no prefix parse func for {} found",
            t
        )));
    }

    fn cur_token_is(&self, t: TokenType) -> bool {
        self.cur_token.Type == t
    }
    fn peek_token_is(&self, t: TokenType) -> bool {
        self.peek_token.Type == t
    }

    fn expect_peek(&mut self, t: TokenType) -> bool {
        if self.peek_token_is(t) {
            self.next_token();
            return true;
        } else {
            self.peek_errors(t);
            return false;
        }
    }

    pub fn error(&self) -> Vec<String> {
        self.errors.clone()
    }

    fn peek_errors(&mut self, t: TokenType) {
        let msg = format!(
            "expected next token to {}, but got {}",
            t, self.peek_token.Type
        );
        self.errors.push(msg);
    }

    fn peek_precedence(&mut self) -> u8 {
        get_precedence(self.peek_token.Type)
    }
    fn cur_precedence(&mut self) -> u8 {
        get_precedence(self.cur_token.Type)
    }
}
