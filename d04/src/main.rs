extern crate regex;

use std::io::{self, Read, Write};
use std::iter::Iterator;
use std::str::Lines;

use regex::Regex;


fn parse_passports(lines: Lines) -> Vec<String> {
    let mut passports: Vec<String> = vec![String::new()];

    for l in lines {
        match l.trim() {
            "" => passports.push(String::new()),
            _  => if let Some(last) = passports.last_mut() {
                last.push(' ');
                last.push_str(l);
            }
        };
    }

    return passports;
}

fn parse_fields(pass: &String) -> Vec<String> {
    let mut fields: Vec<String> = Vec::new();
    let w_vals = pass.split_whitespace();

    for v in w_vals {
        if let Some(f) = v.split(':').next() {
            fields.push(f.to_string());
        }
    }

    return fields;
}

fn has_required_fields(fields: &Vec<String>) -> bool {
    let required: Vec<&str> = vec![
        "byr",
        "iyr",
        "eyr",
        "hgt",
        "hcl",
        "ecl",
        "pid"
            // ,"cid"
    ];

    for r in required {
        let mut found = false;
        for f in fields {
            if r == f {
                found = true;
                break;
            }
        }
        if !found {
            return false;
        }
    }

    return true;
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let passports = parse_passports(input.lines());
    let pass_fields: Vec<Vec<String>> = passports.into_iter().map(|x| parse_fields(&x)).collect();

    let mut counter: i32 = 0;

    for fields in &pass_fields {
        if has_required_fields(fields) {
            counter += 1;
        }
    }

    writeln!(io::stdout(), "{} valid passports", counter)?;

    Ok(())
}

