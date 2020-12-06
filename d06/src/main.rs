use std::collections::HashSet;
use std::io::{self, Read, Write};
use std::string::String;

fn parse_into_groups(lines: &Vec<String>) -> Vec<String> {
    let mut groups: Vec<String> = vec![String::new()];

    for l in lines {
        match l.trim() {
            "" => groups.push(String::new()),
            _  => if let Some(last) = groups.last_mut() {
                // last.push(' ');
                last.push_str(l);
            }
        };
    }

    groups
}

type Set = HashSet<char>;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let lines: Vec<String> = input.lines().map(|x| x.to_string()).collect();

    let groups = parse_into_groups(&lines);

    let mut sets: Vec<Set> = Vec::new();

    for g in groups {
        let mut set: Set = HashSet::new();
        for c in g.chars() {
            set.insert(c);
        }
        sets.push(set);
    }

    let mut counter = 0;

    for s in sets {
        counter += s.len();
    }

    writeln!(io::stdout(), "Sum of yes questions: {}", counter)?;

    Ok(())
}

