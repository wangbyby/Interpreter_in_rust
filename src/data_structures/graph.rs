
use std::collections::HashMap;
//图 邻接矩阵表示
pub struct Graph<T> where T: std::marker::Copy{
    payload : Vec<Vec<T>>,
    pub n_v : usize,
    pub n_e: usize,
    default_value: T,
}

impl<T> Graph<T> T: std::marker::Copy{
    pub fn with_default_value(default_val: T, capacity: usize) -> Self {
        Graph { payload: vec![vec![default_val; capacity]; capacity], 
            n_e: 0, 
            n_v: capacity, 
            default_value: default_val,
        }
    }
    
    /// only set one value 
    pub fn set_one(&mut self, a: usize, b: usize, value : T) {
        self.payload[a][b] = value;
    }
    ///only set one row
    pub fn set_row(&mut self, a: usize, row: Vec<T>) {
        self.payload[a].clone_from(&row);
    }

    pub fn child_nodes(&mut self, a: usize) -> Vec<T> {
        self.payload[a].clone()
        //self.payload[a].iter().filter(|&x| *x!=default_value).collect::<Vec<_>>()
    }
    pub fn child_nodes_hashmap(&mut self, a: usize) -> HashMap<usize,T> {
        self.payload[a].clone().iter().enumerate().filter(|&(x,y)| *y!=self.default_value).collect::<HashMap<_,_>>()
    }
    // //返回最小生成树
    // pub fn prim(&mut self) -> Vec<usize> {

    // }
    
    // pub fn dijkstra(&mut self, start: usize) -> (HashMap<usize,usize>, HashMap<usize,T>) {
        
    // }
}