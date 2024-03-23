use crate::token::token::{self, Token};

type TokenAST = Option<token::Token>;

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum ASTNode {
    Program(Program),
    LetStatement(LetStatement),
    BlockStatement(BlockStatement),
    Boolean(Boolean),
    CallExpression(CallExpression),
    ExpressionStatement(ExpressionStatement),
    FuncLiteral(FuncLiteral),
    Identifier(Identifier),
    IfExpression(IfExpression),
    InfixExpression(InfixExpression),
    IntegerLiteral(IntegerLiteral),
    PrefixExpression(PrefixExpression),
    ReturnStatement(ReturnStatement),
    StringLiteral(TokenAST, String),
    ArrayLiteral(TokenAST, Vec<Box<ASTNode>>),
    IndexLiteral(TokenAST, Box<ASTNode>, Box<ASTNode>), //token, left, index
    HashLiteral(TokenAST, Vec<(Box<ASTNode>, Box<ASTNode>)>),
    None,
}

//可以为hash的key的类型
//bool,string, i64,没了?好像没有啦

impl ASTNode {
    pub fn new() -> Self {
        ASTNode::None
    }
    pub fn is_none(&self) -> bool {
        use self::ASTNode::None;
        match self {
            None => true,
            _ => false,
        }
    }
    pub fn is_some(&self) -> bool {
        !self.is_none()
    }
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct Program {
    pub statements: Vec<Box<ASTNode>>,
}

impl Program {
    pub fn new() -> Self {
        Program { statements: vec![] }
    }
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct LetStatement {
    pub token: TokenAST,
    pub name: Identifier,
    pub value: Box<ASTNode>,
}

impl LetStatement {
    pub fn new() -> LetStatement {
        LetStatement {
            token: None,
            name: Identifier::default(),
            value: Box::new(ASTNode::None),
        }
    }
    pub fn is_none(&self) -> bool {
        self.value.is_none()
    }
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct Identifier {
    pub token: TokenAST,
    pub value: String,
}

impl Identifier {
    pub fn new(token: token::Token, value: String) -> Self {
        Self { 
            token: Some(token), value }
    }

    pub fn default() -> Self {
        Identifier {
            token: None,
            value: String::default(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct ReturnStatement {
    pub token: TokenAST,
    pub return_value: Box<ASTNode>,
}

impl ReturnStatement {
    pub fn new(token: token::Token) -> Self {
        ReturnStatement {
            token: Some(token),
            return_value: Box::new(ASTNode::None),
        }
    }
}

//Expression

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct ExpressionStatement {
    token: token::Token,
    pub expression: Box<ASTNode>,
}

impl ExpressionStatement {
    pub fn new(token: token::Token) -> Self {
        ExpressionStatement {
            token: token,
            expression: Box::new(ASTNode::None),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct IntegerLiteral {
    token: token::Token,
    pub value: i64,
}
impl IntegerLiteral {
    pub fn new(token: token::Token) -> Self {
        IntegerLiteral {
            token: token,
            value: 0,
        }
    }
}

impl Default for IntegerLiteral {
    fn default() -> Self {
        IntegerLiteral {
            token: Token::default(),
            value: 0,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct PrefixExpression {
    token: token::Token,
    pub operator: String,
    pub right: Box<ASTNode>,
}

impl PrefixExpression {
    pub fn new<S: Into<String>>(token: token::Token, operator: S) -> Self {
        PrefixExpression {
            token: token,
            operator: operator.into(),
            right: Box::new(ASTNode::None),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct InfixExpression {
    token: token::Token,
    pub left: Box<ASTNode>,
    pub right: Box<ASTNode>,
    pub operator: String,
}

impl InfixExpression {
    pub fn new<S: Into<String>>(token: token::Token, operator: S) -> Self {
        InfixExpression {
            token: token,
            operator: operator.into(),
            left: Box::new(ASTNode::None),
            right: Box::new(ASTNode::None),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct Boolean {
    pub token: token::Token,
    pub value: bool,
}
impl Boolean {
    pub fn new(token: token::Token, value: bool) -> Self {
        Boolean {
            token: token,
            value: value,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct IfExpression {
    token: token::Token,
    pub condition: Box<ASTNode>,
    pub consequence: Box<ASTNode>,
    pub alternative: Box<ASTNode>,
}

impl IfExpression {
    pub fn new(token: token::Token) -> Self {
        IfExpression {
            token: token,
            condition: Box::new(ASTNode::None),
            consequence: Box::new(ASTNode::None),
            alternative: Box::new(ASTNode::None),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct BlockStatement {
    token: token::Token,
    pub statements: Vec<Box<ASTNode>>,
}

impl BlockStatement {
    pub fn new(token: token::Token) -> Self {
        BlockStatement {
            token: token,
            statements: vec![],
        }
    }
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct FuncLiteral {
    token: token::Token,
    pub params: Vec<Option<Box<Identifier>>>,
    pub body: Box<ASTNode>,
}

impl FuncLiteral {
    pub fn new(token: token::Token) -> Self {
        FuncLiteral {
            token: token,
            params: Vec::new(),
            body: Box::new(ASTNode::None),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct CallExpression {
    token: token::Token,
    pub func: Box<ASTNode>,      //函数名
    pub args: Vec<Box<ASTNode>>, //函数定义时的参数
}

impl CallExpression {
    pub fn new(token: token::Token) -> Self {
        CallExpression {
            token: token,
            func: Box::new(ASTNode::None),
            args: Vec::new(),
        }
    }
}
