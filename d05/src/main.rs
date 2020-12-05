use std::io::{self, Read, Write};
use std::iter::Iterator;
use std::str::{Lines, FromStr};
use std::string::ParseError;
use std::collections::HashMap;

enum Half {
    Upper,
    Lower,
}

type List = Vec<u32>;

fn get_half(list: &List, dir: Half) -> List {

    let start = 0;
    let middle = (list.len()/2) as usize;
    let end = list.len() as usize;

    match dir {
        Half::Upper => list[middle..end].to_vec(),
        Half::Lower => list[start..middle].to_vec()
    }
}

fn calc_seat_id(row: u32, col: u32) -> u32 {
    (row*8 + col)
}


fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut highest_seat_id = 0;

    for line in input.lines() {

        let mut rows: List = (0..128).collect();
        let mut cols: List = (0..8).collect();

        let mut cit = line.chars();

        while rows.len() > 1 {
            if let Some(c) = cit.next() {
                rows = match c {
                    'F' => get_half(&rows, Half::Lower),
                    'B' => get_half(&rows, Half::Upper),
                    _ => panic!("asd")
                };
            }
        }

        while cols.len() > 1 {
            if let Some(c) = cit.next() {
                cols = match c {
                    'L' => get_half(&cols, Half::Lower),
                    'R' => get_half(&cols, Half::Upper),
                    _ => panic!("asd")
                };
            }
        }
        println!("Row: {}", rows[0]);
        println!("Col: {}", cols[0]);

        let seat_id = calc_seat_id(rows[0], cols[0]);

        if highest_seat_id < seat_id {
            highest_seat_id = seat_id;
        }
    }


    writeln!(io::stdout(), "Highest seat ID: {}", highest_seat_id)?;

    Ok(())
}

