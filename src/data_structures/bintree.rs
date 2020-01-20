use std::collections::VecDeque;


//预计写语法树
//终结符, 非终结符, None
#[derive(Debug, Copy, Clone)]
pub enum Gkey {
    op(char), //非终结符
    num(i64), //终结符
}
type Num = i64;

fn compute(k: Gkey, a:Option<Num>  , b: Option<Num>) -> Option<Num> {
    match (k,a,b) {
        (Gkey::op(operator), Some(aa),Some(bb)) => {
            match operator {
                '+' => Some(aa + bb),
                '-' => Some(aa - bb),
                '*' => Some(aa * bb),
                '/' => Some(aa / bb),
                '%' => Some(aa % bb),
                _=> None,
            }
        },
        _=> None,
    }
}
type GNodeOptions<V> = Option<Box<GNode<V>>>;

#[derive(Debug)]
pub struct GNode<V> {
    pub left : GNodeOptions<V>,
    pub right : GNodeOptions<V>,
    pub key : Gkey ,
    pub value : V,
}
impl<V> GNode<V> {
    pub fn new(k : Gkey,val : V)->Self {
        GNode {
            left:None,
            right:None,
            key: k,
            value:val,
        }
    }
    //在后序遍历的基础上,计算
    pub fn judge(&mut self) -> Option<Num>{
        match self.key {
            Gkey::op(c) => {
                match (self.left.as_mut(), self.right.as_mut()) {
                    (Some(ref mut l), Some(ref mut r)) => {
                        compute(self.key, l.judge(), r.judge())
                    }
                    _=> None,
                }
            },
            Gkey::num(num)=> Some(num),
            _=> None,
        }
    }

    
}


type BinNodeEnum<K,V> = Option<Box<BinNode<K,V>>>;

#[derive(Debug)]//二叉树
pub struct BinNode<K,V>  where K:std::cmp::PartialOrd + std::fmt::Debug { //节点
    left : BinNodeEnum<K,V>,
    right : BinNodeEnum<K,V>,
    pub key : K,
    pub value : V,
}


impl<K,V> BinNode<K,V>  where K:std::cmp::PartialOrd + std::fmt::Debug {
    pub fn new(key1: K, value1: V) -> BinNode<K,V> {
        BinNode{
            left:None,
            right:None,
            key: key1,
            value:value1,
        }
    }

    //果然是递归的好写, 先写完递归的, 然后按照 递归->非递归转为非递归代码
    //疑问? : 对于LLVM来说, 递归与非递归是不是都会优化为循环... 那么函数式语言的递归表达力就很强了
    pub fn insert_recursive(&mut self,k:K,v:V) {
        if self.key < k {
            if let Some(ref mut right) = self.right {
                right.insert_recursive(k,v);
            }else {
                self.right = Some(Box::new(BinNode::new(k, v))); 
            }
        }else {
            if let Some(ref mut left) = self.left {
                left.insert_recursive(k,v);
            }else {
                self.left = Some(Box::new(BinNode::new(k, v))); 
            }
        }
    }
    
    //如果k相同, 也添加
    pub fn insert_nonrecursive(&mut self,k:K,v:V) {   
        let mut node = self;
        loop {
            match node.key<=k {
                true => {
                    if let Some(ref mut right) = node.right {
                        node = right;
                    }else{
                        node.right = Some(Box::new(BinNode::new(k, v)));
                        break;
                    }
                },
                false =>{
                    if let Some(ref mut l) = node.left {
                        node = l;
                    }else{
                        node.left = Some(Box::new(BinNode::new(k, v)));
                        break;
                    }
                },
            }
        }
    }

    //仅仅是打印
    fn visit_node(&self) {
        println!("key = {:#?}",self.key);
    }
    //遍历树
    //前序
    pub fn pre_order(&mut self) {
        self.visit_node();
        
        if let Some(ref mut l) = self.left{
            l.pre_order();
        }
        if let Some(ref mut r) = self.right{
            r.pre_order();
        }
    }

    //中序
    pub fn in_order(&mut self){
        if let Some(ref mut l) = self.left{
            l.in_order();
        }
        self.visit_node();
        if let Some(ref mut r) = self.right{
            r.in_order();
        }
    }
    //后序
    pub fn post_order(&mut self) {
        if let Some(ref mut l) = self.left{
            l.post_order();
        }
        if let Some(ref mut r) = self.right{
            r.post_order();
        }
        self.visit_node();
    }

    
    //DFS-nonrecur
    pub fn dfs(&mut self){
        let mut stack = vec![];
        stack.push(self);
        
        while let Some(node) = stack.pop() {
        
            node.visit_node();
            if let Some(ref mut l) = node.left{
                stack.push(l);
            }
            if let Some(ref mut r) = node.right{
                stack.push(r);
            }
            
        }
    }
    //BFS - nonrecur
    pub fn bfs(&mut self) {
        let mut queue = VecDeque::new(); //标准库的容器
        queue.push_back(self);
        while let Some(node) = queue.pop_front() {
            node.visit_node();
            if let Some(ref mut l) = node.left{
                queue.push_back(l);
            }
            if let Some(ref mut r) = node.right{
                queue.push_back(r);
            }
        }
    }
}
