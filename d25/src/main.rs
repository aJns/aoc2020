use std::io::{self, Read, Write};

type Key = u64;

fn get_public_keys(mut lines: std::str::Lines) -> (Key, Key) {
    let card = lines.next().unwrap().parse().unwrap();
    let door = lines.next().unwrap().parse().unwrap();

    (card, door)
}

const DIV: Key = 20201227;

fn transform_number(subject: Key, loop_size: usize) -> Key {
    let mut val = 1;

    for _ in 0..loop_size {
        val = val * subject;
        val = val % DIV;
    }

    val
}

fn find_loop_size(key: Key) -> usize {
    let subject = 7;

    let mut ls = 0;
    let mut val = 1;

    loop {
        ls += 1;
        if ls % 1000 == 0 {
            println!("ls: {}", ls);
        }
        val = val * subject;
        val = val % DIV;
        if key == val {
            return ls;
        }
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let (card_public, door_public) = get_public_keys(input.lines());

    let card_loop = find_loop_size(card_public);
    println!("Card loop size: {}", card_loop);
    let door_loop = find_loop_size(door_public);
    println!("Door loop size: {}", door_loop);

    let encrypt1 = transform_number(door_public, card_loop);
    println!("Encrypt1: {}", encrypt1);
    let encrypt2 = transform_number(card_public, door_loop);
    println!("Encrypt2: {}", encrypt2);

    assert_eq!(encrypt1, encrypt2);

    // writeln!(io::stdout(), "hi")?;

    Ok(())
}
