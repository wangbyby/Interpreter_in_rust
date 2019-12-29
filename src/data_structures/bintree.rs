


pub type BinNodeEnum<K,V> = Option<Box<BinNode<K,V>>>;
#[derive(Debug)]//二叉树
pub struct BinNode<K,V> where K:std::cmp::PartialOrd{ //节点
    left : BinNodeEnum<K,V>,
    right : BinNodeEnum<K,V>,
    pub key : K,
    pub value : V,
}


// #[derive(Debug)]
// pub struct Node<K,V>where K:std::cmp::PartialOrd {
//     pub root: BinNodeEnum<K,V>,
// }

// impl<K,V> BinNodeEnum<K,V>  where K:std::cmp::PartialOrd {
//     pub fn get_key(&self) ->K {
//         match self {
//             Some(ref e) => e.key,
//             _=>0,
//         }
//     }
// }

// impl<K,V> Node<K,V> where K:std::cmp::PartialOrd{
//     pub fn new(k:K,v:V) -> Self {
//         Node {
//             root:Some(Box::new(BinNode{
//                 key:k,
//                 value:v,
//                 left:None,
//                 right:None,
//             }))
//         }
//     }

//     pub fn insert(&mut self,kkk:K,vvv:V) {
//         let mut tmp = self.root;
        
//         // let mut new_node = Some(Box::new(BinNode::new(kkk,vvv)));
//         while is_not_none!(tmp) {
            
//             let a = tmp.get_key();
//             if a<kkk {
//                 tmp = match tmp {
//                     Some(ref mut t)=> &t.right,
//                     _=>None,
//                 }
//             }
//             else {
//                 tmp = match tmp {
//                     Some(ref mut t)=> &t.left,
//                     _=>None,
//                 }
//             }
//         }
//         tmp = &Some(Box::new(BinNode::new(kkk,vvv)));
//     }
// }


impl<K,V> BinNode<K,V> where K:std::cmp::PartialOrd {
    pub fn new(key1: K, value1: V) -> BinNode<K,V> {
        BinNode{
            left:None,
            right:None,
            key: key1,
            value:value1,
        }
    }
    pub fn insert_binnode(&mut self,k:K,v:V) {
        if self.key < k {
            if let Some(ref mut right) = self.right {
                right.insert_binnode(k,v);
            }else {
                self.right = Some(Box::new(BinNode::new(k, v))); 
            }
        }else {
            if let Some(ref mut left) = self.left {
                left.insert_binnode(k,v);
            }else {
                self.left = Some(Box::new(BinNode::new(k, v))); 
            }
        }
    }
    //以后换出智能指针试试...
    // pub fn insert_nonrecursive(&mut self,k:K,v:V) {
    //     let mut tmp = Some(Box::new(self));
    //     while let Some(ref mut ttmp) = tmp {
    //         if ttmp.key < k {
    //             tmp = Some(*ttmp);
    //         }else{
    //             tmp = Some(*ttmp);
    //         }
    //     }
    //     tmp = Some(Box::new(&mut BinNode::new(k,v)));

    // }
}