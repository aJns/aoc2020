use std::collections::{HashMap, HashSet};
use std::io::{self, Read, Write};

type Tile = (i32, i32, i32);

fn line_to_tile(line: &str) -> Tile {
    let mut tile = (0, 0, 0);
    let mut chars = line.chars();

    while let Some(c) = chars.next() {
        let add = match c {
            'e' => (1, -1, 0),
            'w' => (-1, 1, 0),
            'n' => match chars.next().unwrap() {
                'e' => (1, 0, -1),
                'w' => (0, 1, -1),
                _ => panic!(),
            },
            's' => match chars.next().unwrap() {
                'e' => (0, -1, 1),
                'w' => (-1, 0, 1),
                _ => panic!(),
            },
            _ => panic!(),
        };
        tile = add_tiles(&tile, &add);
    }

    assert!(tile.0 + tile.1 + tile.2 == 0);

    tile
}

fn add_tiles(a: &Tile, b: &Tile) -> Tile {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2)
}

fn neighbors(tile: &Tile) -> Vec<Tile> {
    let ads = vec![
        (1, -1, 0),
        (-1, 1, 0),
        (1, 0, -1),
        (0, 1, -1),
        (0, -1, 1),
        (-1, 0, 1),
    ];

    let neigh: Vec<Tile> = ads.iter().map(|x| add_tiles(tile, &x)).collect();

    assert!(neigh.len() == 6);
    neigh
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut black_tiles: HashSet<Tile> = HashSet::new();

    for tile in input.lines().map(|x| line_to_tile(x)) {
        if black_tiles.contains(&tile) {
            black_tiles.remove(&tile);
        } else {
            black_tiles.insert(tile);
        }
    }

    for day in 1..=100 {
        let mut energies: HashMap<Tile, usize> = HashMap::new();
        for black in &black_tiles {
            energies.entry(*black).or_insert(0);
            for tile in neighbors(black) {
                let energy = energies.entry(tile).or_insert(0);
                *energy += 1;
            }
        }
        for (k, v) in energies {
            if black_tiles.contains(&k) {
                if v == 0 || v > 2 {
                    black_tiles.remove(&k);
                }
            } else {
                if v == 2 {
                    black_tiles.insert(k);
                }
            }
        }
        writeln!(io::stdout(), "Day {:>4}: {}", day, black_tiles.len())?;
    }

    Ok(())
}
