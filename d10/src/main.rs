use std::collections::HashSet;
use std::io::{self, Read, Write};
use std::string::String;


fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let input: Vec<u64> = input.lines().map(|x| x.parse().unwrap()).collect();

    let mut sorted = input.clone();

    sorted.push(0);
    sorted.sort();
    sorted.push(sorted.last().unwrap()+3);

    let mut diff1 = 0;
    let mut diff3 = 0;

    for i in 1..sorted.len() {
        let diff = sorted[i] - sorted[i-1];

        match diff {
            1 => diff1 += 1,
            3 => diff3 += 1,
            _ => (),
        }
    }

    let mult = diff1*diff3;

    writeln!(io::stdout(), "diffs, 1: {}, 3: {}. mult: {}", diff1, diff3, mult)?;

    Ok(())
}

