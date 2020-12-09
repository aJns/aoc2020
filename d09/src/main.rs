//#[macro_use] extern crate lazy_static;
//extern crate regex;

//use regex::{Regex, RegexSet};

use std::collections::HashSet;
use std::io::{self, Read, Write};
use std::string::String;

use std::env::args;


fn has_sum(preamble: &[i64], target: i64) -> bool {
    for i in preamble {
        for j in preamble {
            if i != j && i+j == target {
                return true
            }
        }
    }

    false
}

fn find_invalid(input: &[i64], preamble_len: usize) -> i64 {
    for index in preamble_len..input.len()-1 {
        let preamble = &input[index-preamble_len..index];
        let target = input[index];

        if !has_sum(preamble, target) {
            return target
        }
    }
    panic!("Fucked up: no invalid number found");
}

fn calc_contiguous(input: &[i64], invalid: i64) -> Option<(usize, usize)> {
    for i in 0..input.len() {
        for j in i+1..input.len() {
            let cont = &input[i..j+1];
            let sum = cont.iter().fold(0, |acc, x| acc + x);

            if sum == invalid {
                return Some((i, j))
            }
        }
    }
    None
}

fn find_contiguous(input: &[i64], invalid: i64) -> Option<(usize, usize)> {
    if input.len() > 2 {
        let middle = input.len()/2;
        if let Some(ret) = find_contiguous(&input[..middle], invalid) {
            return Some(ret)
        }
        if let Some(ret) = find_contiguous(&input[middle..], invalid) {
            let start = middle + ret.0;
            let end = middle + ret.1;
            return Some((start, end))
        }
    } else {
        return calc_contiguous(input, invalid)
    }
    None
}

fn get_smallest_largest(input: &[i64]) -> (i64, i64) {
    let mut vec = Vec::from(input);
    vec.sort();
    (*vec.first().unwrap(), *vec.last().unwrap())
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let input: Vec<i64> = input.lines().map(|x| x.parse().unwrap()).collect();

    let str_args = args().collect::<Vec<String>>();

    let preamble_len = match str_args.len() {
        2 => str_args[1].parse().unwrap(),
        _ => 25
    };

    println!("Preamble length: {}", preamble_len);

    let invalid = find_invalid(&input, preamble_len);

    writeln!(io::stdout(), "{} is not a preamble sum", invalid)?;

    let (start, end) = calc_contiguous(input.as_slice(), invalid).unwrap();

    let (s, l) = get_smallest_largest(&input[start..end+1]);
    let sum = s+l;

    writeln!(io::stdout(), "smallest: {}, largest: {}, sum: {}", s, l, sum)?;

    Ok(())
}

