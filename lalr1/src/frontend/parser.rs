use super::lexer::Lexer;
use super::token::{Token, TokenType, FILLER};
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;

//干!

type GrammarVar = String;
type GrammarTerminal = Token;

//语法变量与语法常量
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Clone)]
pub enum GrammarSymbol {
    Terminal(GrammarTerminal), //语法常量
    Variable(GrammarVar),      //语法变量
}

macro_rules! GTerminal {
    ($x:expr) => {
        GrammarSymbol::Terminal($x)
    };
}

macro_rules! GVariable {
    ($x:expr) => {
        GrammarSymbol::Variable($x)
    };
}

impl Default for GrammarSymbol {
    fn default() -> Self {
        GrammarSymbol::Terminal((FILLER.to_string(), TokenType::VareEpsilon))
    }
}
impl GrammarSymbol {
    pub fn is_terminal(&self) -> bool {
        match &self {
            GrammarSymbol::Terminal(_) => true,
            _ => false,
        }
    }
    pub fn is_variable(&self) -> bool {
        !self.is_terminal()
    }
    pub fn is_epsilon(&self) -> bool {
        match &self {
            GrammarSymbol::Terminal((_, TokenType::VareEpsilon)) => true,
            _ => false,
        }
    }
    pub fn unwrap_terminal(&self) -> &GrammarTerminal {
        match &self {
            GrammarSymbol::Terminal(t) => &t,
            _ => panic!("\n\t not terminal"),
        }
    }
}

type SymbolQueue = Vec<GrammarSymbol>;

type ItemLeft = GrammarVar;
type ItemRight = (usize, usize); //第一个是产生式的下标, 第二个usize代表下一个将要读取文法符号的的下标
                                 // type GrammarItem = BTreeMap<ItemRight, Vec<GrammarSymbol>>;
type GrammarItem = (ItemRight, Vec<GrammarTerminal>);

pub struct Grammar {
    pub produces: HashMap<GrammarVar, HashSet<usize>>, //左部->右部
    pub vec_produces: Vec<(GrammarVar, SymbolQueue)>,
    pub terminals: HashSet<GrammarSymbol>, //终结符
    pub vars: HashSet<GrammarSymbol>,      //变量
    pub start: GrammarSymbol,              //开始
}

#[derive(Debug)]
enum LRControll {
    Shift(usize),
    Reduce(ItemLeft, ItemRight),
    Error(String),
    GOTO(usize),
    ACC,
}

impl Default for LRControll {
    fn default() -> Self {
        LRControll::Error("default wrong".to_string())
    }
}

impl Grammar {
    ///empty Grammar_Struct
    pub fn new() -> Self {
        Grammar {
            produces: HashMap::new(),
            vec_produces: vec![], //S'为起始
            terminals: HashSet::new(),
            vars: HashSet::new(),
            start: GrammarSymbol::default(),
        }
    }

    ///vec包括拓广文法的S'
    pub fn from(vec: Vec<(GrammarVar, SymbolQueue)>, start: String) -> Self {
        let mut produces = HashMap::new();
        vec.iter().enumerate().for_each(|(i, (left, _))| {
            let m = produces.entry(left.clone()).or_insert(HashSet::new());
            m.insert(i);
        });

        Grammar {
            vars: produces
                .iter()
                .map(|(i, _)| GrammarSymbol::Variable(i.clone()))
                .collect(),

            produces: produces,
            vec_produces: vec,
            terminals: HashSet::new(),

            start: GrammarSymbol::Variable(start),
        }
    }

    //
    //产生式右部
    fn get_rights(&self, left: &GrammarSymbol) -> Vec<ItemRight> {
        match left {
            GrammarSymbol::Variable(left) => self
                .produces
                .get(left)
                .map(|right| right.iter().map(|x| (*x, 0)).collect())
                .unwrap_or_default(),
            _ => vec![],
        }
    }

    fn get_closure(&self, key_items: BTreeSet<GrammarItem>) -> BTreeSet<GrammarItem> {
        //通过核心项目得到闭包
        let mut closure: BTreeSet<GrammarItem> = key_items;
        //消除左递归
        loop {
            let a = closure.len();
            let mut tmp: BTreeSet<GrammarItem> = BTreeSet::new();
            closure
                .iter()
                .for_each(|((produces_index, index), look_ahead)| {
                    tmp = self
                        .get_rights(
                            self.vec_produces
                                .get(*produces_index)
                                .map(|(_, right)| right.get(*index))
                                .flatten()
                                .unwrap_or(&GrammarSymbol::default()),
                        )
                        .into_iter()
                        .map(|(pi, i)| {
                            (
                                (pi, i),
                                self.first(*produces_index, *index, &look_ahead.to_vec()),
                            )
                        })
                        .collect();
                });
            closure.extend(tmp);
            let b = closure.len();
            if a == b {
                break;
            }
        }
        
        closure
    }
    fn go(
        &self,
        old_items: &BTreeSet<GrammarItem>,
        eat_symbol: &GrammarSymbol,
    ) -> BTreeSet<GrammarItem> {
        let key_items: BTreeSet<GrammarItem> = old_items
            .iter()
            .filter(|((produces_index, index), la)| {
                self.vec_produces
                    .get(*produces_index)
                    .map(|(_, right)| right.get(*index))
                    .flatten()
                    == Some(eat_symbol)
            })
            .map(|((produces_index, index), la)| ((*produces_index, *index + 1), la.clone()))
            .collect();
        self.get_closure(key_items)
    }

    fn get_canonical_collection(
        &self,
    ) -> (
        Vec<BTreeSet<GrammarItem>>,
        HashMap<(usize, GrammarSymbol), usize>,
        Vec<usize>,
        BTreeMap<(usize, usize, usize), BTreeSet<GrammarTerminal>>,
    ) {
        let mut start_set = BTreeSet::new();
        start_set.insert(((0, 0), vec![("#".to_string(), TokenType::EOF)])); //S'为起始
        let mut tmp_canonical_collection: HashSet<BTreeSet<GrammarItem>> = HashSet::new();
        tmp_canonical_collection.insert(self.get_closure(start_set));
        let mut move_table = BTreeMap::new();
        let all_symbols = {
            let mut tmp = HashSet::new();
            tmp.extend(&self.terminals);
            tmp.extend(&self.vars);
            tmp
        };
        
        loop {
            let a = tmp_canonical_collection.len();

            let mut tmp_items: HashSet<BTreeSet<GrammarItem>> = HashSet::new();

            tmp_canonical_collection.iter().for_each(|closure| {
                all_symbols.iter().for_each(|eat_symbol| {
                    let next_closure = self.go(closure, eat_symbol);
                    
                    move_table.insert((closure.clone(), eat_symbol.clone()), next_closure.clone());
                    tmp_items.insert(next_closure.clone());
            
                });
            });
            tmp_canonical_collection.extend(tmp_items);
            let b = tmp_canonical_collection.len();
            if a == b {
                break;
            }
        }

        let canonical_collection: Vec<BTreeSet<GrammarItem>> =
            tmp_canonical_collection.into_iter().collect();

        let mut start = vec![];
        for (i, v) in canonical_collection.iter().enumerate() {
            if v == &self.get_closure({
                let mut start_set = BTreeSet::new();
                start_set.insert(((0, 0), vec![("#".to_string(), TokenType::EOF)]));
                start_set
            }) {
                start.push(i);
            }
        }

        let tmp: BTreeMap<&BTreeSet<GrammarItem>, usize> = canonical_collection
            .iter()
            .enumerate()
            .map(|(x, y)| (y, x))
            .collect();
        let simple_move_table: HashMap<(usize, GrammarSymbol), usize> = move_table
            .into_iter()
            .map(|((now, symbol), next)| {
                (
                    tmp.get(&now).map(|x| *x).unwrap_or_default(),
                    symbol.clone(),
                    tmp.get(&next).map(|x| *x).unwrap_or_default(),
                )
            })
            .map(|(now, symbol, next)| ((now, symbol), next))
            .collect();

        // println!("all closures are {:?}", canonical_collection);
        // println!("\t all move_table = {:?}", move_table);
        // println!("\t simple_table = {:?}", simple_move_table);
        let table = self.lalr1(&canonical_collection, &simple_move_table, all_symbols);
        (canonical_collection, simple_move_table, start, table)
    }

    fn lalr1(
        &self,
        canonical_collection: &Vec<BTreeSet<GrammarItem>>,
        simple_move_table: &HashMap<(usize, GrammarSymbol), usize>,
        all_symbols: HashSet<&GrammarSymbol>,
    ) -> BTreeMap<(usize, usize, usize), BTreeSet<GrammarTerminal>> {
        let mut bc = BTreeMap::new();
        let mut table = BTreeMap::new();
        //lalr(1)的分析开始!
        //1.传播符表和计算表的建立
        // panic!("canonical_collection = {:?}",canonical_collection);
        let key_items: Vec<BTreeSet<GrammarItem>> = canonical_collection
            .iter()
            .map(|set| {
                set.iter()
                    .filter(|((pi, i), la)| *i != 0 || (*i == 0 && *pi == 0)) //wa!!!
                    .map(|((pi, i), la)| ((*pi, *i), la.clone()))
                    .collect()
            })
            .collect();
        key_items.iter().enumerate().for_each(|(goto_i, K)| {
            K.iter().for_each(|((pi, i), la)| {
                //for A->alpha . beta in K
                let j = self.get_closure({
                    let mut t = BTreeSet::new();
                    t.insert(((*pi, *i), vec![("#".to_string(), TokenType::EOF)]));
                    t
                });

                all_symbols.iter().for_each(|X| {
                    let goto_i_x = simple_move_table
                        .get(&(goto_i, X.clone().clone()))
                        .map(|a| *a)
                        .unwrap();

                    if let Some(Itemj) = key_items.get(goto_i_x) {
                        Itemj.iter().for_each(|((pi2, i2), nla)| {
                            if j.contains(&((*pi2, i2 - 1), nla.to_vec()))
                                && la != &vec![("#".to_string(), TokenType::EOF)]
                            {
                                //自发
                                // todo!();
                                //(闭包下标, 产生式下标)
                                table
                                    .entry((goto_i, *pi, *i))
                                    .or_insert(BTreeSet::new())
                                    .extend(la.to_vec());
                                table
                                    .entry((goto_i_x, *pi2, *i2))
                                    .or_insert(BTreeSet::new())
                                    .extend(nla.to_vec());
                            }
                            if j.contains(&(
                                (*pi2, i2 - 1),
                                vec![("#".to_string(), TokenType::EOF)],
                            )) {
                                //传播
                                // todo!();
                                bc.entry((goto_i, *pi, *i))
                                    .or_insert(BTreeSet::new())
                                    .insert(((goto_i_x, pi2, i2), la));
                            }
                        });
                    }
                });
            });
        });

        //2.由传播符表来填充计算表
        bc.iter().for_each(|((ci, pi, i), vec_next)| {
            vec_next.iter().for_each(|((nci, npi, ni), nla)| {
                table
                    .entry((*nci, **npi, **ni))
                    .or_insert(nla.to_vec().into_iter().collect())
                    .extend(nla.to_vec());
            });
        });

        // panic!(
        //     "\n \t 1.broadcast = {:?} \n\t 2.table_compute = {:?} \n\t3. cc={:?}",
        //     bc, table, canonical_collection
        // );
        table
    }

    fn build_lalr1_table(&self) -> (HashMap<(usize, GrammarSymbol), LRControll>, Vec<usize>) {
        let (canonical_collection, move_table, start, latable) = self.get_canonical_collection();
        let mut lalr1_table: HashMap<(usize, GrammarSymbol), LRControll> = HashMap::new();
        // panic!(
        //     "\n\t cc = {:?} \n\t la={:?}",
        //     canonical_collection,
        //     latable
        // );
        canonical_collection
            .into_iter()
            .enumerate()
            .for_each(|(k, closure)| {
                closure
                    .into_iter()
                    .for_each(|((produces_index, index), la)| {
                        match self
                            .vec_produces
                            .get(produces_index)
                            .map(|(A, rights)| (A, rights.get(index)))
                        {
                            Some((_A, Some(syb))) => match syb {
                                GrammarSymbol::Terminal(a) => {
                                    if let Some(j) = move_table.get(&(k, GTerminal!(a.clone()))) {
                                        lalr1_table.insert(
                                            (k, GTerminal!(a.clone())),
                                            LRControll::Shift(*j),
                                        );
                                    }
                                }
                                GrammarSymbol::Variable(B) => {
                                    if let Some(j) =
                                        move_table.get(&(k, GrammarSymbol::Variable(B.to_string())))
                                    {
                                        lalr1_table.insert(
                                            (k, GrammarSymbol::Variable(B.to_string())),
                                            LRControll::GOTO(*j),
                                        );
                                    }
                                }
                                _ => panic!(
                                    "\n \tbuild lr0_table failed have GrammarSymbol::Epsilon"
                                ),
                            },
                            Some((A, None)) => {
                                if A == &"S'".to_string() {
                                    lalr1_table.insert(
                                        (k, GTerminal!(("#".to_string(), TokenType::EOF))),
                                        LRControll::ACC,
                                    );
                                } else {
                                    let left = self
                                        .vec_produces
                                        .get(produces_index)
                                        .map(|(a, _)| a.clone())
                                        .unwrap_or(FILLER.to_string());
                                    // panic!("there lalar_move_table = {:?} \n\t k = {}, pi = {},index={} \n\t la={:?}", latable, k,produces_index,index,la);
                                    let tmp: HashMap<(usize, GrammarSymbol), LRControll> = latable
                                        .get(&(k, produces_index, index))
                                        .map(|la| {
                                            // let tmp: HashMap<(usize, GrammarSymbol), LRControll> =
                                            la.iter()
                                                .map(|las| {
                                                    (
                                                        (k, GTerminal!(las.clone())),
                                                        LRControll::Reduce(
                                                            left.to_string(),
                                                            (produces_index, index),
                                                        ),
                                                    )
                                                })
                                                .collect()
                                        })
                                        .unwrap();

                                    lalr1_table.insert(
                                        (k, GTerminal!(("#".to_string(), TokenType::EOF))),
                                        LRControll::Reduce(
                                            left.to_string(),
                                            (produces_index, index),
                                        ),
                                    );
                                    lalr1_table.extend(tmp);
                                }
                            }
                            _ => panic!("vec_pro index out of bounds"),
                        }
                    });
            });
        // println!("\t lr0_table = {:?}", lr0_table);
        (lalr1_table, start)
    }

    pub fn run(&self, input: &str) {
        use LRControll::*;
        let mut lexer = Lexer::new(input);

        let (lalr1_table, start) = self.build_lalr1_table();

        let mut symbols = vec![GTerminal!(("#".to_string(), TokenType::EOF))];
        let mut states = start;

        let mut now_token = lexer.next_token().unwrap();

        //TODO 终结符如何与非终结符区分?
        loop {
            let now_state = states.last().map(|x| *x).unwrap();
            match lalr1_table
                .get(&(now_state, GTerminal!(now_token.clone())))
                .unwrap_or(&LRControll::default())
            {
                Shift(next_state) => {
                    symbols.push(GTerminal!(now_token));
                    states.push(*next_state);
                    now_token = lexer.next_token().unwrap();
                }
                Reduce(left, (right, _)) => {
                    for _ in 0..self
                        .vec_produces
                        .get(*right)
                        .map(|(_, r)| r.len())
                        .unwrap_or(0)
                    {
                        symbols.pop();
                        states.pop();
                    }

                    let next_state = lalr1_table.get(&(
                        states.last().map(|x| *x).unwrap(),
                        GVariable!(left.to_string()),
                    ));
                    match next_state {
                        Some(GOTO(next_s)) => states.push(*next_s),
                        _ => panic!("\n\t 1. \twant goto but have others={:?} \n\t 2. the env is {:?}, {:?} \n\t 3. the stack is {:?} {:?} \n\r 4. table = {:?}", next_state, now_state, left,symbols,states, lalr1_table),
                    }
                    symbols.push(GVariable!(left.to_string()));
                }
                ACC => break,
                _ => panic!(
                    "\n\t 1. the state is {:?} \t input is {:?} \n\t 2. the stack is {:?} && {:?} \n\t3. the table is {:?}",
                    now_state, now_token, symbols, states, lalr1_table
                ),
            }
        }
    }

    //求First(beta z)
    fn first(&self, pi: usize, i: usize, z: &Vec<GrammarTerminal>) -> Vec<GrammarTerminal> {
        let beta = self
            .vec_produces
            .get(pi)
            .map(|(a, b)| b.get(i + 1))
            .flatten(); //这里的加一  A->a.X \beta, z
        if let Some(beta) = beta {
            let mut Y = beta;
            let mut tmp: BTreeSet<GrammarTerminal> = BTreeSet::new();
            loop {
                match Y {
                    GrammarSymbol::Terminal(a) => {
                        tmp.insert(a.clone());
                        break;
                    }
                    GrammarSymbol::Variable(v) => {
                        self.produces.get(v).map(|Xs| {
                            Xs.iter().for_each(|pi| {
                                //X -> Y1Y2...Yn
                                let pro_tmp = self.vec_produces.get(*pi);
                                let mut i = 0;
                                loop {
                                    match pro_tmp.map(|(_, r)| r.get(i)).flatten() {
                                        Some(Yi) => {
                                            if Yi.is_epsilon() {
                                                i += 1;
                                            } else if Yi.is_terminal() {
                                                tmp.insert(Yi.unwrap_terminal().clone());
                                                break;
                                            } else {
                                                Y = &Yi;
                                                break;
                                            }
                                        }
                                        _ => {
                                            tmp.extend(z.clone());
                                            break;
                                        }
                                    }
                                }
                            });
                        });
                    }
                }
            }
            tmp.into_iter().collect()
        } else {
            z.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lalr1_run() {
        let v = vec![
            (
                "S'".to_string(),
                vec![GrammarSymbol::Variable("S".to_string())],
            ),
            (
                "S".to_string(),
                vec![GrammarSymbol::Terminal(("a".to_string(), TokenType::IDENT))],
            ),
            (
                "S".to_string(),
                vec![
                    GrammarSymbol::Terminal(("a".to_string(), TokenType::IDENT)),
                    GrammarSymbol::Variable("S".to_string()),
                ],
            ),
        ];
        let mut g = Grammar::from(v, "S".to_string());
        let mut ts = HashSet::new();
        ts.insert(GrammarSymbol::Terminal(("a".to_string(), TokenType::IDENT)));
        g.terminals = ts;
        g.run(&"a a a  #");
    }
}
