use std::io::{self, Read, Write};
use std::string::String;
use std::fmt;

#[derive(PartialEq, Clone)]
enum State {
   Floor,
   Free,
   Occupied
}

impl From<char> for State {
   fn from(c: char) -> Self {
      match c {
         '.' => State::Floor,
         'L' => State::Free,
         '#' => State::Occupied,
         x   => panic!("Can't convert {} to State", x)
      }
   }
}

impl fmt::Display for State {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      let c = match self {
         State::Floor     => '.',
         State::Free      => 'L',
         State::Occupied  => '#'
      };

      write!(f, "{}", c)
   }
}

type CellMatrix = Vec<Vec<State>>;

fn parse_to_cells(lines: &[&str]) -> CellMatrix {
   let mut outer = Vec::new();

   for l in lines {
      let mut inner = Vec::new();

      for c in l.chars() {
         inner.push(State::from(c));
      }

      outer.push(inner);
   }

   outer
}

fn index_occ(cells: &CellMatrix, ij: (i64, i64), count: &mut u8) -> bool {
   let i = ij.0 as usize;
   let j = ij.1 as usize;

   match cells[i][j] {
      State::Floor    => false,
      State::Free     => true,
      State::Occupied => { *count += 1; true },
   }
}

fn count_neighbors(cells: &CellMatrix, index: (usize, usize)) -> u8 {
   // This seems really dumb, but I'm too lazy and tired to do it better

   let row_n = cells.len();
   let col_n = cells[0].len();

   let bounded = |xy: (i64, i64)| {
      let row_ok = xy.0 >= 0 && xy.0 < row_n as i64;
      let col_ok = xy.1 >= 0 && xy.1 < col_n as i64;

      return row_ok && col_ok
   };
   
   let mut count = 0;
   let mut ij = (0,0);

   // rows
   ij.0 = 1 + index.0 as i64;
   ij.1 = index.1 as i64;

   while bounded(ij) {
      if index_occ(cells, ij, &mut count) {
         break;
      }
      ij = (ij.0+1, ij.1);
   }
   ij.0 = -1 + index.0 as i64;

   while bounded(ij) {
      if index_occ(cells, ij, &mut count) {
         break;
      }
      ij = (ij.0-1, ij.1);
   }

   // cols
   ij.0 = index.0 as i64;
   ij.1 = 1 + index.1 as i64;

   while bounded(ij) {
      if index_occ(cells, ij, &mut count) {
         break;
      }
      ij = (ij.0, ij.1+1);
   }
   ij.1 = -1 + index.1 as i64;

   while bounded(ij) {
      if index_occ(cells, ij, &mut count) {
         break;
      }
      ij = (ij.0, ij.1-1);
   }

   // diags
   ij.0 = 1+index.0 as i64;
   ij.1 = 1+index.1 as i64;

   while bounded(ij) {
      if index_occ(cells, ij, &mut count) {
         break;
      }
      ij = (ij.0+1, ij.1+1);
   }

   ij.0 = -1+index.0 as i64;
   ij.1 = -1+index.1 as i64;

   while bounded(ij) {
      if index_occ(cells, ij, &mut count) {
         break;
      }
      ij = (ij.0-1, ij.1-1);
   }

   ij.0 = 1+index.0 as i64;
   ij.1 = -1+index.1 as i64;

   while bounded(ij) {
      if index_occ(cells, ij, &mut count) {
         break;
      }
      ij = (ij.0+1, ij.1-1);
   }

   ij.0 = -1+index.0 as i64;
   ij.1 = 1+index.1 as i64;

   while bounded(ij) {
      if index_occ(cells, ij, &mut count) {
         break;
      }
      ij = (ij.0-1, ij.1+1);
   }


   // print!("{}", count);

   count
}

fn is_stable(old: &CellMatrix, new: &CellMatrix) -> bool {
   old == new
}

fn new_state(old: &State, n_count: u8) -> State {
   let free = |n| {
      match n {
         0 => State::Occupied,
         _ => State::Free
      }
   };

   let occ = |n| {
      match n {
         0..=4 => State::Occupied,
         _     => State::Free
      }
   };

   match old {
      State::Floor    => State::Floor,
      State::Free     => free(n_count),
      State::Occupied => occ(n_count)
   }
}

fn count_occupied(cells: &CellMatrix) -> u64 {
   let mut counter = 0;

   for row in cells {
      for c in row {
         if *c == State::Occupied {
            counter += 1;
         }
      }
   }

   counter
}

fn main() -> io::Result<()> {
   let mut input = String::new();
   io::stdin().read_to_string(&mut input)?;

   let lines: Vec<&str> = input.lines().collect();
   let cells = parse_to_cells(&lines);
   let mut old_world = cells.clone();
   let mut new_world = old_world.clone();

   let rows = cells.len();
   let cols = cells[0].len();

   let mut round = 0;

   loop {
      writeln!(io::stdout(), "round {}", round)?;
      writeln!(io::stdout(), "-----------")?;

      for i in 0..rows {
         for j in 0..cols {
            let c = &old_world[i][j];
            // write!(io::stdout(), "{}", c)?;

            let n_count = count_neighbors(&old_world, (i, j));
            let new = new_state(c, n_count);
            new_world[i][j] = new;
         }
         print!("\t");
         for j in 0..cols {
            let c = &old_world[i][j];
            print!("{}", c);
         }
         writeln!(io::stdout(), "")?;
      }
      // let occupied_seats = count_occupied(&old_world);
      // writeln!(io::stdout(), "-----------")?;
      // writeln!(io::stdout(), "{} occupied seats", occupied_seats)?;
      writeln!(io::stdout(), "-----------")?;

      if is_stable(&old_world, &new_world) {
         break;
      }
      old_world = new_world.clone();
      round += 1;
   }

   writeln!(io::stdout(), "Stablized after {} rounds", round)?;
   writeln!(io::stdout(), "-----------")?;

   let occupied_seats = count_occupied(&old_world);
   writeln!(io::stdout(), "{} occupied seats", occupied_seats)?;

   Ok(())
}

