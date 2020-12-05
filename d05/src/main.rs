use std::io::{self, Read, Write};
use std::iter::Iterator;

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
    row*8 + col
}


fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut remaining_seat_ids: Vec<i64> = (0..127*8+7).collect();

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

        let seat_id = calc_seat_id(rows[0], cols[0]);
        remaining_seat_ids[seat_id as usize] = -1;
    }

    let mut my_seat = 0;

    let mut prev = 0;
    for id in remaining_seat_ids {
        if prev == -1 && id != -1 {
            my_seat = id;
            break;
        }
        prev = id;
    }


    writeln!(io::stdout(), "Your seat ID: {}", my_seat)?;

    Ok(())
}

