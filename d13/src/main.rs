use std::io::{self, Read, Write};
use std::string::String;

fn parse_id(id: &str) -> u128 {
    match id.parse::<u128>() {
        Ok(x) => x,
        Err(_) => 0,
    }
}

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn lcm(a: u128, b: u128) -> u128 {
    (a * b) / gcd(a, b)
}

fn find_sequence(prev_t: u128, prev_lcm: u128, remain: &[u128]) -> u128 {
    println!(
        "prev_t: {}, prev_lcm: {}, remaining ids: {}",
        prev_t,
        prev_lcm,
        remain.len()
    );
    if remain.is_empty() {
        return prev_t;
    }
    let curr = remain[0];
    if curr == 0 {
        return find_sequence(prev_t + 1, prev_lcm, &remain[1..]);
    }

    let mut n = 0;
    let next_t;
    loop {
        let test = prev_t + n * prev_lcm + 1;
        if test % curr == 0 {
            next_t = test;
            break;
        }
        n += 1;
    }
    let next_lcm = lcm(prev_lcm, curr);
    return find_sequence(next_t, next_lcm, &remain[1..]);
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let lines: Vec<&str> = input.lines().collect();

    let bus_ids: Vec<u128> = lines[1].split(',').map(|x| parse_id(x)).collect();

    let minus = (bus_ids.len() as u128) - 1;

    let earliest = find_sequence(0, 1, &bus_ids) - minus;

    writeln!(io::stdout(), "earliest sequence start: {}", earliest)?;

    Ok(())
}
