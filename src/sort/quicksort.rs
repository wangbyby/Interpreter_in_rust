
//实际包的实现
use std::cmp::PartialOrd;
use std::cmp::Ordering;
//保证age是可比较的
pub struct Person<T : std::cmp::PartialOrd> {
    pub age: T,
}

//注意泛型T的位置
impl<T> Person<T> where T:std::cmp::PartialOrd{
    //也可impl<T:std::cmp::PartialOrd> Person<T>
    pub fn new(a:T) -> Self{
        Person {age:a}
    }
}
//让Person可比较大小, 操作符重载???
impl<T:std::cmp::PartialOrd> PartialOrd for Person<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.age.partial_cmp(&other.age)
    }
}
//让Person可比较是否相等, 操作符重载???
impl<T:std::cmp::PartialOrd> PartialEq for Person<T> {
    fn eq(&self, other: &Self) -> bool {
        self.age == other.age
    }
}

/*
--排序的Vec<T>中的T
--跟Person<T>的T 之间不一样
*/
pub fn quicksort<T>( arr : &mut [T])where T:std::cmp::PartialOrd {
    quick_sorted(arr,0,arr.len()-1);
}
fn quick_sorted<T>( arr:&mut [T], a:usize,b : usize)where T:std::cmp::PartialOrd {
    match b-a {
        1 ... 19 => insert_sorted(arr,a,b),
        0 => {},
        _ => {
            let p = partion(arr,a,b);
            if p !=0 {
                quick_sorted(arr,a,p-1); //无符号整数...越界
            }
            quick_sorted(arr,p+1,b);
        },
    }
    // if a<b {
    //     if b-a < 20 {
    //         insert_sorted(arr,a,b);
    //     }else{
    //         let p = partion(arr,a,b);
    //         if p !=0 {
    //             quick_sorted(arr,a,p-1); //无符号整数...越界
    //         }
    //         quick_sorted(arr,p+1,b);
    //     }
    // }
}
//注意A的写法... vec下标为 usize, 容易越界, 不过发现更多的小错误...
fn partion<T>( arr :&mut [T], p:usize,r:usize)->usize  where T:std::cmp::PartialOrd { 
    let mut i = p;
    for j in p..r {
        if arr[j] < arr[r] { //比较T需要加std::cmp::PartialOrd
            arr.swap(i, j);
            i+=1;
        }
    }
    arr.swap(i,r);
    i
}

//插入排序
fn insert_sorted<T>( arr :&mut [T],l:usize,r:usize) where T:std::cmp::PartialOrd {
    let mut i:usize;
    for j in l+1..r+1 {
        i = j;
        while i>l && arr[i] < arr[i-1] {
            arr.swap(i,i-1);
            i-=1;
        }
    }
}



// //Sort行为... 类似go的接口, 不过感觉还是泛型在某些方面舒服,虽然代码多一点...
// pub trait Sort {
//     fn less(&self,a:usize, b:usize) -> bool; //大小
//     fn len(&self) -> usize; // 长度
//     fn swap_by_index(&mut self,a:usize, b:usize); //交换
// }

// //vec泛型...
// impl<T> Sort for Vec<T> where T:std::cmp::PartialOrd{
//     fn less(&self,a:usize, b : usize) ->bool { 
//         &self[a] < &self[b]
//     }
//     fn len(&self) -> usize {
//         self.len()
//     }
//     fn swap_by_index(&mut self,a:usize, b : usize) {
//         self.swap(a,b);
//     }
// }

// fn sort(s : &mut Sort) {
//     let l = s.len()-1;
//     quick_sort_trait(s,0,l);
// }
// fn quick_sort_trait( arr:&mut Sort, a:usize,b : usize){
//     if a<b {
//         if b-a < 20 {
//             insert_sort_Sort(arr,a,b);
//         }else{
//             let p = partion_trait(arr,a,b);
//             if p != 0 {
//                 quick_sort_trait(arr,a,p-1); //无符号整数...越界
//             }
//             quick_sort_trait(arr,p+1,b);
//         }
        
//     }
// }

// fn insert_sort_Sort(arr: &mut Sort,a:usize, b : usize) {
//     let mut i:usize;
//     for j in a+1..b+1 {
//         i= j;
//         while i>a&& arr.less(i,i-1) { // self.less()变成了less(&self)???
//             arr.swap_by_index(i,i-1);
//             i-=1;
//         }
//     }
// }
// fn partion_trait( arr :&mut Sort, p:usize,r:usize)->usize { 
//     let mut i = p;
//     for j in p..r {
//         if arr.less(j,r) {  //注意是与主键比较
//             arr.swap_by_index(i, j);
//             i+=1;
//         }
//     }
//     arr.swap_by_index(i,r);
//     i
// }

