#[macro_use] extern crate lazy_static;
extern crate regex;

use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, Read, Write};
use std::string::String;
use regex::{Regex, RegexSet};

type BagCountMap = HashMap<String, u32>;
type RuleMap = HashMap<String, BagCountMap>;

fn parse_rule(rule: &str) -> RuleMap {
    lazy_static! {
        static ref BAG: Regex = Regex::new(r" *(.*) bags?").unwrap();
    }

    let contain_split: Vec<&str> = rule.split(" contain ").collect();

    let mut map = HashMap::new();
    let mut inner: BagCountMap = HashMap::new();

    if contain_split.len() == 2 {
        for i in contain_split[1].split(", ") {
            let cs: Vec<char> = i.chars().collect();
            if let Ok(count) = cs[0].to_string().parse::<u32>() {
                let bag = i[1..].trim().to_string();
                if let Some(cap) = BAG.captures(&bag) {
                    let b = cap.get(1).map_or("", |m| m.as_str());
                    inner.insert(b.to_string(), count);
                }
            }
        }
        if let Some(cap) = BAG.captures(contain_split[0]) {
            let b = cap.get(1).map_or("", |m| m.as_str());
            map.insert(b.to_string(), inner);
        }
    }

    map
}

fn print_rule_map(map: &RuleMap) {
    for (key, val) in map {
        println!("{}", key);
        for (k, v) in val {
            println!("\t{}:{}", k, v);
        }
    }
}


fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    lazy_static! {
        static ref GOLD: Regex = Regex::new(r"shiny gold").unwrap();
    }

    let mut color_count = 0;

    let temp = input.split(".").map(|x| parse_rule(x.trim()));
    let mut rules: RuleMap = HashMap::new();

    for t in temp {
        for (key,val) in t {
            rules.insert(key, val);
        }
    }

    // remove empty bags
    let mut remove: HashSet<String> = HashSet::new();
    for (k, v) in &rules {
        if v.is_empty() {
            remove.insert(k.to_string());
        }
    }

    for r in &remove {
        rules.remove(r);
    }

    remove.clear();


    // Start with shiny gold in containers
    let mut containers: HashSet<String> = HashSet::new();
    containers.insert("shiny gold".to_string());

    while !rules.is_empty() {
        // remove containers; We dont wanna check those
        for c in &containers {
            rules.remove(c);
        }

        let mut ins = false;
        // add containers to set
        for (key, val) in &rules {
            for (k, _) in val {
                if containers.contains(k) {
                    containers.insert(key.clone());
                    ins = true;
                }
            }
        }

        if !ins {
            break;
        }
    }

    color_count = containers.len() -1;


    writeln!(io::stdout(), "{} colors can contain a shiny gold bag", color_count)?;

    Ok(())
}

