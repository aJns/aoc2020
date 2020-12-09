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

impl State {
    fn new() -> State {
        State {point: 0, acc: 0}
    }
}

fn add_usize_int(u: usize, i: i32) -> usize {
    let ui = u as i32;
    let sum = ui + i;

    if sum < 0 {
        return 0;
    }

    sum as usize
}

fn get_cmd_type(cmd: &String) -> String {
    let command: Vec<&str> = cmd.split_whitespace().collect();
    command[0].to_string()
}

fn get_arg(cmd: &String) -> i32 {
    let command: Vec<&str> = cmd.split_whitespace().collect();
    let arg: i32 = match command[1].parse() {
        Ok(x)   => x,
        _       => panic!("asd")
    };

    arg
}


fn exec_cmd(state: &State, cmd: String) -> State {
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

struct ExitStatus {
    terminated: bool,
    state: State,
    furthest: usize
}

fn exec_tape(tape: &Vec<String>, start_state: State) -> ExitStatus {
    let mut executed_cmds: HashSet<usize> = HashSet::new();

    let mut state = start_state;

    let mut counter = 0;

    let mut furthest = 0;

    let mut terminated = false;

    loop {
        let index = state.point;

        if index == tape.len() {
            terminated = true;
            break;
        }
        let cmd = tape[index].clone();

        //print!("{:<10}", cmd);

        if executed_cmds.contains(&index) {
            println!("");
            break;
        }

        state = exec_cmd(&state, cmd);
        executed_cmds.insert(index);

        furthest = match index > furthest {
            true  => index,
            false => furthest
        };

        //println!("| {}/{}", counter, index);
        counter += 1;
    }

    ExitStatus {terminated, state, furthest}
}

fn replace_cmd(old: &str, new_type: &str) -> String {
    let arg = match old.split_whitespace().last() {
        Some(a) => a,
        None    => panic!("aaaa")
    };

    let mut ret = String::from(new_type);
    ret.push_str(" ");
    ret.push_str(arg);

    println!("{} -> {}", old, ret);

    ret
}


fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let orig_tape: Vec<String> = input.lines().map(|x| x.trim().to_string()).collect();

    let mut tape: Vec<String> = orig_tape.clone();
    let start_state = State::new();

    let mut tried_jmp_change = false;
    let mut tried_nop_change = false;

    let mut changed_cmds: HashSet<usize> = HashSet::new();

    loop {
        let status = exec_tape(&tape, start_state);

        let raport = match status.terminated {
            true  => "Ran successfully!",
            false => "Infinite loop."
        };

        writeln!(io::stdout(), "{} acc val: {}, furthest index: {}", raport, status.state.acc, status.furthest)?;

        if status.terminated {
            break;
        } else {
            tape = orig_tape.clone();
            let furthest = status.furthest;
            let mut index = furthest;
            if !tried_jmp_change {
                println!("jmp change");

                while index > 0 {
                    let cmd = orig_tape[index].clone();
                    if get_cmd_type(&cmd) == "jmp" {
                        //let arg = get_arg(&cmd);
                        //if add_usize_int(index, arg) > furthest {
                        if !changed_cmds.contains(&index) {
                            let changed = &replace_cmd(&cmd, "nop");
                            tape[index] = changed.clone();
                            changed_cmds.insert(index);
                            break;
                        }
                    }
                    index = add_usize_int(index, -1);
                }
                if index == 0 {
                    tried_jmp_change = true;
                }
            } else if !tried_nop_change {
                println!("nop change");

                while index > 0 {
                    let cmd = orig_tape[index].clone();
                    if get_cmd_type(&cmd) == "nop" {
                        //let arg = get_arg(&cmd);
                        //if add_usize_int(index, arg) > furthest {
                        if !changed_cmds.contains(&index) {
                            let changed = &replace_cmd(&cmd, "jmp");
                            tape[index] = changed.clone();
                            changed_cmds.insert(index);
                            break;
                        }
                    }
                    index = add_usize_int(index, -1);
                }
                if index == 0 {
                    tried_nop_change = true;
                }
            } else {
                panic!("asd");
            }
        }
    }

    Ok(())
}

