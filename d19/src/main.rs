#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections::HashMap;
use std::io::{self, Read, Write};

use regex::Regex;

type Rule = Vec<String>;

#[derive(Clone)]
struct BinTree {
    a: Option<Box<BinTree>>,
    b: Option<Box<BinTree>>,
}

impl BinTree {
    fn new() -> Self {
        BinTree { a: None, b: None }
    }

    fn from_trees(a: Self, b: Self) -> Self {
        BinTree {
            a: Some(Box::new(a)),
            b: Some(Box::new(b)),
        }
    }

    fn add_node(&mut self, node: Self, val: char) {
        match val {
            'a' => self.a = Some(Box::new(node)),
            'b' => self.b = Some(Box::new(node)),
            _ => panic!("invalid val"),
        }
    }

    fn add_leaf(&mut self, leaf: &Self) {
        if let Some(a) = &mut self.a {
            a.add_leaf(leaf);
        }
        if let Some(b) = &mut self.b {
            b.add_leaf(leaf);
        }
        if self.is_leaf() {
            if let Some(a) = leaf.a.clone() {
                self.a = Some(a);
            }
            if let Some(b) = leaf.b.clone() {
                self.b = Some(b.clone());
            }
        }
    }

    fn is_leaf(&self) -> bool {
        !(self.has_a() || self.has_b())
    }

    fn has_a(&self) -> bool {
        self.a.is_some()
    }

    fn has_b(&self) -> bool {
        self.b.is_some()
    }

    fn has_only_a(&self) -> bool {
        self.has_a() && !self.has_b()
    }

    fn has_only_b(&self) -> bool {
        !self.has_a() && self.has_b()
    }

    fn matches(&self, chars: &mut std::str::Chars) -> bool {
        if let Some(c) = chars.next() {
            match c {
                'a' => {
                    if let Some(a) = &self.a {
                        return a.matches(chars);
                    } else {
                        return false;
                    }
                }
                'b' => {
                    if let Some(b) = &self.b {
                        return b.matches(chars);
                    } else {
                        return false;
                    }
                }
                _ => panic!(),
            }
        } else {
            if self.is_leaf() {
                true
            } else {
                false
            }
        }
    }
}

fn parse_rule0(rules: &HashMap<usize, String>) -> BinTree {
    parse_rule(rules, 0)
}

struct SpecRules {
    rule31: Rule,
    rule42: Rule,
}

fn add_rule8(rule42: &Rule, n: usize) -> Rule {
    println!("Adding Rule8...");
    let mut iterations: Vec<Rule> = vec![vec![String::new()]];
    for _ in 0..n {
        let last_iter = iterations.last();

        let mut new_iter: Rule = Vec::new();
        for last in last_iter {
            for r in rule42 {
                for l in last {
                    let mut new = l.clone();
                    new.push_str(&r.clone());
                    new_iter.push(new);
                }
            }
        }
        iterations.push(new_iter);
    }
    let mut rule = Vec::new();

    for mut iter in &mut iterations {
        rule.append(&mut iter);
    }
    println!("Added");
    rule
}

fn add_rule11(rule31: &Rule, rule42: &Rule, n: usize) -> Rule {
    Vec::new()
}

fn merge_trees(left: BinTree, right: BinTree) -> BinTree {
    if left.is_leaf() {
        return right;
    }
    if right.is_leaf() {
        return left;
    }

    let mut ret = BinTree::new();

    if left.has_a() && right.has_a() {
        let a = merge_trees(*left.a.clone().unwrap(), *right.a.clone().unwrap());

        if !a.is_leaf() {
            ret.a = Some(Box::new(a));
        }
    }

    if left.has_b() && right.has_b() {
        let b = merge_trees(*left.b.clone().unwrap(), *right.b.clone().unwrap());

        if !b.is_leaf() {
            ret.b = Some(Box::new(b));
        }
    }

    if !ret.has_a() {
        if left.has_a() {
            ret.a = Some(left.a.clone().unwrap());
        }
        if right.has_a() {
            ret.a = Some(right.a.clone().unwrap());
        }
    }
    if !ret.has_b() {
        if left.has_b() {
            ret.b = Some(left.b.clone().unwrap());
        }
        if right.has_b() {
            ret.b = Some(right.b.clone().unwrap());
        }
    }

    ret
}

fn parse_rule(rules: &HashMap<usize, String>, index: usize) -> BinTree {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[ab]").unwrap();
    }

    let rule = &rules.get(&index).unwrap();

    if RE.is_match(rule) {
        let c: &str = rule.split("\"").collect::<Vec<&str>>()[1];
        return match c {
            "a" => BinTree {
                a: Some(Box::new(BinTree::new())),
                b: None,
            },
            "b" => BinTree {
                a: None,
                b: Some(Box::new(BinTree::new())),
            },
            _ => panic!("bad char"),
        };
    }

    let mut branches: Vec<BinTree> = Vec::new();
    for or in rule.split("|") {
        let mut indices = Vec::new();

        for s in or.split(" ") {
            if let Ok(x) = s.parse::<usize>() {
                indices.push(x);
            }
        }

        let mut branch: Vec<BinTree> = Vec::new();
        for i in indices.iter().rev() {
            branch.push(parse_rule(rules, *i));
        }

        let mut leaf = branch[0].clone();
        for i in 1..branch.len() {
            let mut but = branch[i].clone();

            but.add_leaf(&leaf);

            leaf = but;
        }

        branches.push(leaf);
    }

    if let Some(start) = branches.get(0).clone() {
        let merged = branches
            .iter()
            .fold(start.clone(), |acc, x| merge_trees(acc, x.clone()));

        return merged;
    }

    panic!()
}

fn is_match(rule: &BinTree, msg: &str) -> bool {
    let mut chars = msg.chars();

    rule.matches(&mut chars)
}

fn calc_matches(rule: &BinTree, msgs: &[&str]) -> u64 {
    let mut count = 0;
    let mut msg_i = 0;
    for msg in msgs {
        if is_match(rule, msg) {
            count += 1;
        }

        msg_i += 1;
        println!("{}/{} matches", count, msg_i);
    }

    count
}

fn rules2map(unsorted: &[&str]) -> HashMap<usize, String> {
    let mut map: HashMap<usize, String> = HashMap::new();
    for us in unsorted {
        let split: Vec<&str> = us.split(":").collect();

        map.insert(split[0].to_string().parse().unwrap(), split[1].to_string());
    }

    map
}

fn vis_tree(tree: &BinTree) {
    let dep = 0;
    if let Some(abox) = &tree.a {
        println!("{}: a", dep);
        vis_node(&abox, 1);
    }
    if let Some(bbox) = &tree.b {
        println!("{}: b", dep);
        vis_node(&bbox, 1);
    }
}

fn vis_node(tree: &BinTree, dep: usize) {
    if let Some(abox) = &tree.a {
        println!("{}: a", dep);
        vis_node(&abox, dep + 1);
    }
    if let Some(bbox) = &tree.b {
        println!("{}: b", dep);
        vis_node(&bbox, dep + 1);
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let mut unsorted = Vec::new();
    while let Some(line) = lines.next() {
        let trim = line.trim();
        if trim == "" {
            break;
        }
        unsorted.push(trim);
    }

    let rules = rules2map(&unsorted);

    println!("Parsing rules...");
    //    let rule42 = parse_rule(&rules, None, 42);
    //    let rule31 = parse_rule(&rules, None, 31);
    //
    //    let spec = SpecRules { rule42, rule31 };

    let rule0 = parse_rule0(&rules);
    println!("Parsed");

    //    vis_tree(&rule0);

    let messages: Vec<&str> = lines.map(|x| x).collect();

    let matches = calc_matches(&rule0, &messages);

    writeln!(io::stdout(), "{} matches", matches)?;

    Ok(())
}
