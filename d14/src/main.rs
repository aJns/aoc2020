#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::io::{self, Read, Write};
use std::string::String;

use std::collections::HashMap;

use regex::Regex;

type Binary = Vec<char>;

fn to_binary(str_rep: &str) -> Binary {
    str_rep.chars().rev().collect()
}

fn to_dec(bin: &Binary) -> u64 {
    let mut acc = 0;
    let mut pos = 0;
    for b in bin {
        acc += b.to_string().parse::<u64>().unwrap() * (2 as u64).pow(pos);
        pos += 1;
    }
    acc
}

#[derive(PartialEq)]
enum Op {
    Mask,
    Save,
}

struct MemCmd {
    op: Op,
    i: usize,
    bin: Binary,
}

fn dec2bin(dec: u64) -> Binary {
    let str_rep = format!("{:036b}", dec);
    to_binary(&str_rep)
}

fn parse_cmd(line: &str) -> MemCmd {
    lazy_static! {
        static ref OP: Regex = Regex::new(r"([a-z]*).* =").unwrap();
        static ref IN: Regex = Regex::new(r"\[([0-9]*)\]").unwrap();
        static ref VAL: Regex = Regex::new(r"= (.*)").unwrap();
    }

    let op = &OP.captures(line).unwrap()[1];

    if op == "mask" {
        let val = to_binary(&VAL.captures(line).unwrap()[1]);
        MemCmd {
            op: Op::Mask,
            i: 0,
            bin: val,
        }
    } else {
        let index: usize = IN.captures(line).unwrap()[1].parse().unwrap();
        let val = dec2bin(VAL.captures(line).unwrap()[1].parse().unwrap());
        MemCmd {
            op: Op::Save,
            i: index,
            bin: val,
        }
    }
}

fn apply_mask(val: &Binary, mask: &Binary) -> Vec<Binary> {
    let mut ret: Vec<Binary> = Vec::new();
    ret.push(Vec::new());

    for i in 0..mask.len() {
        let m = mask[i];
        if m == 'X' {
            let mut ret_cp = ret.clone();

            for r in &mut ret {
                r.push('0');
            }
            for r in &mut ret_cp {
                r.push('1');
            }

            ret.append(&mut ret_cp);
        } else {
            for r in &mut ret {
                match m {
                    '0' => r.push(val[i]),
                    '1' => r.push('1'),
                    _ => panic!("wrong val"),
                }
            }
        }
    }
    ret
}

fn run_prog(mem: &mut HashMap<usize, u64>, cmds: &[MemCmd]) {
    let mut mask: Binary = Vec::new();
    for c in cmds {
        match c.op {
            Op::Mask => mask = c.bin.clone(),
            Op::Save => {
                for mskd in apply_mask(&dec2bin(c.i as u64), &mask) {
                    let index = to_dec(&mskd) as usize;
                    let dec = to_dec(&c.bin);
                    mem.insert(index, dec);
                }
            }
        }
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let lines: Vec<&str> = input.lines().collect();

    let cmds: Vec<MemCmd> = lines.into_iter().map(|x| parse_cmd(x)).collect();

    for c in &cmds {
        if c.op == Op::Save {
            print!("{:<10} = ", to_dec(&c.bin));
        } else {
            print!("{:<10} = ", "bitmask");
        }
        for b in &c.bin {
            print!("{}", b);
        }
        println!("");
    }

    let mut mem = HashMap::new();

    run_prog(&mut mem, &cmds);

    let mem_sum = mem.iter().fold(0, |acc, x| acc + x.1);

    writeln!(io::stdout(), "Sum of mem values: {}", mem_sum)?;

    Ok(())
}
