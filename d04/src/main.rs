#[macro_use] extern crate lazy_static;
extern crate regex;

use std::io::{self, Read, Write};
use std::iter::Iterator;
use std::str::{Lines, FromStr};
use std::string::ParseError;
use std::collections::HashMap;

use regex::{Regex, RegexSet};


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

struct Data {
    field: String,
    value: String,
}

impl FromStr for Data {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i: Vec<&str> = s.trim().split(':').collect();

        let f = i[0];
        let v = i[1];

        Ok(Data {field: f.to_string(), value: v.to_string()})
    }
}

fn parse_fields(pass: &String) -> Vec<Data> {
    let mut fields: Vec<Data> = Vec::new();
    let w_vals = pass.split_whitespace();

    for v in w_vals {
        if let Ok(f) = v.parse() {
            fields.push(f);
        }
    }

    return fields;
}

fn has_required_fields(fields: &Vec<Data>) -> bool {
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
            if r == f.field {
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

fn birth_valid(data: Option<&String>) -> bool {
    if let Some(d) = data {
        if let Ok(year) = d.parse::<u32>() {
            return year >= 1920 && year <= 2002;
        }
    }

    return false;
}

fn issue_valid(data: Option<&String>) -> bool {
    if let Some(d) = data {
        if let Ok(year) = d.parse::<u32>() {
            return year >= 2010 && year <= 2020;
        }
    }

    return false;
}

fn exp_valid(data: Option<&String>) -> bool {
    if let Some(d) = data {
        if let Ok(year) = d.parse::<u32>() {
            return year >= 2020 && year <= 2030;
        }
    }

    return false;
}

fn height_valid(data: Option<&String>) -> bool {
    lazy_static! {
        static ref CM: Regex = Regex::new(r"^(.*?)cm$").unwrap();
        static ref INCH: Regex = Regex::new(r"^(.*?)in$").unwrap();
    }
    if let Some(d) = data {
        if let Some(cap) = CM.captures(d) {
            if let Ok(hgt) = cap.get(1).map_or("", |m| m.as_str()).parse::<u32>() {
                return hgt >= 150 && hgt <= 193;
            }
        }
        if let Some(cap) = INCH.captures(d) {
            if let Ok(hgt) = cap.get(1).map_or("", |m| m.as_str()).parse::<u32>() {
                return hgt >= 59 && hgt <= 76;
            }
        }
    }

    return false;
}

fn hair_valid(data: Option<&String>) -> bool {
    lazy_static! {
        static ref ECL: Regex = Regex::new(r"^#[[0-9][a-f]]{6}?$").unwrap();
    }
    if let Some(d) = data {
        return ECL.is_match(d);
    }

    return false;
}

fn eye_valid(data: Option<&String>) -> bool {
    lazy_static! {
        static ref HCL: RegexSet = RegexSet::new(&[
                                                 r"^amb$",
                                                 r"^blu$",
                                                 r"^brn$",
                                                 r"^gry$",
                                                 r"^grn$",
                                                 r"^hzl$",
                                                 r"^oth$",
        ]).unwrap();
    }
    if let Some(d) = data {
        let mut counter = 0;
        for _ in HCL.matches(d) {
            counter += 1;
        }

        return counter == 1;
    }

    return false;
}

fn pid_valid(data: Option<&String>) -> bool {
    lazy_static! {
        static ref PID: Regex = Regex::new(r"^[0-9]{9}?$").unwrap();
    }
    if let Some(d) = data {
        return PID.is_match(d);
    }

    return false;
}

fn are_fields_valid(fields: &Vec<Data>) -> bool {
    let mut dict = HashMap::new();

    for f in fields {
        dict.insert(f.field.clone(), f.value.clone());
    }

    let mut valid = true;

    valid = valid && birth_valid(dict.get("byr"));
    valid = valid && issue_valid(dict.get("iyr"));
    valid = valid && exp_valid(dict.get("eyr"));
    valid = valid && height_valid(dict.get("hgt"));
    valid = valid && hair_valid(dict.get("hcl"));
    valid = valid && eye_valid(dict.get("ecl"));
    valid = valid && pid_valid(dict.get("pid"));

    return valid;
}

fn print_fields(fields: &Vec<Data>) {
    for f in fields {
        println!("\t{}: {}", f.field, f.value);
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let passports = parse_passports(input.lines());
    let pass_fields: Vec<Vec<Data>> = passports.into_iter().map(|x| parse_fields(&x)).collect();

    let mut counter: i32 = 0;

    for fields in &pass_fields {
        let has_fields = has_required_fields(fields);
        let fields_valid = are_fields_valid(fields);

        if has_fields && fields_valid {
            counter += 1;
            println!("valid");
            print_fields(fields);
        } else {
            // println!("invalid");
            // print_fields(fields);
        }
    }

    writeln!(io::stdout(), "{} valid passports", counter)?;

    Ok(())
}

