
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
