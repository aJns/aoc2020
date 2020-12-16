use std::collections::HashMap;
use std::env::args;
use std::io::{self, Read, Write};
use std::iter::Iterator;
use std::string::String;

type Range = (u64, u64);
type Ticket = Vec<u64>;
type Rules = HashMap<String, Vec<Range>>;

fn parse_rule(line: &str) -> (String, Vec<Range>) {
    let colon: Vec<&str> = line.split(":").collect();
    let rule_str = colon[0].to_string();

    let mut ranges = Vec::new();

    for range in colon[1].split("or") {
        let r: Vec<u64> = range
            .trim()
            .split("-")
            .map(|x| x.parse().unwrap())
            .collect();
        let ra = (r[0], r[1]);

        ranges.push(ra);
    }

    (rule_str, ranges)
}

fn parse_input(lines: &mut dyn Iterator<Item = &str>) -> (Rules, Ticket, Vec<Ticket>) {
    let mut rules = HashMap::new();
    while let Some(rule) = lines.next() {
        if rule.trim() == "" {
            break;
        }

        let (key, val) = parse_rule(rule);
        rules.insert(key, val);
    }

    while let Some(ticket) = lines.next() {
        if ticket.trim() == "your ticket:" {
            break;
        }
    }

    let my_ticket = lines
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    while let Some(ticket) = lines.next() {
        if ticket.trim() == "nearby tickets:" {
            break;
        }
    }

    let other_tickets = lines
        .map(|x| x.split(",").map(|y| y.parse().unwrap()).collect())
        .collect();

    (rules, my_ticket, other_tickets)
}

fn check_validity(tickets: &[Ticket], rules: &Rules) -> Vec<u64> {
    let mut invalids = Vec::new();

    for ticket in tickets {
        for val in ticket {
            let mut invalid = true;
            'rule: for (_, rule) in rules {
                for range in rule {
                    if range.0 <= *val && *val <= range.1 {
                        invalid = false;
                        break 'rule;
                    }
                }
            }
            if invalid {
                invalids.push(*val);
            }
        }
    }

    invalids
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let (rules, my_ticket, tickets) = parse_input(&mut lines);

    let invalids = check_validity(&tickets, &rules);

    let sum = invalids.iter().fold(0, |acc, x| acc + x);
    writeln!(io::stdout(), "sum of invalids: {}", sum)?;

    Ok(())
}
