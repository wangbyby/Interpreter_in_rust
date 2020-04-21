use super::token::{Token, TokenType, FILLER};
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;

macro_rules! make_token {
    ($literal:expr, $type:ident) => {
        (format!("{}", $literal), TokenType::$type)
    };
}


type TheToken = Token;
type TokenQueue = Vec<TheToken>;

type ItemLeft = TheToken;
type ItemRight = (TokenQueue, usize); //这个usize代表下一个将要读取的下标

/// 第一个是产生式左部, 第二个是产生式右部中[点左边], 第三个是点后直接跟的, 第四个是剩余部分.
/// Ex. S'->.S : (S', "",S,"")
/// S -> a. : (S,a,"","")
/// S -> a.c : (S, a, c,"")
/// S -> a.cd : (S, a, c,d)
type GrammerItem = (ItemLeft, ItemRight);

pub struct Grammer {
    pub produces: HashMap<String, Vec<TokenQueue>>, //左部->右部
    pub terminals: HashSet<String>,                 //终结符
    pub vars: HashSet<String>,                      //变量
    pub start: TheToken,                            //开始
}

#[derive(Debug)]
enum LRControll {
    Shift(usize),
    Reduce(ItemLeft, ItemRight),
    Error(String),
    GOTO(usize),
    ACC,
}

impl Grammer {
    pub fn new() -> Self {
        Grammer {
            produces: HashMap::new(),
            terminals: HashSet::new(),
            vars: HashSet::new(),
            start: make_token!(FILLER, GVariable),
        }
    }
    //产生式右部
    fn get_rights(&self, left: &String) -> Vec<ItemRight> {
        self.produces
            .get(left)
            .map(|right| right.iter().map(|x| ((*x).clone(), 0)).collect())
            .unwrap_or_default()
    }

    fn get_closure(&self, key_items: BTreeSet<GrammerItem>) -> BTreeSet<GrammerItem> {
        //通过核心项目得到闭包
        let mut closure: BTreeSet<GrammerItem> = BTreeSet::new();
        key_items.into_iter().for_each(|(left, (queue, index))| {
            let tmp: BTreeSet<GrammerItem> = self
                .get_rights(
                    queue
                        .get(index)
                        .map(|(a, _)| a)
                        .unwrap_or(&FILLER.to_string()),
                )
                .into_iter()
                .filter(|_| queue.get(index).is_some())
                .map(|x| (queue.get(index).map(|a| a.clone()).unwrap(), x))
                .collect();
            closure.extend(tmp);
            closure.insert((left, (queue, index)));
        });
        closure
    }
    fn go(&self, old_items: &BTreeSet<GrammerItem>, eat_symbol: &String) -> BTreeSet<GrammerItem> {
        let key_items: BTreeSet<GrammerItem> = old_items
            .iter()
            .filter(|(_, (queue, index))| queue.get(*index).map(|a| &a.0) == Some(eat_symbol))
            .map(|(left, (queue, index))| (left.clone(), (queue.clone(), index + 1)))
            .collect();
        self.get_closure(key_items)
    }

    fn get_canonical_collection(
        &self,
    ) -> (
        Vec<BTreeSet<GrammerItem>>,
        HashMap<(usize, std::string::String), usize>,
    ) {
        let mut start_set = BTreeSet::new();
        start_set.insert((make_token!("S'", GVariable), (vec![self.start.clone()], 0)));
        let mut canonical_collection = vec![self.get_closure(start_set)];
        let mut move_table = BTreeMap::new();
        let mut nn = 0;
        
        loop {
            for eat_symbol in &self.terminals | &self.vars {
                let closure = &canonical_collection[nn];
                let next_closure: BTreeSet<GrammerItem> = self.go(closure, &eat_symbol.to_string());
                if next_closure.len() != 0 {
                    //TODO 用Cell改写?
                    move_table.insert((closure.clone(), eat_symbol.clone()), next_closure.clone());
                    canonical_collection.push(next_closure);
                }
            }
            nn += 1;
            if canonical_collection.len() <= nn {
                break;
            }
        }
        let tmp: BTreeMap<&BTreeSet<GrammerItem>, usize> = canonical_collection
            .iter()
            .enumerate()
            .map(|(x, y)| (y, x))
            .collect();
        let simple_move_table: HashMap<(usize, String), usize> = move_table
            .iter()
            .map(|((now, symbol), next)| {
                (
                    tmp.get(now).unwrap(),
                    symbol.to_string(),
                    tmp.get(next).unwrap(),
                )
            })
            .map(|(now, symbol, next)| ((*now, symbol), *next))
            .collect();

        // println!("all closures are {:?}", canonical_collection);
        // println!("\t all move_table = {:?}", move_table);
        // println!("\t simple_table = {:?}", simple_move_table);

        (canonical_collection, simple_move_table)
    }

    fn build_lr0_table(&self) -> HashMap<(usize, String), LRControll> {
        let (canonical_collection, move_table) = self.get_canonical_collection();
        let mut lr0_table: HashMap<(usize, String), LRControll> = HashMap::new();

        canonical_collection
            .into_iter()
            .enumerate()
            .for_each(|(k, closure)| {
                closure
                    .into_iter()
                    .for_each(|(left, (queue, index))| match queue.get(index) {
                        Some((want, t)) => {
                            if left.0 == "S'".to_string()
                                && queue.len() == 1
                                && (want, t) == (&self.start.0,&self.start.1)
                                && index == queue.len()
                            {
                                lr0_table.insert((k, "#".to_string()), LRControll::ACC);
                            } else if self.terminals.contains(want) {
                                if let Some(j) = move_table.get(&(k, want.to_string())) {
                                    lr0_table.insert((k, want.to_string()), LRControll::Shift(*j));
                                }
                            } else if self.vars.contains(want) {
                                if let Some(j) = move_table.get(&(k, want.to_string())) {
                                    lr0_table.insert((k, want.to_string()), LRControll::GOTO(*j));
                                }
                            }
                        }
                        _ => {
                            let tmp: HashMap<(usize, String), LRControll> = self
                                .terminals
                                .iter()
                                .map(|next| {
                                    ((k, next.to_string()), LRControll::Reduce(left.clone(), (queue.clone(),index)))
                                })
                                .collect();
                            lr0_table
                                .insert((k, "#".to_string()), LRControll::Reduce(left, (queue, index)));
                            lr0_table.extend(tmp);
                        }
                    });
            });
        // println!("\t lr0_table = {:?}", lr0_table);
        lr0_table
    }

    pub fn run(&self, input: String) {
        use LRControll::*;
        let lr0_table = self.build_lr0_table();
        let mut symbols = vec!["#".to_string()];
        let mut states = vec![0];
        let mut input = input.chars();
        let mut now_input = input.next().unwrap();

        //TODO 终结符如何与非终结符区分?
        loop {
            let now_state = states.last().map(|x| *x).unwrap();
            match lr0_table
                .get(&(now_state, now_input.to_string()))
                .unwrap_or(&Error("wrong".to_string()))
            {
                Shift(next_state) => {
                    symbols.push(now_input.to_string());
                    states.push(*next_state);
                    now_input = input.next().unwrap();
                }
                Reduce(left, (right,_)) => {
                    for _ in 0..right.len() {
                        symbols.pop();
                        states.pop();
                    }

                    let next_state =
                        lr0_table.get(&(states.last().map(|x| *x).unwrap(), left.0.to_string()));
                    match next_state {
                        Some(GOTO(next_s)) => states.push(*next_s),
                        _ => panic!("\n\t 1. \twant goto but have others={:?} \n\t 2. the env is {:?}, {:?} \n\t 3. the stack is {:?} {:?}", next_state, now_state, left,symbols,states),
                    }
                    symbols.push(left.0.to_string());
                }
                ACC => break,
                _ => panic!(
                    "\n\t 1. the state is {:?} \t input is {:?} \n\t 2. the stack is {:?} {:?}",
                    now_state, now_input, symbols, states
                ),
            }
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test_lr0_run() {
//         let mut g = Grammer::new();
//         g.start = make_token!('S', GVariable);
//         let mut t = HashSet::new();
//         t.insert("a".to_string());
//         t.insert("b".to_string());
//         g.terminals = t;

//         let mut v = HashSet::new();
//         v.insert("S".to_string());
//         v.insert("B".to_string());
//         g.vars = v;
//         let mut p = HashMap::new();
//         p.insert("S".to_string(), vec!["a".to_string(), "b".to_string()]);

//         g.produces = p;

//         g.run("a#".to_string());
//     }
// }
