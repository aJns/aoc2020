//#[macro_use] extern crate lazy_static;
//extern crate regex;

use std::collections::HashSet;
use std::io::{self, Read, Write};
use std::string::String;
//use regex::{Regex, RegexSet};

#[derive(Copy, Clone)]
struct State {
    point: usize,
    acc: i32,
}

fn add_usize_int(u: usize, i: i32) -> usize {
    let ui = u as i32;
    let sum = ui + i;

    if sum < 0 {
        return 0;
    }

    sum as usize
}


fn exec_cmd(state: &State, cmd: &str) -> State {
    let command: Vec<&str> = cmd.split_whitespace().collect();
    let cmd_type = command[0];
    let arg: i32 = match command[1].parse() {
        Ok(x)   => x,
        _       => panic!("asd")
    };

    let (point, acc) = match cmd_type {
        "nop" => (state.point+1, state.acc),
        "acc" => (state.point+1, state.acc + arg),
        "jmp" => (add_usize_int(state.point, arg), state.acc),
        _     => panic!("asd")
    };

    State { point, acc }
}



fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let tape: Vec<&str> = input.lines().map(|x| x.trim()).collect();

    let mut executed_cmds: HashSet<usize> = HashSet::new();
    let mut prev_cmd = "";

    let mut state = State {point: 0, acc: 0};

    let mut counter = 1;

    loop {
        let index = state.point;
        let cmd = tape[index];

        print!("{}", cmd);

        if executed_cmds.contains(&index) {
            println!("");
            break;
        }

        state = exec_cmd(&state, cmd);
        executed_cmds.insert(index);

        println!("  | {}", counter);
        counter += 1;

        prev_cmd = cmd;
    }

    writeln!(io::stdout(), "last command: {}, acc val: {}", prev_cmd, state.acc)?;

    Ok(())
}

