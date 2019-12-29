
use rand::Rng; //导入外部的包... 记得修改toml文件


mod sort;//在入口文件定义mod
mod data_structures;
mod tests;



use tests::is_sorted;
use data_structures::heap::MinHeap;
use sort::quicksort::Person;

fn main() {
    // let mut rag = rand::thread_rng();
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
}


#[test]
fn test_heap(){
    let mut rag = rand::thread_rng();
    let mut heap = MinHeap::new();
    
    for _j in 0..20 {
        let tmp:u8 = rag.gen(); //产生随机数 
        let person = Person::new(tmp); //创建结构体
        heap.push_tail(person);
        
    }  
    for j in 0..20 {
        
        match heap.pop_head() {
            Some(e) => println!("element.age = {:?}",e.age),
            _ => println!("none"),
        }
    }
}

#[test]
fn test_sorting() {
    is_sorted();
}