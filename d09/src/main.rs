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

    for index in preamble_len..input.len()-1 {
        let preamble = &input[index-preamble_len..index];
        let target = input[index];

//        for p in preamble {
//            println!("preamble: {}", p);
//        }

        if !has_sum(preamble, target) {
            writeln!(io::stdout(), "{} is not a preamble sum", target)?;
            break;
        }
    }


    Ok(())
}

