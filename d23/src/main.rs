use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::rc::{Rc, Weak};

type Label = usize;

#[derive(Clone, Copy)]
struct Cup {
    label: Label,
    c_wise: Label,
    anti: Label,
}

type Game = HashMap<Label, Cup>;

fn print_cups(game: &Game, start: Label) {
    let mut curr = start;
    let mut next = game.get(&start).unwrap().c_wise;

    while next != start {
        print!("{} ", curr);
        curr = next;
        next = game.get(&curr).unwrap().c_wise;
    }
    println!("");
}

static mut MOVE_COUNT: usize = 0;

fn take_next_clockwise(game: &mut Game, next_from: &Label, n: usize) -> Vec<Cup> {
    let mut cups = Vec::new();

    let start_label = *next_from;
    let mut end_label = game.get(next_from).unwrap().c_wise;

    let mut prev_label = next_from;
    for _ in 0..n {
        let cup_label = game.get(prev_label).unwrap().c_wise;
        let cup = game.get(&cup_label).unwrap();
        end_label = cup.c_wise;

        cups.push(*cup);
        prev_label = &cup.label;
    }

    let mut start_cup = game.remove(&start_label).unwrap();
    let mut end_cup = game.remove(&end_label).unwrap();

    start_cup.c_wise = end_cup.label;
    end_cup.anti = start_cup.label;

    game.insert(start_label, start_cup);
    game.insert(end_label, end_cup);

    cups
}

fn insert_next_clockwise(game: &mut Game, next_to: &Label, cups: Vec<Cup>) {
    let start_label = next_to;
    let end_label = game.get(next_to).unwrap().c_wise;

    let first_label = cups.first().unwrap().label;
    let last_label = cups.last().unwrap().label;

    // TODO: luultavasti liian kallis operaatio
    let mut start = game.remove(&start_label).unwrap();
    let mut end = game.remove(&end_label).unwrap();
    let mut first = game.remove(&first_label).unwrap();
    let mut last = game.remove(&last_label).unwrap();

    start.c_wise = first.label;
    first.anti = start.label;

    end.anti = last.label;
    last.c_wise = end.label;

    // TODO: luultavasti liian kallis operaatio
    game.insert(start.label, start);
    game.insert(end.label, end);
    game.insert(first.label, first);
    game.insert(last.label, last);
}

fn do_move(mut game: Game, current: Label, max: Label) -> (Game, Label) {
    unsafe {
        if MOVE_COUNT % 1_000 == 0 {
            println!("Move: {}", MOVE_COUNT);
        }
        MOVE_COUNT += 1;
    }

    let three = take_next_clockwise(&mut game, &current, 3);

    let mut dest = current - 1;

    'outer: loop {
        for t in &three {
            if dest == t.label {
                dest -= 1;
                continue 'outer;
            }
        }
        if dest == 0 {
            dest = max;
            continue;
        }
        break;
    }

    insert_next_clockwise(&mut game, &dest, three);

    let next = game.get(&current).unwrap().c_wise;

    (game, next)
}

fn arrange_labels(mut labels: Vec<Label>, n: usize) -> (Vec<Label>, Label) {
    let mut highest = 0;
    for l in &labels {
        if l > &highest {
            highest = *l;
        }
    }
    let mut l = highest;

    for _ in labels.len()..n {
        l += 1;
        labels.push(l);
    }

    (labels, l)
}

fn build_cups(labels: &Vec<Label>) -> Game {
    let mut game: Game = HashMap::new();

    let first_label = labels.first().unwrap();
    let last_label = labels.last().unwrap();

    let mut prev_label = first_label;
    for label in labels {
        let mut cup = Cup {
            label: *label,
            c_wise: 0,
            anti: 0,
        };

        if label != prev_label {
            if let Some(mut prev_cup) = game.get_mut(prev_label) {
                cup.anti = prev_cup.label;
                prev_cup.c_wise = cup.label;
            }
        }

        prev_label = label;
        game.insert(*label, cup);
    }

    let mut first_cup = game.remove(first_label).unwrap();
    first_cup.anti = *last_label;
    game.insert(*first_label, first_cup);

    let mut last_cup = game.remove(last_label).unwrap();
    last_cup.c_wise = *first_label;
    game.insert(*last_label, last_cup);

    game
}

fn get_star_cups(cups: &Game) -> (Label, Label) {
    let labels = get_label_from1(cups);

    return (labels[0], labels[1]);
}

fn get_label_from1(cups: &Game) -> Vec<Label> {
    let mut cup_label = 0;

    for (k, v) in cups {
        if *k == 1 {
            cup_label = v.c_wise;
            break;
        }
    }

    let mut labels = Vec::new();
    loop {
        if cup_label == 1 {
            break;
        }
        labels.push(cup_label);

        cup_label = cups.get(&cup_label).unwrap().c_wise;
    }
    labels
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut labels: Vec<Label> = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|x| x.to_string().parse().unwrap())
        .collect();

    let cup_n = 1_000_000;
    let (ls, max) = arrange_labels(labels, cup_n);
    labels = ls;

    assert_eq!(labels.len(), cup_n);

    let mut cups = build_cups(&labels);
    let mut label = *labels.first().unwrap();
    for _ in 0..10_000_000 {
        let (c, l) = do_move(cups, label, max);
        cups = c;
        label = l;
    }

    // println!("cups: ");
    // for l in &get_label_from1(&cups) {
    //     print!("{}", l);
    // }
    // println!("");

    let star_cups = get_star_cups(&cups);
    writeln!(io::stdout(), "Star cups: {}, {}", star_cups.0, star_cups.1)?;
    let mult = star_cups.0 * star_cups.1;
    writeln!(io::stdout(), "Multiplied: {}", mult)?;

    Ok(())
}
