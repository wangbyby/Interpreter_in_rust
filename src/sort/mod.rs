pub mod quicksort; //定义子包, 以文件名为包名


pub fn is_sorted<T>( arr :&mut [T]) -> bool
    where T:std::cmp::PartialOrd 
{
    if arr.len() <2 {
        return true;
    }
    let mut pre = &arr[0];

    for i in 1..arr.len() {
        if pre > &arr[i] {
            return false;
        }
        pre = &arr[i];
    }
    true
}