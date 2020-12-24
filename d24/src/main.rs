use std::collections::HashSet;
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
        tile.0 += add.0;
        tile.1 += add.1;
        tile.2 += add.2;
    }

    assert!(tile.0 + tile.1 + tile.2 == 0);

    tile
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut tiles: HashSet<Tile> = HashSet::new();

    for tile in input.lines().map(|x| line_to_tile(x)) {
        if tiles.contains(&tile) {
            tiles.remove(&tile);
        } else {
            tiles.insert(tile);
        }
    }

    let blacks = tiles.len();

    writeln!(io::stdout(), "{} black tiles", blacks)?;

    Ok(())
}
