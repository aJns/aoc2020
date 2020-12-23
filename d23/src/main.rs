use std::io::{self, Read, Write};

type Cup = u8;

fn do_move(mut cups: Vec<Cup>, index: usize) -> (Vec<Cup>, usize) {
    for c in &cups {
        print!("{} ", c);
    }
    println!("");
    println!("{}", cups[index]);

    let current = cups[index];

    let mut three = Vec::new();

    for _ in 0..3 {
        let mut i = index + 1;

        if i >= cups.len() {
            i = 0;
        }

        let rem = cups.remove(i);
        three.push(rem);
    }
    for t in &three {
        print!("{} ", t);
    }
    println!("");

    let mut i = index;

    while i >= cups.len() {
        i -= 1;
    }

    let mut label = cups[i] - 1;

    while !cups.contains(&label) {
        if label == 0 {
            label = 9;
            continue;
        }
        label -= 1;
    }

    let mut dest = 0;
    for cup in &cups {
        if cup == &label {
            break;
        }
        dest += 1;
    }

    println!("{}", cups[dest]);
    println!("");

    for cup in three {
        dest += 1;
        cups.insert(dest, cup);
    }

    let mut next = 1;
    for cup in &cups {
        if cup == &current {
            break;
        }
        next += 1;
    }

    if next < cups.len() {
        (cups, next)
    } else if next == cups.len() {
        (cups, 0)
    } else {
        panic!()
    }
}

fn get_cup_order(cups: &Vec<Cup>) -> Vec<Cup> {
    let mut order = Vec::new();

    let mut i = 1;
    for cup in cups {
        if *cup == 1 {
            break;
        }
        i += 1;
    }

    while order.len() < cups.len() - 1 {
        if i >= cups.len() {
            i = 0;
        }
        order.push(cups[i]);
        i += 1;
    }

    order
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut cups: Vec<Cup> = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|x| x.to_string().parse().unwrap())
        .collect();

    let mut index = 0;
    for _ in 0..100 {
        let (c, i) = do_move(cups, index);
        cups = c;
        index = i;
    }

    let ordered = get_cup_order(&cups);
    write!(io::stdout(), "Cups: ")?;
    for c in ordered {
        write!(io::stdout(), "{}", c)?;
    }
    writeln!(io::stdout(), "")?;

    Ok(())
}
