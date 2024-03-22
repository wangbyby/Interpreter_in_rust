use crate::ast::ast;
use crate::ast::ast::ASTNode;
use crate::mylexer::lexer;
use crate::token::token::{Token, TokenType, TokenType::*};
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

pub struct Parser<'a> {
    l: Peekable<lexer::Lexer<'a>>,
    cur_token: Token,
    peek_token: Token,
    pub errors: Vec<String>,
}

type PrefixFn = fn(&mut Parser) -> Box<ast::ASTNode>;
type InfixFn = fn(&mut Parser, Box<ast::ASTNode>) -> Box<ast::ASTNode>;
pub struct FuncParser {
    prefix_parser_fns: HashMap<TokenType, PrefixFn>,
    infix_parser_fns: HashMap<TokenType, InfixFn>,
}

impl FuncParser {
    pub fn insert_prefix(&mut self, ty: TokenType, pre: PrefixFn) {
        self.prefix_parser_fns.insert(ty, pre);
    }
    pub fn insert_infix(&mut self, ty: TokenType, infix: InfixFn) {
        self.infix_parser_fns.insert(ty, infix);
    }
}

impl Default for FuncParser {
    fn default() -> Self {
        Self {
            prefix_parser_fns: HashMap::new(),
            infix_parser_fns: HashMap::new(),
        }
    }
}

fn parse_identifier(this: &mut Parser) -> Box<ast::ASTNode> {
    Box::new(ASTNode::Identifier(ast::Identifier {
        token: this.cur_token.clone(),
        value: this.cur_token.literal.clone(),
    }))
}

fn parse_integer_literal(this: &mut Parser) -> Box<ast::ASTNode> {
    let mut lit = ast::IntegerLiteral::new(this.cur_token.clone());
    lit.value = match this.cur_token.literal.parse::<i64>() {
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
        ast::PrefixExpression::new(this.cur_token.clone(), this.cur_token.literal.clone());

    this.next_token();

    expression.right = this.parse_expression(get_pri!(PREFIX));

    Box::new(ASTNode::PrefixExpression(expression))
}
fn parse_infix_expression(this: &mut Parser, left: Box<ast::ASTNode>) -> Box<ast::ASTNode> {
    let mut expression =
        ast::InfixExpression::new(this.cur_token.clone(), this.cur_token.literal.clone());
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
        this.cur_token.literal.clone(),
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

    if let Some(hash) = parse_hash_helper(this) {
        Box::new(ast::ASTNode::HashLiteral(cur_token, hash))
    } else {
        ASTNode_None!()
    }
}

fn parse_assign_expression(this: &mut Parser, left: Box<ast::ASTNode>) -> Box<ast::ASTNode> {
    let mut letstmt = ast::LetStatement::new();
    letstmt.token = this.cur_token.clone();

    match left.as_ref() {
        ast::ASTNode::Identifier(ref value) => letstmt.name = value.clone(),
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

    if let Some(hash) = parse_hash_helper(this) {
        Box::new(ast::ASTNode::HashLiteral(cur_token, hash))
    } else {
        ASTNode_None!()
    }
}

impl FuncParser {
    pub fn new() -> Self {
        let mut fp = Self::default();
        fp.insert_prefix(IDENT, parse_identifier);
        fp
    }
}

lazy_static! {
    static ref FUNCPARSER: FuncParser = FuncParser::new();
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

    pub fn new(l: lexer::Lexer) -> Parser {
        let mut p = Parser {
            l: l.peekable(),
            cur_token: Token::default(),
            peek_token: Token::default(),
            errors: vec![],
        };

        p.next_token();
        p.next_token();
        p
    }

    //入口函数
    pub fn parse_program(&mut self) -> ast::ASTNode {
        let mut program = ast::Program::new();

        while let Some(token) = self.next() {
            if let Some(stmt) = self.parse_statement(token) {
                program.statements.push(stmt);
            }
        }

        ast::ASTNode::Program(program)
    }

    fn parse_statement(&mut self, cur_token: Token) -> Option<Box<ast::ASTNode>> {
        Some(match cur_token.ty {
            Let => {
                if let Some(letstmt) = self.parse_letstatement(cur_token) {
                    Box::new(ASTNode::LetStatement(letstmt))
                } else {
                    ASTNode_None!()
                }
            }
            Return => {
                if let Some(stmt) = self.parse_returnstatement(cur_token) {
                    Box::new(ASTNode::ReturnStatement(stmt))
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
        let cur_token = self.next().unwrap();
        if let Some(prefix) = FUNCPARSER.prefix_parser_fns.get(&cur_token.ty) {
            let mut left_expr = prefix(self);
            while let Some(peek) = self.peek_token() {
                let p = peek.ty != SEMICOLON && precedence < get_precedence(peek.ty);
                if p {
                    if let Some(infix) = FUNCPARSER.infix_parser_fns.get(&peek.ty) {
                        self.next_token();
                        left_expr = infix(self, left_expr);
                    } else {
                        return left_expr;
                    }
                }
            }
            return left_expr;
        } else {
            self.no_prefix_parse_error(cur_token.ty);
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

        let ident = ast::Identifier::new(self.cur_token.clone(), self.cur_token.literal.clone());

        ids.push(Some(Box::new(ident)));

        while self.peek_token_is(COMMA) {
            self.next_token();
            self.next_token();

            let ident =
                ast::Identifier::new(self.cur_token.clone(), self.cur_token.literal.clone());
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

        while let Some(token) = self.next_token() {
            if !token.is_ty(RBRACE) {
                break;
            }
            if let Some(value) = self.parse_statement(token) {
                block.statements.push(value);
            }
        }

        Box::new(ast::ASTNode::BlockStatement(block))
    }

    // let与return
    fn parse_letstatement(&mut self, cur_token: Token) -> Option<ast::LetStatement> {
        let mut stmt = ast::LetStatement::new();
        if !self.expect_peek(IDENT) {
            return None;
        }
        let val = cur_token.literal.clone();
        stmt.name = ast::Identifier {
            token: cur_token,
            value: val,
        };

        if !self.expect_peek(ASSIGN) {
            return None;
        }
        self.next_token();

        stmt.value = self.parse_expression(get_pri!(LOWEST));

        if self.peek_token_is(SEMICOLON) {
            self.next_token();
        }
        Some(stmt)
    }

    fn parse_returnstatement(&mut self, cur_token: Token) -> Option<ast::ReturnStatement> {
        let mut restmt = ast::ReturnStatement::new(cur_token);

        restmt.return_value = self.parse_expression(get_pri!(LOWEST));

        if self.peek_token_is(SEMICOLON) {
            self.next_token();
        }
        Some(restmt)
    }

    //以下为辅助函数

    fn no_prefix_parse_error(&mut self, t: TokenType) {
        self.errors.push(String::from(format!(
            "no prefix parse func for {} found",
            t
        )));
    }

    fn cur_token_is(&self, t: TokenType) -> bool {
        self.cur_token.ty == t
    }
    fn peek_token_is(&self, t: TokenType) -> bool {
        self.peek_token.ty == t
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
            t, self.peek_token.ty
        );
        self.errors.push(msg);
    }

    fn peek_precedence(&mut self) -> u8 {
        get_precedence(self.peek_token.ty)
    }
    fn cur_precedence(&mut self) -> u8 {
        get_precedence(self.cur_token.ty)
    }
}

#[cfg(test)]
mod parser {
    use crate::{
        ast::ast::{ASTNode, Identifier, IntegerLiteral, LetStatement},
        mylexer::lexer::{self, Lexer},
        token::token::Token,
    };

    use super::Parser;

    #[test]
    fn test() {
        let input = "let a = 10";
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let ast = p.parse_program();

        let mut letstat = LetStatement::new();
        let mut id = Identifier::default();
        id.value = "a".to_string();
        letstat.name = id;

        let mut num = IntegerLiteral::new(Token::default());
        num.value = 10;
        letstat.value = Box::new(crate::ast::ast::ASTNode::IntegerLiteral(num));

        assert_eq!(ast, ASTNode::LetStatement(letstat));
    }
}
