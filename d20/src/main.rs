use std::fmt;
use std::io::{self, Read, Write};
use std::iter::FromIterator;

#[derive(Copy, Clone, PartialEq)]
enum Bit {
    V0,
    V1,
}

impl From<char> for Bit {
    fn from(c: char) -> Self {
        match c {
            '.' => Bit::V0,
            '#' => Bit::V1,
            _ => panic!(),
        }
    }
}

impl From<&Bit> for char {
    fn from(b: &Bit) -> Self {
        match b {
            Bit::V0 => '.',
            Bit::V1 => '#',
        }
    }
}

impl fmt::Display for Bit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c: char = self.into();
        write!(f, "{}", c)
    }
}
impl<'a> FromIterator<&'a Bit> for String {
    fn from_iter<I: IntoIterator<Item = &'a Bit>>(iter: I) -> Self {
        let mut s = String::new();

        for i in iter {
            s.push(i.into());
        }

        s
    }
}

const TILE_W: usize = 10;

type TileArr = [[Bit; TILE_W]; TILE_W];

#[derive(Clone)]
struct Tile {
    id: u64,
    data: TileArr,
    edges: Vec<String>,
    n_count: usize,
}

impl Tile {
    fn set_edges(&mut self) {
        let north: String = self.data.first().unwrap().iter().collect();
        let south: String = self.data.last().unwrap().iter().collect();

        let mut west = String::new();
        let mut east = String::new();

        for row in &self.data {
            west.push(row.first().unwrap().into());
            east.push(row.last().unwrap().into());
        }

        self.edges.push(north);
        self.edges.push(south);
        self.edges.push(west);
        self.edges.push(east);
    }

    fn find_neighbours(&mut self, tiles: &[Self]) {
        while let Some(edge) = self.edges.pop() {
            let rev: String = edge.chars().rev().collect();

            'outer: for tile in tiles {
                if tile.id == self.id {
                    continue;
                }

                for e in &tile.edges {
                    if edge == *e || rev == *e {
                        self.n_count += 1;
                        break 'outer;
                    }
                }
            }
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "tile id: {}", self.id)?;

        for row in &self.data {
            for col in row {
                write!(f, "{}", col)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

fn parse_tiles(lines: &mut std::str::Lines) -> Vec<Tile> {
    let mut tiles = Vec::new();
    while let Some(line) = lines.next() {
        if line.contains("Tile") {
            let chars = line.chars();

            let skip = chars.skip_while(|x| *x != ' ');
            let num: String = skip.take_while(|x| *x != ':').collect();

            let id = num.trim().parse().unwrap();

            let mut data = [[Bit::V0; TILE_W]; TILE_W];

            for i in 0..TILE_W {
                let line = lines.next().unwrap();
                let mut chars = line.chars();
                for j in 0..TILE_W {
                    let c = chars.next().unwrap();
                    data[i][j] = Bit::from(c);
                }
            }

            tiles.push(Tile {
                id,
                data,
                edges: Vec::new(),
                n_count: 0,
            });
        }
    }

    tiles
}

fn find_corners(tiles: &[Tile]) -> Vec<&Tile> {
    let mut vec = Vec::new();
    for tile in tiles {
        if tile.n_count == 2 {
            vec.push(tile)
        }
    }
    vec
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut tiles = parse_tiles(&mut input.lines());

    for t in &mut tiles {
        t.set_edges();
    }

    let mut copy = tiles.clone();
    for t in &mut copy {
        t.find_neighbours(&tiles);
    }

    tiles = copy;

    let corner_ids: Vec<u64> = find_corners(&tiles).iter().map(|x| x.id).collect();

    let mult = corner_ids.iter().fold(1, |acc, x| x * acc);

    writeln!(io::stdout(), "Corner ids multiplied: {}", mult)?;

    Ok(())
}
