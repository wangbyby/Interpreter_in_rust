use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
enum LRControll {
    Shift(usize),
    Reduce(String, String),
    Error(String),
    GOTO(usize),
    ACC,
}

/// 第一个是产生式左部, 第二个是产生式右部中[点左边], 第三个是点后直接跟的, 第四个是剩余部分.
/// Ex. S'->.S : (S', "",S,"")
/// S -> a. : (S,a,"","")
/// S -> a.c : (S, a, c,"")
/// S -> a.cd : (S, a, c,d)
type GrammerItem = (String, String, String, String);

pub struct Grammer {
    pub produces: HashMap<String, Vec<String>>, //左部->右部
    pub terminals: HashSet<String>,             //终结符
    pub vars: HashSet<String>,                  //变量
    pub start: String,                          //开始
}

impl Grammer {
    pub fn new() -> Self {
        Grammer {
            produces: HashMap::new(),
            terminals: HashSet::new(),
            vars: HashSet::new(),
            start: String::new(),
        }
    }

    //产生式右部
    fn get_rights(&self, left: &String) -> Option<Vec<(String, String, String)>> {
        self.produces.get(left).map(|right| {
            right
                .iter()
                .map(|x| x.chars())
                .map(|mut x| {
                    (
                        "".to_string(),
                        x.next().map(|c| c.to_string()).unwrap_or(String::new()),
                        x.as_str().to_string(),
                    )
                })
                .collect()
        })
    }

    fn get_closure(&self, key_items: BTreeSet<GrammerItem>) -> BTreeSet<GrammerItem> {
        //通过核心项目得到闭包
        let mut closure: BTreeSet<GrammerItem> = BTreeSet::new();
        key_items.into_iter().for_each(|it| {
            self.get_rights(&it.2)
                .unwrap_or_default()
                .into_iter()
                .for_each(|r| {
                    closure.insert((it.2.clone(), r.0, r.1, r.2));
                });
            closure.insert(it);
        });
        closure
    }

    fn go(
        &self,
        old_items: &BTreeSet<GrammerItem>,
        // len: usize,
        eat_symbol: &String,
    ) -> BTreeSet<GrammerItem> {
        let key_items: BTreeSet<GrammerItem> = old_items
            .iter()
            .filter(|(_, _, want, _)| *want == *eat_symbol)
            .map(|(left, beforedot, want, after)| {
                let mut after_chars = after.chars();
                let next_want = after_chars
                    .next()
                    .map(|x| x.to_string())
                    .unwrap_or_default();
                let after_str = after_chars.as_str().to_string();
                (
                    left.clone(),
                    [beforedot.clone(), want.clone()].concat(),
                    next_want,
                    after_str,
                )
            })
            .collect();
        self.get_closure(key_items)
        // (self.get_closure(key_items), len + 1)
    }

    fn get_canonical_collection(
        &self,
    ) -> (
        Vec<BTreeSet<GrammerItem>>,
        HashMap<(usize, std::string::String), usize>,
    ) {
        let mut start_set = BTreeSet::new();
        start_set.insert((
            "S'".to_string(),
            "".to_string(),
            self.start.to_owned(),
            "".to_string(),
        ));
        let mut canonical_collection = vec![self.get_closure(start_set)];
        let mut move_table = HashMap::new();
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
        let tmp: HashMap<&BTreeSet<GrammerItem>, usize> = canonical_collection
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
                closure.into_iter().for_each(|(left, before, want, after)| {
                    if left == "S'".to_string()
                        && before == self.start
                        && want.len() == 0
                        && after.len() == 0
                    {
                        lr0_table.insert((k, "#".to_string()), LRControll::ACC);
                    } else if self.terminals.contains(&want) {
                        if let Some(j) = move_table.get(&(k, want.to_string())) {
                            lr0_table.insert((k, want), LRControll::Shift(*j));
                        }
                    } else if self.vars.contains(&want) {
                        if let Some(j) = move_table.get(&(k, want.to_string())) {
                            lr0_table.insert((k, want), LRControll::GOTO(*j));
                        }
                    } else if want.len() == 0 && after.len() == 0 {
                        for a in &self.terminals {
                            lr0_table.insert(
                                (k, a.to_string()),
                                LRControll::Reduce(
                                    left.clone(),
                                    [before.clone(), want.clone(), after.clone()].concat(),
                                ),
                            );
                        }
                        lr0_table.insert(
                            (k, "#".to_string()),
                            LRControll::Reduce(left, [before, want, after].concat()),
                        );
                    }
                });
            });
        // println!("\t lr0_table = {:?}", lr0_table);
        lr0_table
    }

    pub fn run(&self, input: String) {
        use LRControll::*;
        let lr0_table = self.build_lr0_table();
        let mut symbols = Vec::new();
        let mut states = Vec::new();
        symbols.push("#".to_string());
        states.push(0);
        let mut input = input.chars();
        let mut now_input = input.next().unwrap();
        loop {
            let now_state = states.last().map(|x| *x).unwrap(); //TODO
            match lr0_table
                .get(&(now_state, now_input.to_string()))
                .unwrap_or(&Error("wrong".to_string()))
            {
                Shift(next_state) => {
                    symbols.push(now_input.to_string());
                    states.push(*next_state);
                    now_input = input.next().unwrap();
                }
                Reduce(left, right) => {
                    let mut len = right.len();
                    while len > 0 {
                        symbols.pop();
                        states.pop();
                        len -= 1;
                    }

                    let next_state =
                        lr0_table.get(&(states.last().map(|x| *x).unwrap(), left.to_string()));
                    match next_state {
                        Some(GOTO(next_s)) => states.push(*next_s),
                        _ => panic!("\n\t 1. \twant goto but have others={:?} \n\t 2. the env is {:?}, {:?} \n\t 3. the stack is {:?} {:?}", next_state, now_state, left,symbols,states),
                    }
                    symbols.push(left.to_string());
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lr0_run() {
        let mut g = Grammer::new();
        g.start = "S".to_string();
        let mut t = HashSet::new();
        t.insert("a".to_string());
        t.insert("b".to_string());
        g.terminals = t;

        let mut v = HashSet::new();
        v.insert("S".to_string());
        v.insert("B".to_string());
        g.vars = v;
        let mut p = HashMap::new();
        p.insert("S".to_string(), vec!["a".to_string(), "b".to_string()]);

        g.produces = p;

        g.run("a#".to_string());
    }
}
