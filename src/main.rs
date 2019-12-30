
mod sort;//在入口文件定义mod
mod data_structures;
mod tests;

use rand::Rng; //导入外部的包... 记得修改toml文件



use data_structures::heap::MinHeap;
use sort::quicksort::Person;
use crate::data_structures::bintree::BinNode;
fn main() {
    let mut rag = rand::thread_rng();
    // let mut heap = MinHeap::new();
    // // let mut a  = vec![]; //暂时不指定类型
    // for _j in 0..20 {
    //     let tmp:u8 = rag.gen(); //产生随机数
        
    //     let person = Person::new(tmp); //创建结构体
    //     heap.push_tail(person);
    //     // a.push(person);
    // }


    // // for i in &a { //引用, 不释放空间
    // //     println!("原始数据{:#?}",i)
    // // }
    // // quicksort(&mut a);
    // // sort(&mut a);
    
    
    // for j in 0..20 {
        
    //     match heap.pop_head() {
    //         Some(e) => println!("element.age = {:?}",e.age),
    //         _ => println!("none"),
    //     }
    // }

    // // for i in &heap.payload { //引用, 不释放空间
    // //     println!("{:#?}",i.age);
    // // }
    // let mut bt_node = crate::data_structures::bintree::BinNode::new(1, "string");
    // println!("{:#?}",bt_node);

    // bt_node.insert_binnode(2,"hello");
    // println!("{:#?}",bt_node);
    
    let mut tree = data_structures::bintree::BinNode::new(1u8, 20);
    // tree.insert_nonrecursive(20, "world");
    for i in 0..10 {
        let tmp:u8 = rag.gen(); //产生随机数
        tree.insert_nonrecursive(tmp, 10);

    }
    tree.dfs();
    tree.bfs();
    println!("{:?}", tree);

}


#[test]
fn test_sorting() {
    tests::is_sorted();
}


#[test]
fn test_bintree() {
    tests::test_bintree();
}

#[test]
fn test_list() {
    tests::test_list();
}