
use rand::Rng;

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


#[test]
pub fn is_heaped(){
    let mut rag = rand::thread_rng();
    let mut heap = crate::data_structures::heap::MinHeap::new();
    
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
    let mut list = crate::data_structures::list::ListNode::new(0);
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
