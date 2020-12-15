use std::io::{self, Read, Write};
use std::string::String;

use std::collections::HashMap;

fn calc_turn_diff(turns: &[u32]) -> u32 {
    let last_i = turns.len() - 1;
    let second_i = last_i - 1;

    let last = turns[last_i];
    let second = turns[second_i];

    let diff = last - second;

    // for t in turns {
    //     println!("\tSaid on turns: {}", t);
    // }

    // println!("\tLast: {}, Sec2last: {}, Diff: {}", last, second, diff);

    diff
}

fn play_game(start: &Vec<u32>) -> u32 {
    let mut turn = 0;
    let mut prev = 0;

    let mut turn_spoken = HashMap::new();

    for s in start {
        turn += 1;
        prev = *s;

        // println!("Turn: {}, Starting number: {}", turn, prev);
        turn_spoken.insert(prev, vec![turn]);
    }

    while turn < 30000000 {
        turn += 1;
        let turns = &turn_spoken.get(&prev).unwrap();

        if turns.len() == 1 {
            prev = 0;
        } else {
            prev = calc_turn_diff(turns);
        }
        if turn % 300_000 == 0 {
            println!("Turn: {}, number: {}", turn, prev);
        }

        let mut new_turns;
        if turn_spoken.contains_key(&prev) {
            new_turns = turn_spoken.get(&prev).unwrap().clone();
        } else {
            new_turns = Vec::new();
        }
        new_turns.push(turn);
        turn_spoken.insert(prev, new_turns);
    }

    prev
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let lines: Vec<&str> = input.lines().collect();

    for line in lines {
        let nums: Vec<u32> = line.split(",").map(|x| x.parse().unwrap()).collect();
        let result = play_game(&nums);
        write!(io::stdout(), "input: {:<17}", line)?;
        writeln!(io::stdout(), "30000000th number: {}", result)?;
    }

    Ok(())
}
