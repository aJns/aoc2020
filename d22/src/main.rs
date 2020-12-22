use std::collections::{HashSet, VecDeque};
use std::io::{self, Read, Write};

type Deck = VecDeque<usize>;

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

#[derive(Eq, Hash, Clone, PartialEq)]
struct GameState {
    p1: Deck,
    p2: Deck,
}

fn print_state(state: &GameState) {
    print!("Player 1's deck: ");
    for c in state.p1.iter() {
        print!("{}, ", c);
    }
    println!("");

    print!("Player 2's deck: ");
    for c in state.p2.iter() {
        print!("{}, ", c);
    }
    println!("");
}

fn play_round(
    mut state: GameState,
    mut round: usize,
    game: usize,
    states: &mut HashSet<GameState>,
) -> (GameState, bool) {
    loop {
        println!("-- Round {:3} --", round);

        print_state(&state);

        for prev in states.iter() {
            if &state == prev {
                println!("loop: ");
                return (state.clone(), true);
            }
        }
        states.insert(state.clone());

        let mut p1 = state.p1.clone();
        let mut p2 = state.p2.clone();

        let c1 = p1.pop_front().unwrap();
        let c2 = p2.pop_front().unwrap();
        println!("Player 1 plays: {}", c1);
        println!("Player 2 plays: {}", c2);

        let next;

        if c1 <= p1.len() && c2 <= p2.len() {
            let mut p1_copy = VecDeque::new();

            let mut work = p1.clone();
            for _ in 0..c1 {
                p1_copy.push_back(work.pop_front().unwrap());
            }

            let mut p2_copy = VecDeque::new();

            let mut work = p2.clone();
            for _ in 0..c2 {
                p2_copy.push_back(work.pop_front().unwrap());
            }

            let temp = GameState {
                p1: p1_copy,
                p2: p2_copy,
            };
            let (_, p1_won) = play_game(&temp, game + 1);

            if p1_won {
                p1.push_back(c1);
                p1.push_back(c2);
                println!("Player 1 wins the round!");
            } else {
                p2.push_back(c2);
                p2.push_back(c1);
                println!("Player 2 wins the round!");
            }

            next = GameState {
                p1: p1.clone(),
                p2: p2.clone(),
            };

            if p1.is_empty() {
                return (next, false);
            }
            if p2.is_empty() {
                return (next, true);
            }
        } else {
            if c1 > c2 {
                p1.push_back(c1);
                p1.push_back(c2);
                println!("Player 1 wins the round!");
            } else {
                p2.push_back(c2);
                p2.push_back(c1);
                println!("Player 2 wins the round!");
            }

            next = GameState {
                p1: p1.clone(),
                p2: p2.clone(),
            };

            if p1.is_empty() {
                return (next, false);
            }
            if p2.is_empty() {
                return (next, true);
            }
        }
        round += 1;
        state = next;
    }
}

fn calc_score(winner: &Deck) -> usize {
    let mut mult = 0;
    let mut count = 0;

    for c in winner.iter().rev() {
        mult += 1;
        count += c * mult;
    }

    count
}

fn play_game(state: &GameState, game: usize) -> (GameState, bool) {
    println!("== Game {:3} ==", game);
    let mut states = HashSet::new();
    let ret = play_round(state.clone(), 1, game, &mut states);
    println!("Game {} over", game);
    ret
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let (p1, p2) = parse_decks(input.lines());

    let state = GameState { p1, p2 };

    println!("Player 1:");
    for c in &state.p1 {
        println!("{}", c);
    }
    println!("");

    println!("Player 2:");
    for c in &state.p2 {
        println!("{}", c);
    }

    let (end_state, p1_won) = play_game(&state, 1);

    println!("");
    println!("");
    println!("== Post-game results ==");

    print!("Player 1's deck: ");
    for c in end_state.p1.iter() {
        print!("{}, ", c);
    }
    println!("");

    print!("Player 2's deck: ");
    for c in end_state.p2.iter() {
        print!("{}, ", c);
    }
    println!("");

    let winner = match p1_won {
        true => end_state.p1,
        false => end_state.p2,
    };
    let score;
    score = calc_score(&winner);

    writeln!(io::stdout(), "Score: {}", score)?;

    Ok(())
}
