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

fn count_bags(rules: &RuleMap, bag: &str) -> u32 {
    if let Some(map) = rules.get(bag) {
        let mut count = 1;

        for (k, v) in map {
            let mult: u32 = match v {
                0 => 1,
                _ => *v
            };
            count += mult*count_bags(rules, k);
        }
        println!("\"{}\" has {} bags inside", bag, count);
        count
    } else {
        1
    }
}


fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    lazy_static! {
        static ref GOLD: Regex = Regex::new(r"shiny gold").unwrap();
    }

    let temp = input.split(".").map(|x| parse_rule(x.trim()));
    let mut rules: RuleMap = HashMap::new();

    for t in temp {
        for (key,val) in t {
            rules.insert(key, val);
        }
    }


    writeln!(io::stdout(), "{} bags required", count_bags(&rules, "shiny gold")-1)?;

    Ok(())
}

