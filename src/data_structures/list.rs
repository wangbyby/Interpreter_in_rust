

#[derive(Debug)]
pub struct ListNode<T> {
    pub value: T,
    next: Option<Box<ListNode<T>>>,
}

impl<T> ListNode<T> {
    pub fn new(val: T) -> ListNode<T> {
        ListNode {
            value: val,
            next:None,
        }
    }

    fn insert(&mut self,val:T) {
        let mut new_node  =ListNode::new(val);
        let node = self.next.take(); // self.next为Option<>, take()会取走值
        new_node.next = node;
        self.next = Some(Box::new(new_node));
    }
    pub fn push(&mut self,val:T) {
        self.insert(val);
    }
    //1.有则返回Some(value), 2.没有的话返回None
    pub fn pop(&mut self) ->Option<T> {
        
        let node_n = self.next.take();
        if let Some(mut node) = node_n { //核心是 mut node
            let val = node.value; 
            self.next = node.next.take();
            return Some(val);
        }
        None
    }
}