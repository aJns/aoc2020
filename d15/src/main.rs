use std::env::args;
use std::io::{self, Read, Write};
use std::string::String;

fn play_game(start: &Vec<usize>, end: usize) -> usize {
    let mut turn = 0;
    let mut prev = 0;

    let mut turn_spoken = vec![0; 30e6 as usize];

    for s in start {
        turn += 1;
        prev = *s;

        // println!("Turn: {}, Starting number: {}", turn, prev);
        turn_spoken[prev] = turn;
    }

    let mut pp_turn = 0;
    let mut pp_val = 0;

    while turn < end {
        let prev_turn = turn_spoken[prev];

        // print!("Last: {}, turn: {}, prev_turn: {}\t", prev, turn, prev_turn);

        if prev_turn == 0 {
            prev = 0;
        } else {
            prev = turn - prev_turn;
        }

        if pp_turn != 0 {
            turn_spoken[pp_val] = pp_turn;
        }

        turn += 1;
        pp_turn = turn;
        pp_val = prev;

        // println!("Turn: {}, number: {}", turn, prev);
    }

    prev
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let lines: Vec<&str> = input.lines().collect();

    let mut args = args();
    args.next();
    let end = match args.next() {
        Some(x) => x.parse().unwrap(),
        None => 10,
    };

    for line in lines {
        let nums: Vec<usize> = line.split(",").map(|x| x.parse().unwrap()).collect();
        let result = play_game(&nums, end);
        write!(io::stdout(), "input: {:<17}", line)?;
        writeln!(io::stdout(), "{}th number: {}", end, result)?;
    }

    Ok(())
}
