use std::collections::VecDeque;
use std::io::{self, Read, Write};

type Deck = VecDeque<u32>;

fn parse_decks(mut lines: std::str::Lines) -> (Deck, Deck) {
    let mut p1 = VecDeque::new();
    let mut p2 = VecDeque::new();

    while let Some(line) = lines.next() {
        if line.contains("Player 1:") {
            while let Some(line) = lines.next() {
                if let Ok(card) = line.parse() {
                    p1.push_back(card);
                } else {
                    break;
                }
            }
        }
        if line.contains("Player 2:") {
            while let Some(line) = lines.next() {
                if let Ok(card) = line.parse() {
                    p2.push_back(card);
                } else {
                    break;
                }
            }
        }
    }
    (p1, p2)
}

fn play_round(p1: &mut Deck, p2: &mut Deck, round: &mut u32) -> bool {
    println!("-- Round {:3} --", round);

    print!("Player 1's deck: ");
    for c in p1.iter() {
        print!("{}, ", c);
    }
    println!("");

    print!("Player 2's deck: ");
    for c in p2.iter() {
        print!("{}, ", c);
    }
    println!("");

    let c1 = p1.pop_front().unwrap();
    let c2 = p2.pop_front().unwrap();

    println!("Player 1 plays: {}", c1);
    println!("Player 2 plays: {}", c2);

    if c1 > c2 {
        p1.push_back(c1);
        p1.push_back(c2);
        println!("Player 1 wins the round!");
    } else {
        p2.push_back(c2);
        p2.push_back(c1);
        println!("Player 2 wins the round!");
    }
    *round += 1;

    if p1.is_empty() || p2.is_empty() {
        return true;
    }
    false
}

fn calc_score(winner: &Deck) -> u32 {
    let mut mult = 0;
    let mut count = 0;

    for c in winner.iter().rev() {
        mult += 1;
        count += c * mult;
    }

    count
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let (mut p1, mut p2) = parse_decks(input.lines());

    println!("Player 1:");
    for c in &p1 {
        println!("{}", c);
    }
    println!("");

    println!("Player 2:");
    for c in &p2 {
        println!("{}", c);
    }

    let mut round = 1;
    while !play_round(&mut p1, &mut p2, &mut round) {
        println!("");
    }

    println!("");
    println!("");
    println!("== Post-game results ==");

    print!("Player 1's deck: ");
    for c in p1.iter() {
        print!("{}, ", c);
    }
    println!("");

    print!("Player 2's deck: ");
    for c in p2.iter() {
        print!("{}, ", c);
    }
    println!("");

    let score;
    if p1.is_empty() {
        score = calc_score(&p2);
    } else {
        score = calc_score(&p1);
    }

    writeln!(io::stdout(), "Score: {}", score)?;

    Ok(())
}
