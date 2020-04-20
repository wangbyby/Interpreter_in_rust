pub mod frontend;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
use frontend::parser::Grammer;

fn main() {

    let mut g = Grammer::new();
        g.start = "S".to_string();
        let mut t = HashSet::new();
        t.insert("a".to_string());
        t.insert("b".to_string());
        g.terminals = t;

        let mut v = HashSet::new();
        v.insert("S".to_string());
        // v.insert("B".to_string());
        g.vars = v;
        let mut p = HashMap::new();
        p.insert("S".to_string(), vec!["a".to_string(), "b".to_string()]);

        g.produces = p;

        g.run("a#".to_string());

}