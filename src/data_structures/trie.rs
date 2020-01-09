// #[derive(Debug)]
// pub struct Trie<T> where T: std::marker::Copy{
//     root : Option<Box<TrieNode<T>>>, //根节点
//     size : usize, // number of values
// }

// impl<T> Trie<T> where T: std::marker::Copy{
//     pub fn new() -> Self {
//         Trie{
//             root : None,
//             size : 0,
//         }
//     }
//     //根据in_str查询
//     //查询到了 返回Some(result)
//     //没有查到 None
//     pub fn get<S>(&mut self,in_str: S)->Option<T>  where S: std::borrow::Borrow<str>{
//         if let Some(ref mut node) = self.root {
//             return node.search(in_str)
//         }
//         None
//     } 
//     //这里能输入&str和String, 设置value
//     pub fn set<S : std::borrow::Borrow<str>>(&mut self, in_str: S,in_value:T) {
//         self.size+=1;
//         let nl = in_str.borrow().chars().count();
//         if let Some(ref mut node) = self.root{
//             node.insert(&mut in_str.borrow().chars(), in_value, nl);
//             return;
//         }
//         let mut tmp_str = in_str.borrow().chars();

//         if let Some(ref mut k) = tmp_str.nth(0) {
//             self.root = Some(Box::new(TrieNode::new(*k)));
//             if let Some(ref mut node) = self.root{
//                 node.insert(&mut tmp_str, in_value, nl-1);
//                 return;
//             }
//         }
//     }
//     pub fn del_value<S> (&mut self,in_str: S)->bool where S: std::borrow::Borrow<str>{
//         true
//     } 
// }
// //TrieNode
// //有key和value, key为String
// #[derive(Debug)]
// struct TrieNode<T> where T: std::marker::Copy{
//     key : char, //key char utf8
//     value : Option<T>, //泛型
//     left : Option<Box<TrieNode<T>>>, //左边的   left.key < self.key  
//     right : Option<Box<TrieNode<T>>>,//右边的   right.key > self.key
//     middle : Option<Box<TrieNode<T>>>, //中间的 middle.key == self.key
// }

// impl<T> TrieNode<T> where T: std::marker::Copy{
//     fn new(input_key: char) -> Self {
//         TrieNode{
//             key : input_key,
//             left :None,
//             value : None,
//             right : None,
//             middle : None,
//         }
//     }
//     fn search<S> (&mut self, in_str: S) -> Option<T> where S: std::borrow::Borrow<str>{
//         let nl = in_str.borrow().chars().count();
//         let mut tmp_str = in_str.borrow().chars();
//         if let Some(k) = tmp_str.nth(0) {
//             return self.__search(&mut tmp_str,nl,0);
//         }
//         None
        
//     }
//     //search有很大的问题...
//     fn __search(&mut self,in_str: &mut std::str::Chars<'_>, length: usize, hit_time: usize) -> Option<T> {
//         let mut node = self;
//         for k in in_str{
//             if k == node.key{
//                 if let Some(ref mut mid) = node.middle{
//                     node = mid;
//                 }
//             }else if k < node.key {
//                 if let Some(ref mut l) = node.left{
//                     node = l;
//                 }
//             }else {
//                 if let Some(ref mut r) = node.right{
//                     node = r;
//                 }
//             }
//         }
//         None
//             // if k == self.key {
//             //     if length == hit_time + 1{
//             //         return self.value;
//             //     }else{
//             //         if let Some(next_k) = in_str.nth(0) {
//             //             if next_k > k {
//             //                 if let Some(ref mut node) = self.right{
//             //                     node.__search(in_str, length, hit_time + 1, next_k);
//             //                 }
//             //             }else if next_k == k {
//             //                 if let Some(ref mut node) = self.middle {
//             //                     node.__search(in_str, length, hit_time + 1, next_k);
//             //                 }
//             //             }
//             //             else{
//             //                 if let Some(ref mut node) = self.left{
//             //                     node.__search(in_str, length, hit_time + 1, next_k);
//             //                 }
//             //             }
//             //         }
//             //     }
                
//             // }else if k > self.key {
            
//             //     if let Some(ref mut node) = self.right {
//             //         node.__search(in_str, length, hit_time,k);
//             //     }
//             // }else {
            
//             //     if let Some(ref mut node) = self.left {
//             //         node.__search(in_str, length, hit_time,k);
//             //     }
//             // }
//             // None
    
//     }

//     fn set_value(&mut self, input_value:T) {
//         self.value = Some(input_value);
//     }
    
//     //insert函数的屏障
//     fn set<S : std::borrow::Borrow<str>>(&mut self, in_str: S,in_value:T) {
//         let nl = in_str.borrow().chars().count();
//         self.insert(&mut in_str.borrow().chars(), in_value, nl)
//     }
//     //插入
//     fn insert(&mut self, input_string :&mut std::str::Chars<'_>, input_value:T,length:usize) {
//         if length < 1 {
//             self.set_value(input_value);
//             return;
//         }
//         if let Some(k) = input_string.nth(0) {
//             if self.key < k {
//                 if let Some(ref mut right) = self.right {
//                     right.insert(input_string,input_value,length-1);
//                 }else{
//                     self.right = Some(Box::new(TrieNode::new(k)));
//                     if let Some(ref mut tr) = self.right {
//                         tr.insert(input_string,input_value,length-1);
//                     }
//                 }
//             }else if self.key > k {
//                 if let Some(ref mut left) = self.left {
//                     left.insert(input_string,input_value,length-1);
//                 }else{
//                     self.left = Some(Box::new(TrieNode::new(k)));
//                     if let Some(ref mut tr) = self.left {
//                         tr.insert(input_string,input_value,length-1);
//                     }
//                 }
//             }else{
//                 if let Some(ref mut middle) = self.middle {
//                     middle.insert(input_string,input_value,length-1);
//                 }else{
//                     self.middle = Some(Box::new(TrieNode::new(k)));
//                     if let Some(ref mut tr) = self.middle {
//                         tr.insert(input_string,input_value,length-1);
//                     }
//                 }
//             }
//         }
    
//     }
// }