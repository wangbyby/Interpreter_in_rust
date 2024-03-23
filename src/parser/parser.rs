use crate::ast::ast;
use crate::ast::ast::ASTNode;
use crate::mylexer::lexer;
use crate::token::token::{Token, TokenType, TokenType::*};
use crate::FullError::*;
use crate::{FullError, Result};
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

#[derive(Debug)]
pub struct Parser<'a> {
    l: Peekable<lexer::Lexer<'a>>,
    cur_token: Token,
}

type PrefixFn = fn(&mut Parser) -> Result<ast::ASTNode>;
type InfixFn = fn(&mut Parser, ast::ASTNode) -> Result<ast::ASTNode>;
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

fn parse_identifier(this: &mut Parser) -> Result<ast::ASTNode> {
    Ok(ASTNode::Identifier(ast::Identifier {
        token: this.cur_token.clone(),
        value: this.cur_token.literal.clone(),
    }))
}

fn parse_integer_literal(this: &mut Parser) -> Result<ast::ASTNode> {
    let cur_token = this.next_token().ok_or(FullError::EOF)?;
    println!("cur token={:?}", cur_token);
    let literal = cur_token.literal.parse::<i64>();
    let mut lit = ast::IntegerLiteral::new(cur_token);
    lit.value = match literal {
        Ok(v) => v,
        Err(e) => {
            return Err(e.into());
        }
    };
    println!("here?");
    Ok(ASTNode::IntegerLiteral(lit))
}
fn parse_prefix_expression(this: &mut Parser) -> Result<ast::ASTNode> {
    let mut expression =
        ast::PrefixExpression::new(this.cur_token.clone(), this.cur_token.literal.clone());

    this.next_token();

    expression.right = this.parse_expression(get_pri!(PREFIX))?.into();

    Ok(ASTNode::PrefixExpression(expression))
}
fn parse_infix_expression(this: &mut Parser, left: ast::ASTNode) -> Result<ast::ASTNode> {
    let mut expression =
        ast::InfixExpression::new(this.cur_token.clone(), this.cur_token.literal.clone());
    expression.left = Box::new(left);
    let p = this.cur_precedence();
    this.next_token();
    expression.right = this.parse_expression(p)?.into();

    Ok(ASTNode::InfixExpression(expression))
}
fn parse_boolean(this: &mut Parser) -> Result<ast::ASTNode> {
    Ok(ASTNode::Boolean(ast::Boolean::new(
        this.cur_token.clone(),
        this.cur_token.is_ty(True),
    )))
}

fn parse_group_expression(this: &mut Parser) -> Result<ast::ASTNode> {
    this.next_token();
    let exp = this.parse_expression(get_pri!(LOWEST))?;
    if !this.expect_peek(RPAREN) {
        return Err(crate::FullError::GroupErr);
    }
    Ok(exp)
}

fn parse_if_expression(this: &mut Parser) -> Result<ast::ASTNode> {
    let mut exp = ast::IfExpression::new(this.cur_token.clone());

    if !this.expect_peek(LPAREN) {
        return Err(crate::FullError::IfErr);
    }

    this.next_token();
    exp.condition = this.parse_expression(get_pri!(LOWEST))?.into();

    if !this.expect_peek(RPAREN) {
        return Err(crate::FullError::IfErr);
    }
    if !this.expect_peek(LBRACE) {
        return Err(crate::FullError::IfErr);
    }
    exp.consequence = this.parse_block_statement()?.into();

    if this.expect_peek(Else) {
        this.next_token();

        if !this.expect_peek(LBRACE) {
            return Err(crate::FullError::IfErr);
        }
        exp.alternative = this.parse_block_statement()?.into();
    }
    Ok(ASTNode::IfExpression(exp))
}

fn parse_func_literal(this: &mut Parser) -> Result<ast::ASTNode> {
    let mut lit = ast::FuncLiteral::new(this.cur_token.clone());
    if !this.expect_peek(LPAREN) {
        return Err(crate::FullError::FuncErr);
    }
    lit.params = this.parse_func_params();

    if !this.expect_peek(LBRACE) {
        return Err(crate::FullError::FuncErr);
    }
    lit.body = Box::new(this.parse_block_statement()?);
    Ok(ASTNode::FuncLiteral(lit))
}

fn parse_call_expression(this: &mut Parser, func: ast::ASTNode) -> Result<ast::ASTNode> {
    let mut call_expression = ast::CallExpression::new(this.cur_token.clone());
    call_expression.func = Box::new(func);
    call_expression.args = this.parse_expression_list(RPAREN)?;
    Ok(ASTNode::CallExpression(call_expression))
}

fn parse_string_literal(this: &mut Parser) -> Result<ast::ASTNode> {
    Ok(ast::ASTNode::StringLiteral(
        this.cur_token.clone(),
        this.cur_token.literal.clone(),
    ))
}

fn parse_array_literal(this: &mut Parser) -> Result<ast::ASTNode> {
    Ok(ast::ASTNode::ArrayLiteral(
        this.cur_token.clone(),
        this.parse_expression_list(RBRACKET)?,
    ))
}

fn parse_index_expression(this: &mut Parser, left: ast::ASTNode) -> Result<ast::ASTNode> {
    let cur_token = this.cur_token.clone();

    this.next_token();
    let index = this.parse_expression(get_pri!(LOWEST))?;

    if !this.expect_peek(RBRACKET) {
        return Err(crate::FullError::IndexErr);
    }
    Ok(ast::ASTNode::IndexLiteral(
        cur_token,
        Box::new(left),
        Box::new(index),
    ))
}

fn parse_hash_helper(this: &mut Parser) -> Result<Vec<(Box<ast::ASTNode>, Box<ast::ASTNode>)>> {
    let mut hash = vec![];
    while !this.expect_peek(RBRACE) {
        this.next_token();
        let key = this.parse_expression(get_pri!(LOWEST))?;

        if !this.expect_peek(COLON) {
            return Err(crate::FullError::HashErr);
        }

        this.next_token();
        let value = this.parse_expression(get_pri!(LOWEST))?;

        hash.push((Box::new(key), Box::new(value)));

        if !this.expect_peek(COMMA) && !this.expect_peek(RBRACE) {
            return Err(crate::FullError::HashErr);
        }
    }
    if !this.expect_peek(RBRACE) {
        return Err(crate::FullError::HashErr);
    }
    Ok(hash)
}

fn parse_hash_literal(this: &mut Parser) -> Result<ast::ASTNode> {
    let cur_token = this.cur_token.clone();
    let hash = parse_hash_helper(this)?;
    Ok(ast::ASTNode::HashLiteral(cur_token, hash))
}

fn parse_assign_expression(this: &mut Parser, left: ast::ASTNode) -> Result<ast::ASTNode> {
    let mut letstmt = ast::LetStatement::new();
    letstmt.token = this.cur_token.clone();

    match left {
        ast::ASTNode::Identifier(value) => letstmt.name = value,
        _ => return Err(crate::FullError::AssignErr),
    }

    this.next_token();
    letstmt.value = Box::new(this.parse_expression(get_pri!(LOWEST))?);

    if !this.expect_peek(SEMICOLON) {
        return Err(crate::FullError::AssignErr);
    }

    Ok(ast::ASTNode::LetStatement(letstmt))
}

impl FuncParser {
    pub fn new() -> Self {
        let mut fp = Self::default();
        fp.insert_prefix(IDENT, parse_identifier);
        fp.insert_prefix(INT, parse_integer_literal);
        fp.insert_prefix(BANG, parse_prefix_expression);
        fp.insert_prefix(MINUS, parse_prefix_expression);
        fp.insert_prefix(True, parse_boolean);
        fp.insert_prefix(False, parse_boolean);
        fp.insert_prefix(LPAREN, parse_group_expression);
        fp.insert_prefix(If, parse_if_expression);
        fp.insert_prefix(Function, parse_func_literal);
        fp.insert_prefix(Str, parse_string_literal);
        fp.insert_prefix(LBRACE, parse_hash_literal);

        fp.insert_infix(PLUS, parse_infix_expression);
        fp.insert_infix(MINUS, parse_infix_expression);
        fp.insert_infix(SLASH, parse_infix_expression);
        fp.insert_infix(ASTERISK, parse_infix_expression);
        fp.insert_infix(EQ, parse_infix_expression);
        fp.insert_infix(NotEQ, parse_infix_expression);
        fp.insert_infix(LT, parse_infix_expression);
        fp.insert_infix(GT, parse_infix_expression);
        fp.insert_infix(LBRACKET, parse_call_expression);
        fp.insert_infix(ASSIGN, parse_assign_expression);
        fp.insert_infix(LPAREN, parse_call_expression);

        fp
    }
}

lazy_static! {
    static ref FUNCPARSER: FuncParser = FuncParser::new();
}

impl<'a> Parser<'a> {
    fn next_token(&mut self) -> Option<Token> {
        self.l.next()
    }
    fn peek_token(&mut self) -> Option<&Token> {
        self.l.peek()
    }

    pub fn new(l: lexer::Lexer) -> Parser {
        Parser {
            l: l.peekable(),
            cur_token: Token::default(),
        }
    }

    //入口函数
    pub fn parse_program(&mut self) -> Result<ast::ASTNode> {
        let mut program = ast::Program::new();

        while let Some(token) = self.next_token() {
            let stmt = self.parse_statement(token)?;
            program.statements.push(Box::new(stmt));
        }

        Ok(ast::ASTNode::Program(program))
    }

    fn parse_statement(&mut self, cur_token: Token) -> Result<ast::ASTNode> {
        match cur_token.ty {
            Let => {
                let letstmt = self.parse_letstatement(cur_token)?;
                Ok(ASTNode::LetStatement(letstmt))
            }
            Return => {
                let ret = self.parse_returnstatement(cur_token)?;
                Ok(ASTNode::ReturnStatement(ret))
            }
            _ => self.parse_expression_statement(),
        }
    }
    fn parse_expression_statement(&mut self) -> Result<ast::ASTNode> {
        let mut stmt = ast::ExpressionStatement::new(self.cur_token.clone());

        let expr = self.parse_expression(get_pri!(LOWEST))?;
        stmt.expression = Box::new(expr);

        if self.expect_peek(SEMICOLON) {
            self.next_token();
        }
        Ok(ast::ASTNode::ExpressionStatement(stmt))
    }

    fn parse_expression(&mut self, precedence: u8) -> Result<ast::ASTNode> {
        let first_peek = self.peek_token().ok_or(FullError::EOF)?;
        if let Some(prefix) = FUNCPARSER.prefix_parser_fns.get(&first_peek.ty) {
            let mut left_expr = prefix(self)?;
            println!("not end {:?}",self.l.peek());
            while let Some(peek) = self.peek_token() {
                if peek.ty == SEMICOLON{
                    break;
                }
                let p = peek.ty != SEMICOLON && precedence < get_precedence(peek.ty);
                if p {
                    if let Some(infix) = FUNCPARSER.infix_parser_fns.get(&peek.ty) {
                        left_expr = infix(self, left_expr)?;
                    } else {
                        return Ok(left_expr);
                    }
                }
            }
            return Ok(left_expr);
        } else {
            return Err(crate::FullError::ExpressionErr);
        }
    }

    fn parse_expression_list(&mut self, end: TokenType) -> Result<Vec<Box<ast::ASTNode>>> {
        let mut list = vec![];
        if self.expect_peek(end) {
            self.next_token();
            return Ok(list);
        }

        self.next_token();
        let expr = self.parse_expression(get_pri!(LOWEST))?;
        list.push(Box::new(expr));

        while self.expect_peek(COMMA) {
            self.next_token();
            self.next_token();
            let expr = self.parse_expression(get_pri!(LOWEST))?;
            list.push(Box::new(expr));
        }

        if !self.expect_peek(end) {
            return Err(crate::FullError::ExpressionErr);
        }
        Ok(list)
    }
    // fn parse_call_args(&mut self)->Vec<Box<ast::ASTNode>>{
    //     let mut args = vec![];
    //     if self.expect_peek(RPAREN){
    //         self.next_token();
    //         return args;
    //     }
    //     self.next_token();
    //     args.push(self.parse_expression(get_pri!(LOWEST)));

    //     while self.expect_peek(COMMA){
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
        if self.expect_peek(RPAREN) {
            self.next_token();
            return ids;
        }
        self.next_token();

        let ident = ast::Identifier::new(self.cur_token.clone(), self.cur_token.literal.clone());

        ids.push(Some(Box::new(ident)));

        while self.expect_peek(COMMA) {
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
    fn parse_block_statement(&mut self) -> Result<ast::ASTNode> {
        let mut block = ast::BlockStatement::new(self.cur_token.clone());

        while let Some(token) = self.next_token() {
            if !token.is_ty(RBRACE) {
                break;
            }
            let val = self.parse_statement(token)?;

            block.statements.push(Box::new(val));
        }

        Ok(ast::ASTNode::BlockStatement(block))
    }

    // let
    // let name = expr;
    // cur_token is let
    fn parse_letstatement(&mut self, cur_token: Token) -> Result<ast::LetStatement> {
        let mut stmt = ast::LetStatement::new();
        if !self.expect_peek(IDENT) {
            return Err(crate::FullError::LetErr);
        }
        let id = self.next_token().ok_or(FullError::EOF)?;
        let val = id.literal.clone();
        stmt.name = ast::Identifier {
            token: id,
            value: val,
        };

        if !self.expect_peek(ASSIGN) {
            return Err(crate::FullError::AssignErr);
        }
        let _assign = self.next_token();

        stmt.value = Box::new(self.parse_expression(get_pri!(LOWEST))?);

        if self.expect_peek(SEMICOLON) {
            self.next_token();
            Ok(stmt)
        } else {
            Err(crate::FullError::MissSem)
        }
    }

    fn parse_returnstatement(&mut self, cur_token: Token) -> Result<ast::ReturnStatement> {
        let mut restmt = ast::ReturnStatement::new(cur_token);

        restmt.return_value = Box::new(self.parse_expression(get_pri!(LOWEST))?);

        if self.expect_peek(SEMICOLON) {
            self.next_token();
        }
        Ok(restmt)
    }

    fn expect_peek(&mut self, ty: TokenType) -> bool {
        self.peek_token().map(|tt| tt.is_ty(ty)).unwrap_or(false)
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
        let input = "let a = 10;";
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let ast = p.parse_program().unwrap();

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
