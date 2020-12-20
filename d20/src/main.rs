use std::collections::HashMap;
use std::collections::HashSet;
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
const MONSTER: &str = r"                  # 
#    ##    ##    ###
 #  #  #  #  #  #   ";

type TileArr = Vec<Vec<Bit>>;

fn rotate_90(tile: &TileArr) -> TileArr {
    let mut new = Vec::new();

    for i in 0..TILE_W {
        let mut col = Vec::new();
        for row in tile.iter().rev() {
            col.push(row[i]);
        }
        new.push(col);
    }

    new
}

fn rot_img_90(image: &Image) -> Image {
    let mut new = Vec::new();

    let img_w = image[0].len();

    for i in 0..img_w {
        let mut col = Vec::new();
        for row in image.iter().rev() {
            col.push(row[i]);
        }
        new.push(col);
    }

    new
}

#[derive(Copy, Clone, PartialEq)]
enum CDir {
    N,
    S,
    E,
    W,
}

impl fmt::Display for CDir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            CDir::N => 'N',
            CDir::S => 'S',
            CDir::E => 'E',
            CDir::W => 'W',
        };
        write!(f, "{}", c)
    }
}

impl CDir {
    fn to_deg(&self) -> i16 {
        match self {
            Self::N => 0,
            Self::S => 180,
            Self::E => 90,
            Self::W => 270,
        }
    }
}

impl From<usize> for CDir {
    fn from(u: usize) -> Self {
        match u {
            0 => CDir::N,
            1 => CDir::S,
            2 => CDir::E,
            3 => CDir::W,
            _ => panic!(),
        }
    }
}

#[derive(Clone)]
struct Neighbor {
    id: u64,
    dir: CDir,
}

#[derive(Clone)]
struct Edge {
    dir: CDir,
    chars: String,
}

#[derive(Clone)]
struct Tile {
    id: u64,
    data: TileArr,
    edges: Vec<Edge>,
    neighbors: Vec<Neighbor>,
}

impl Tile {
    fn set_edges(&mut self) {
        self.edges.clear();

        let north: String = self.data.first().unwrap().iter().collect();
        let south: String = self.data.last().unwrap().iter().collect();

        let mut west = String::new();
        let mut east = String::new();

        for row in &self.data {
            west.push(row.first().unwrap().into());
            east.push(row.last().unwrap().into());
        }

        self.edges.push(Edge {
            dir: CDir::N,
            chars: north,
        });
        self.edges.push(Edge {
            dir: CDir::S,
            chars: south,
        });
        self.edges.push(Edge {
            dir: CDir::E,
            chars: east,
        });
        self.edges.push(Edge {
            dir: CDir::W,
            chars: west,
        });
    }

    fn find_neighbours(&self, tiles: &mut [Self]) {
        for edge in &self.edges {
            let rev: String = edge.chars.chars().rev().collect();

            'outer: for i in 0..tiles.len() {
                let tile = &mut tiles[i];
                if tile.id == self.id {
                    continue;
                }

                let edges = tile.edges.clone();

                for e in edges {
                    if edge.chars == e.chars || rev == *e.chars {
                        let dir = e.dir;
                        let id = self.id;

                        tile.neighbors.push(Neighbor { id, dir });
                        tile.align_edges(&e, edge);

                        break 'outer;
                    }
                }
            }
        }
    }

    fn flip_vert(&mut self) {
        let mut new = Vec::new();
        for row in self.data.iter().rev() {
            let mut new_r = Vec::new();
            for c in row {
                new_r.push(*c);
            }
            new.push(new_r);
        }
        self.data = new;
        self.set_edges();
    }

    fn flip_hor(&mut self) {
        let mut new = Vec::new();
        for row in &self.data {
            let mut new_r = Vec::new();
            for c in row.iter().rev() {
                new_r.push(*c);
            }
            new.push(new_r);
        }
        self.data = new;
        self.set_edges();
    }

    fn rotate_cw_turns(&mut self, cw_turns: i16) {
        if cw_turns <= 0 {
            return;
        }
        for _ in 0..cw_turns {
            self.data = rotate_90(&self.data);
        }
        self.set_edges();
    }

    fn align_edges(&mut self, this: &Edge, other: &Edge) {
        let target_dir = match other.dir {
            CDir::N => CDir::S,
            CDir::S => CDir::N,
            CDir::E => CDir::W,
            CDir::W => CDir::E,
        };

        if this.dir != target_dir {
            let mut diff = target_dir.to_deg() - this.dir.to_deg();
            while diff < 0 {
                diff += 360
            }
            while diff > 360 {
                diff -= 360
            }
            let cw_turns = diff / 90;
            self.rotate_cw_turns(cw_turns);
        }

        let mut new_chars = this.chars.clone();
        for edge in &self.edges {
            if edge.dir == target_dir {
                new_chars = edge.chars.clone();
            }
        }

        if new_chars != other.chars {
            match target_dir {
                CDir::N => self.flip_hor(),
                CDir::S => self.flip_hor(),
                CDir::E => self.flip_vert(),
                CDir::W => self.flip_vert(),
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

            let mut data = vec![vec![Bit::V0; TILE_W]; TILE_W];

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
                neighbors: Vec::new(),
            });
        }
    }

    tiles
}

fn find_edge_match(this: &Tile, other: &Tile) -> Option<(Edge, Edge)> {
    for te in &this.edges {
        let rev: String = te.chars.chars().rev().collect();
        for oe in &other.edges {
            if te.chars == oe.chars || rev == oe.chars {
                return Some((te.clone(), oe.clone()));
            }
        }
    }
    None
}

type Coord = (i16, i16);

fn organize_tiles(
    tile: &Tile,
    pos: Coord,
    tiles: &Vec<Tile>,
    handled: &mut HashSet<u64>,
    tile_map: &mut HashMap<Coord, Tile>,
) {
    tile_map.insert(pos, tile.clone());
    handled.insert(tile.id);

    for t in tiles {
        if handled.contains(&t.id) {
            continue;
        }

        if let Some(edges) = find_edge_match(tile, t) {
            let (this_edge, other_edge) = edges;

            let mut next = t.clone();
            next.align_edges(&other_edge, &this_edge);

            let (i, j) = pos;
            let p = match this_edge.dir {
                CDir::N => (i - 1, j),
                CDir::S => (i + 1, j),
                CDir::E => (i, j + 1),
                CDir::W => (i, j - 1),
            };
            organize_tiles(&next, p, tiles, handled, tile_map);
        }
    }
}

fn is_square(coords: Vec<&Coord>) -> bool {
    let mut rows = HashSet::new();
    let mut cols = HashSet::new();
    for (r, c) in coords {
        rows.insert(r);
        cols.insert(c);
    }
    rows.len() == cols.len()
}

type Image = Vec<Vec<char>>;

fn assemble_image(tiles: &HashMap<Coord, Tile>, dims: &Coord, mins: &Coord) -> Image {
    let sta = 1;
    let end = TILE_W - 1;
    let mut image = Vec::new();
    for ti in 0..=dims.0 {
        for row_i in sta..end {
            let mut row_dat: Vec<char> = Vec::new();
            for tj in 0..=dims.1 {
                let index = (ti + mins.0, tj + mins.1);
                let tile = tiles.get(&index).unwrap();
                let mut row_chars = tile.data[row_i][sta..end]
                    .iter()
                    .map(|x| x.into())
                    .collect();
                row_dat.append(&mut row_chars);
            }
            image.push(row_dat);
        }
    }
    image
}

fn corner_ids(map: &HashMap<Coord, Tile>, dims: &Coord, mins: &Coord) -> Vec<u64> {
    let maxs = (dims.0 + mins.0, dims.1 + mins.1);

    let mut indices = Vec::new();

    indices.push(mins);
    indices.push(&maxs);

    let a = (mins.0, maxs.1);
    let b = (maxs.0, mins.1);

    indices.push(&a);
    indices.push(&b);

    indices.iter().map(|x| map.get(x).unwrap().id).collect()
}

fn find_monsters(monster: &Image, image: &Image) -> u32 {
    let mon_h = monster.len();
    let mon_w = monster[0].len();

    let im_h = image.len();
    let im_w = image[0].len();

    let mut counter = 0;

    for i in 0..(im_h - mon_h) {
        'outer: for j in 0..(im_w - mon_w) {
            let mut test = Vec::new();
            for r in &image[i..(i + mon_h)] {
                test.push(&r[j..(j + mon_w)]);
            }

            for i in 0..mon_h {
                for j in 0..mon_w {
                    if monster[i][j] == ' ' {
                        continue;
                    }
                    if test[i][j] != monster[i][j] {
                        continue 'outer;
                    }
                }
            }
            counter += 1;
        }
    }

    counter
}

fn vert_flip_image(image: &Image) -> Image {
    let mut new = Vec::new();
    for row in image.iter().rev() {
        let mut new_r = Vec::new();
        for c in row {
            new_r.push(*c);
        }
        new.push(new_r);
    }
    new
}
fn hor_flip_image(image: &Image) -> Image {
    let mut new = Vec::new();
    for row in image {
        let mut new_r = Vec::new();
        for c in row.iter().rev() {
            new_r.push(*c);
        }
        new.push(new_r);
    }
    new
}

fn rot_image(image: &Image, cw_turns: i16) -> Image {
    let mut new = image.clone();
    for _ in 0..cw_turns {
        new = rot_img_90(&new);
    }
    new
}

fn calc_waves(image: &Image) -> u32 {
    let mut count = 0;
    for row in image {
        for c in row {
            if *c == '#' {
                count += 1
            }
        }
    }
    count
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut tiles = parse_tiles(&mut input.lines());

    for t in &mut tiles {
        t.set_edges();
    }

    let mut tile_map = HashMap::new();
    let mut mins = (0, 0);
    let mut dims = (0, 0);

    for tile in &tiles {
        let mut handled = HashSet::new();
        let mut t_map = HashMap::new();
        organize_tiles(tile, (0, 0), &tiles, &mut handled, &mut t_map);

        let mut min = (0, 0);
        let mut max = (0, 0);
        for (k, _) in &t_map {
            if k.0 < min.0 {
                min.0 = k.0;
            }
            if k.1 < min.1 {
                min.1 = k.1;
            }
            if k.0 > max.0 {
                max.0 = k.0;
            }
            if k.1 > max.1 {
                max.1 = k.1;
            }
        }

        let keys = t_map.keys();
        if is_square(keys.collect()) {
            tile_map = t_map;
            mins = min;
            dims = (max.0 - min.0, max.1 - min.1);
            break;
        }
    }

    let image = assemble_image(&tile_map, &dims, &mins);

    let mut monster = Vec::new();
    for line in MONSTER.lines() {
        monster.push(line.chars().collect());
    }

    let mut monsters = find_monsters(&monster, &image);

    if monsters == 0 {
        for turns in 0..4 {
            let rot = rot_image(&image, turns);
            monsters = find_monsters(&monster, &rot);

            if monsters == 0 {
                let flipped = vert_flip_image(&rot);
                monsters = find_monsters(&monster, &flipped);
            }

            if monsters == 0 {
                let flipped = hor_flip_image(&rot);
                monsters = find_monsters(&monster, &flipped);
            }

            if monsters != 0 {
                break;
            }
        }
    }

    writeln!(io::stdout(), "{} monsters found", monsters)?;

    let all_waves = calc_waves(&image);
    let monster_waves = monsters * calc_waves(&monster);

    let roughness = all_waves - monster_waves;

    writeln!(io::stdout(), "roughness: {}", roughness)?;

    Ok(())
}
