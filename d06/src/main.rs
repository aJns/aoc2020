use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::string::String;

#[derive(Clone)]
struct Group {
    answers: String,
    count: u32,
}

impl Group {
    fn new() -> Group {
        Group { answers: String::new(), count: 0 }
    }
}

fn parse_into_groups(lines: &Vec<String>) -> Vec<Group> {
    let mut groups: Vec<Group> = vec![Group::new()];

    for l in lines {
        match l.trim() {
            "" => groups.push(Group::new()),
            _  => if let Some(last) = groups.last_mut() {
                // last.push(' ');
                last.answers.push_str(l);
                last.count += 1;
            }
        };
    }

    groups
}

type Dict = HashMap<char, u32>;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let lines: Vec<String> = input.lines().map(|x| x.to_string()).collect();

    let groups = parse_into_groups(&lines);

    let mut counter = 0;

    let mut i = 0;
    for g in groups {
        println!("group {}", i);
        println!("\t answers: {}", g.answers);
        println!("\t count: {}", g.count);
        i += 1;

        let answer: String = g.answers;

        let mut d: Dict = HashMap::new();
        for c in answer.chars() {
            *d.entry(c).or_insert(0) += 1;
        }

        let mut group_counter = 0;
        for (key, count) in &d {
            println!("\t{}: {}", key, count);


            if *count == g.count {
                group_counter += 1;
            }
        }
        println!("Group counter: {}", group_counter);
        counter += group_counter;
    }


    writeln!(io::stdout(), "Sum of yes questions: {}", counter)?;

    Ok(())
}

