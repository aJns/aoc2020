#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections::HashMap;
use std::io::{self, Read, Write};

use regex::Regex;

#[derive(Clone)]
struct BinTree {
    a: Option<Box<BinTree>>,
    b: Option<Box<BinTree>>,
}

impl BinTree {
    fn new() -> Self {
        BinTree { a: None, b: None }
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
                self.b = Some(b);
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

    // returns chars_left, is_leaf(), and last used char
    fn matches(&self, chars: &mut std::str::Chars) -> (bool, bool, char) {
        if let Some(c) = chars.next() {
            match c {
                'a' => {
                    if let Some(a) = &self.a {
                        print!("a");
                        return a.matches(chars);
                    } else {
                        (true, self.is_leaf(), c)
                    }
                }
                'b' => {
                    if let Some(b) = &self.b {
                        print!("b");
                        return b.matches(chars);
                    } else {
                        (true, self.is_leaf(), c)
                    }
                }
                _ => panic!(),
            }
        } else {
            println!("\n{}|{}", self.has_a(), self.has_b());
            (false, self.is_leaf(), ' ')
        }
    }
}

struct Rule0 {
    rule31: BinTree,
    rule42: BinTree,
}

fn parse_rule0(rules: &HashMap<usize, String>) -> Rule0 {
    let rule31 = parse_rule(rules, 31);
    let rule42 = parse_rule(rules, 42);

    Rule0 { rule31, rule42 }
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

    match index {
        0 => panic!(),
        8 => panic!(),
        11 => panic!(),
        _ => (),
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
            let parsed = parse_rule(rules, *i);
            if !parsed.is_leaf() {
                branch.push(parsed);
            }
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

fn is_match(rule: &Rule0, msg: &str) -> bool {
    let rule31 = &rule.rule31;
    let rule42 = &rule.rule42;

    let mut work_str = msg.to_string();

    let mut rule42_hits = 0;
    loop {
        // returns chars_left, is_leaf(), and last used char
        let mut chars = work_str.chars();
        let (chars_left, leaf, last_c) = rule42.matches(&mut chars);
        if leaf {
            rule42_hits += 1;
        }

        if !chars_left {
            return false;
        } else {
            if !leaf {
                break;
            }
        }
        let mut temp = last_c.to_string();
        temp.extend(chars);
        work_str = temp;
    }
    let mut rule31_hits = 0;
    loop {
        let mut chars = work_str.chars();
        let (chars_left, leaf, last_c) = rule31.matches(&mut chars);
        if leaf {
            rule31_hits += 1;
        }

        if !chars_left {
            if leaf {
                break;
            } else {
                return false;
            }
        }
        let mut temp = last_c.to_string();
        temp.extend(chars);
        work_str = temp;
    }
    if rule31_hits >= 1 && rule31_hits < rule42_hits {
        return true;
    }
    false
}

fn calc_matches(rule: &Rule0, msgs: &[&str]) -> u64 {
    let mut count = 0;
    let mut msg_i = 0;

    let mut vec = Vec::new();
    for msg in msgs {
        println!("--------");
        println!("{}", msg);
        if is_match(rule, msg) {
            count += 1;
            vec.push(msg);
        }

        msg_i += 1;
        println!("");
        println!("{}/{} matches", count, msg_i);
    }
    println!("--------");
    println!("Matches:");
    println!("--------");
    for v in vec {
        println!("{}", v);
    }
    println!("--------");

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
