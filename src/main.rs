
mod sort;//在入口文件定义mod
mod data_structures;
mod tests;

use rand::Rng; //导入外部的包... 记得修改toml文件

use sort::is_sorted;
use sort::quicksort;
use data_structures::heap::MinHeap;
use sort::quicksort::Person;
use data_structures::bintree::BinNode;
use data_structures::trie;






#[test]
fn test_trie(){
    tests::test_trie();
    
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