
//MinHeap
macro_rules! parent {
    ($child:ident) => {
        match $child {
            0=>0,
            _=>($child-1)/2,
        }
    };
}
macro_rules! left_child {
    ($parent:ident)=>{
        ($parent<<1)+1
    };
}
macro_rules! right_child {
    ($parent:ident)=>{
        ($parent+1)<<1
    };
}
pub struct MinHeap<T> where T:std::cmp::PartialOrd{
    payload:Vec<T>,
}
//为MinHeap绑定方法
impl<T> MinHeap<T> where T:std::cmp::PartialOrd{
    pub fn new() -> MinHeap<T> {
        MinHeap{ payload:Vec::new()}
    }
    fn length(&self) -> usize {
        self.payload.len()
    }
    fn less_element(&mut self,a:usize, b:usize) ->bool {
        self.payload[a] < self.payload[b]
    }
    fn swap_element(&mut self,a:usize, b:usize) {
        self.payload.swap(a,b);
    }
    fn shiftup(&mut self, par:usize, size:usize) {
        
        let lchild = left_child!(par);
        let rchild = right_child!(par);
        let mut min = par;
        if lchild < size && self.less_element(lchild,min) {
            min = lchild; 
        }
        if rchild < size && self.less_element(rchild,min) {
            min = rchild;
        }
        if min != par {
            self.swap_element(par,min);
            self.shiftup(min,size);
        }
    }

    //建立最小堆
    pub fn build_heap(&mut self){
        let l = self.payload.len();
        let l_half = l>>1;
        let mut i = l_half+1;
        while i>=1 {
            i-=1;
            self.shiftup(i, l);
        }
    }
    //堆排序 值大到小
    pub fn heapsort(&mut self) {
        let mut running_size = self.length()-1;
        while running_size > 0 {
            self.swap_element(0, running_size);
            self.shiftup(0,running_size);
            running_size -= 1;
        }
    }

    //大下标 --> 小下标
    fn shiftdown(&mut self,child:usize) {
        let par = parent!(child);
        if self.less_element(child, par) { // 这里没有写 0 <= par ,因为 par为uszie(默认>=0)
            self.swap_element(child, par);
            self.shiftdown(par);
        }
    }
    //加入元素
    pub fn push_tail(&mut self, element:T) {
        self.payload.push(element);
        self.shiftdown(self.payload.len()-1);
    }
    //删除最小的元素
    pub fn pop_head(&mut self) -> Option<T> {
        // let res = self.payload[0];
        self.swap_element(0, self.payload.len()-1);
        let res = self.payload.pop();
        self.shiftup(0, self.length());
        res
    }
}