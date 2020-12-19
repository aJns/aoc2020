#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections::HashMap;
use std::io::{self, Read, Write};

use regex::Regex;

type Rule = Vec<String>;

fn parse_rule0(rules: &Vec<String>) -> Rule {
    parse_rule(rules, 0)
}

fn parse_rule(rules: &Vec<String>, i: usize) -> Rule {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[a-b]").unwrap();
    }

    let rule = &rules[i];

    if RE.is_match(rule) {
        let c: &str = rule.split("\"").collect::<Vec<&str>>()[1];
        return vec![c.to_string()];
    }

    let mut ret: Rule = Vec::new();
    for or in rule.split("|") {
        let mut indices = Vec::new();

        for s in or.split(" ") {
            if let Ok(x) = s.parse::<usize>() {
                indices.push(x);
            }
        }

        let mut strings = vec!["".to_string()];

        for parsed_rules in indices.iter().map(|i| parse_rule(rules, *i)) {
            let mut new_str = Vec::new();
            for pr in &parsed_rules {
                for s in &strings {
                    let mut new = s.clone().to_string();
                    new.push_str(&pr.clone());
                    new_str.push(new);
                }
            }
            strings = new_str;
        }
        ret.append(&mut strings);
    }

    ret
}

fn is_match(rule: &Rule, msg: &str) -> bool {
    for r in rule {
        if r.trim() == msg.trim() {
            return true;
        }
    }

    false
}

fn calc_matches(rule: &Rule, msgs: &[&str]) -> u64 {
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

fn sort_rules(unsorted: &[&str]) -> Vec<String> {
    println!("Sorting rules...");
    let mut map: HashMap<String, String> = HashMap::new();
    let mut keys: Vec<usize> = Vec::new();
    for us in unsorted {
        let split: Vec<&str> = us.split(":").collect();

        map.insert(split[0].to_string(), split[1].to_string());
        keys.push(split[0].parse().unwrap());
    }
    keys.sort();

    let mut ret = Vec::new();
    for k in keys {
        let val = map.get(&k.to_string()).unwrap();
        ret.push(val.clone());
    }
    println!("Sorted");

    ret
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

    let rules = sort_rules(&unsorted);

    let rule0 = parse_rule0(&rules);

    let messages: Vec<&str> = lines.map(|x| x).collect();

    let matches = calc_matches(&rule0, &messages);

    writeln!(io::stdout(), "{} matches", matches)?;

    Ok(())
}
