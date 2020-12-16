#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, Read, Write};
use std::iter::Iterator;
use std::string::String;

use regex::Regex;

type Range = (u64, u64);
type Ticket = Vec<u64>;
type Rules = HashMap<String, Vec<Range>>;

fn valid_in_range(val: &u64, range: &Range) -> bool {
    range.0 <= *val && *val <= range.1
}

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

fn check_validity(tickets: &[Ticket], rules: &Rules) -> Vec<usize> {
    let mut valids = Vec::new();

    let mut i = 0;
    for ticket in tickets {
        let mut all_valid = true;
        for val in ticket {
            let mut valid = false;
            'val: for (_, rule) in rules {
                for range in rule {
                    if valid_in_range(val, range) {
                        valid = true;
                        break 'val;
                    }
                }
            }
            all_valid &= valid;
        }
        if all_valid {
            valids.push(i);
        }
        i += 1;
    }

    valids
}

fn assemble_fields(tickets: &[Ticket], rules: &Rules) -> Vec<Vec<Vec<String>>> {
    let mut fields_for_ticket = Vec::new();

    for ticket in tickets {
        let mut fields = Vec::new();
        for val in ticket {
            let mut fitting_fields = Vec::new();
            for (field, ranges) in rules {
                for range in ranges {
                    if valid_in_range(val, range) {
                        fitting_fields.push(field.clone());
                    }
                }
            }
            fields.push(fitting_fields.clone());
        }
        fields_for_ticket.push(fields.clone());
    }

    fields_for_ticket
}

// param: ticket: fields: possible_fields
fn decode_field_order(fields_per_ticket: &Vec<Vec<Vec<String>>>) -> Vec<String> {
    let ticket_count = fields_per_ticket.len();
    let field_count = fields_per_ticket[0].len();
    let mut handled: HashSet<String> = HashSet::new();
    let mut field_i = 0;

    let mut ordered = vec![String::new(); field_count];

    while handled.len() < field_count {
        let mut candidates: Vec<&String> = fields_per_ticket[0][field_i]
            .iter()
            .filter(|x| !handled.contains(*x))
            .collect();

        for ticket_i in 1..ticket_count {
            let fields = fields_per_ticket[ticket_i][field_i].clone();
            candidates = candidates
                .iter()
                .filter(|x| fields.contains(*x))
                .map(|x| *x)
                .collect();
        }

        if candidates.len() == 1 {
            handled.insert(candidates[0].clone());
            ordered[field_i] = candidates[0].clone();
        }
        if field_i < field_count - 1 {
            field_i += 1;
        } else {
            field_i = 0;
        }
    }

    ordered
}

fn get_departure_vals(ticket: &Ticket, fields: &[String]) -> u64 {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^departure").unwrap();
    }

    let mut vals = Vec::new();

    for i in 0..fields.len() {
        if RE.is_match(&fields[i]) {
            vals.push(ticket[i]);
        }
    }

    vals.iter().fold(1, |acc, x| acc * x)
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let (rules, my_ticket, tickets) = parse_input(&mut lines);

    let valid_i = check_validity(&tickets, &rules);
    let valids: Vec<Ticket> = valid_i.iter().map(|i| tickets[*i].clone()).collect();

    let fields_per_ticket = assemble_fields(&valids, &rules);

    // for ticket_fields in &fields_per_ticket {
    //     for fields in ticket_fields {
    //         for field in fields {
    //             write!(io::stdout(), "{},", field)?;
    //         }
    //         writeln!(io::stdout(), "")?;
    //     }
    //     writeln!(io::stdout(), "----")?;
    // }

    let ordered = decode_field_order(&fields_per_ticket);

    for order in &ordered {
        println!("{}", order);
    }

    let mult = get_departure_vals(&my_ticket, &ordered);

    writeln!(io::stdout(), "departure fields multiplied: {}", mult)?;

    Ok(())
}
