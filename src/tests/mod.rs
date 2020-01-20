
use rand::Rng;
use crate::data_structures;
use crate::data_structures::bintree::GNode;
use crate::data_structures::bintree::Gkey;

#[test]
//测试排序的
pub fn is_sorted() {
    assert_eq!(crate::sort::is_sorted(&mut vec![1]), true);
    assert_eq!(crate::sort::is_sorted(&mut vec![1,2,10,2,1]), false);
    let mut rng = rand::thread_rng();
    let mut test_vec = vec![];
    for _j in 0..20 {
        let ele:i32 = rng.gen();
        test_vec.push(ele);
    }
    crate::sort::quicksort::quicksort(&mut test_vec);
    assert!(crate::sort::is_sorted(&mut test_vec), true);
}

#[test] //二叉树
pub fn test_bintree(){
    
}

pub fn bintree(){
    let a = GNode::new(Gkey::num(20), 0);
    let b = GNode::new(Gkey::num(10), 0);
    let mut c = GNode::new(Gkey::op('+'), 0);
    c.left = Some(Box::new(a));
    c.right = Some(Box::new(b));

    let aa = GNode::new(Gkey::num(20), 0);
    let bb = GNode::new(Gkey::num(10), 0);
    let mut cc = GNode::new(Gkey::op('-'), 0);
    cc.left = Some(Box::new(aa));
    cc.right = Some(Box::new(bb));

    let mut ccc = GNode::new(Gkey::op('+'), 0);
    ccc.right = Some(Box::new(cc));
    ccc.left = Some(Box::new(c));
    println!("{:#?}",ccc.judge());
}

#[test]
pub fn is_heaped(){
    let mut rag = rand::thread_rng();
    let mut heap = data_structures::heap::MinHeap::new();
    
    for _j in 0..20 {
        let tmp:u8 = rag.gen(); //产生随机数 
        let person = crate::sort::quicksort::Person::new(tmp); //创建结构体
        heap.push_tail(person);
        
    }  
    let mut vvv = vec![];
    for j in 0..20 {
        
        match heap.pop_head() {
            Some(e) => vvv.push(e.age),
            _ => println!("none"),
        }
    }
    assert!(crate::sort::is_sorted(&mut vvv), true);
}


//测试list
#[test]
pub fn test_list(){
    let mut list = data_structures::list::ListNode::new(0);
    for i in 0..10{

        list.push(i);
    }
    
    for i in 0..10{
        list.pop();
    }
    let e = list.pop();
    assert_eq!(e,None);
    println!("{:#?}",list);
}

