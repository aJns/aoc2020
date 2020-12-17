use std::cmp::max;
use std::cmp::min;
use std::fmt;
use std::io::{self, Read, Write};
use std::str;
use std::string::String;

#[derive(Copy, Clone, PartialEq)]
enum State {
    Dead,
    Alive,
}

#[derive(Copy, Clone)]
struct Cell {
    state: State,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        let state = match c {
            '.' => State::Dead,
            '#' => State::Alive,
            _ => panic!("invalid char"),
        };

        Cell { state }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self.state {
            State::Dead => '.',
            State::Alive => '#',
        };
        write!(f, "{}", c)
    }
}

const WS: usize = 32;
const MID: usize = WS / 2;
type World = [[[Cell; WS]; WS]; WS];

fn visualize_world(world: &World) {
    let x_len = world.len();
    let y_len = world[0].len();
    let z_len = world[0][0].len();

    for z in 0..z_len {
        println!("z={}", (z as i32) - (MID as i32));
        println!("----------------------");
        for x in 0..x_len {
            for y in 0..y_len {
                print!("{}", world[x][y][z]);
            }
            println!("");
        }
        println!("----------------------");
    }
}

fn init_world(world: &mut World, lines: str::Lines) {
    let mut x = MID;
    for line in lines {
        let mut y = MID;
        for c in line.chars() {
            world[x][y][MID] = Cell::from(c);
            y += 1;
        }
        x += 1;
    }
}

fn count_actives(world: &World) -> u32 {
    let mut count = 0;
    for x in 0..WS {
        for y in 0..WS {
            for z in 0..WS {
                if world[x][y][z].state == State::Alive {
                    count += 1;
                }
            }
        }
    }

    count
}

fn count_neighbours(world: &World, indices: (usize, usize, usize)) -> u8 {
    let (x, y, z) = indices;
    let i_max = WS - 1;
    let mut count = 0;

    let x_min = max((x as i64) - 1, 0) as usize;
    let y_min = max((y as i64) - 1, 0) as usize;
    let z_min = max((z as i64) - 1, 0) as usize;

    for i in x_min..=min(x + 1, i_max) {
        for j in y_min..=min(y + 1, i_max) {
            for k in z_min..=min(z + 1, i_max) {
                if (i, j, k) == (x, y, z) {
                    continue;
                }
                if world[i][j][k].state == State::Alive {
                    count += 1;
                }
                // println!("\t{},{},{}", i, j, k);
            }
        }
    }
    // println!("Cell at: {},{},{} has {} neighbours", x, y, z, count);

    count
}

fn run_cycle(world: &World) -> World {
    let mut new = *world;

    for x in 0..WS {
        for y in 0..WS {
            for z in 0..WS {
                let cube = world[x][y][z];
                let nc = count_neighbours(world, (x, y, z));

                let state = match cube.state {
                    State::Alive => match nc {
                        2..=3 => State::Alive,
                        _ => State::Dead,
                    },
                    State::Dead => match nc {
                        3 => State::Alive,
                        _ => State::Dead,
                    },
                };
                new[x][y][z] = Cell { state };

                if cube.state == State::Alive {
                    if x == 0 || y == 0 || z == 0 || WS - 1 == x || WS - 1 == y || WS - 1 == z {
                        panic!("Edge cube alive");
                    }
                }
            }
        }
    }

    new
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut world = [[[Cell { state: State::Dead }; WS]; WS]; WS];

    init_world(&mut world, input.lines());

    // visualize_world(&world);

    for i in 0..6 {
        world = run_cycle(&world);
        // let actives = count_actives(&world);
        // println!("{} Actives after {} cycle", actives, i + 1);
        // visualize_world(&world);
    }

    let actives = count_actives(&world);

    writeln!(io::stdout(), "{} active cells", actives)?;

    Ok(())
}
