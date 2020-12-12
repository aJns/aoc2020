use std::io::{self, Read, Write};
use std::string::String;
use std::fmt;

type Coord = (i32, i32);

#[derive(Clone, Copy)]
enum Cardinal {
   N,
   S,
   W,
   E
}

impl Cardinal {
   fn to_deg(self) -> i32 {
      match self {
         Cardinal::N => 0,
         Cardinal::S => 180,
         Cardinal::W => 270,
         Cardinal::E => 90,
      }
   }

   fn from_deg(deg: i32) -> Self {
      match deg {
         0   => Cardinal::N,
         180 => Cardinal::S,
         270 => Cardinal::W,
         90  => Cardinal::E,
         _   => panic!("cant convert to cardinal")
      }
   }
}

#[derive(Clone, Copy)]
enum Action {
   N,
   S,
   E,
   W,
   L,
   R,
   F
}

impl From<char> for Action {
   fn from(c: char) -> Self {
      match c {
         'N' => Action::N,
         'S' => Action::S,
         'E' => Action::E,
         'W' => Action::W,
         'L' => Action::L,
         'R' => Action::R,
         'F' => Action::F,

         x   => panic!("Can't convert {} to State", x)
      }
   }
}

impl fmt::Display for Action {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      let c = match self {
         Action::N => 'N',
         Action::S => 'S',
         Action::E => 'E',
         Action::W => 'W',
         Action::L => 'L',
         Action::R => 'R',
         Action::F => 'F',
      };

      write!(f, "{}", c)
   }
}

#[derive(Clone, Copy)]
struct Command {
   act: Action,
   val: i32
}

impl From<&str> for Command {
   fn from(s: &str) -> Self {
      let cs: Vec<char> = s.chars().collect();

      let act = Action::from(cs[0]);
      let val = cs[1..].iter().collect::<String>().parse().unwrap();

      Command { act, val }
   }
}

struct Ship {
   pos: Coord,
   dir: Cardinal
}

fn calc_f_coord(dir: Cardinal, val: i32) -> Coord {
   match dir {
         Cardinal::N => (0, -val),
         Cardinal::S => (0, val),
         Cardinal::E => (val, 0),
         Cardinal::W => (-val, 0),
   }
}

fn calc_new_dir(old: Cardinal, deg: i32) -> Cardinal {
   let old_deg = old.to_deg();

   let mut new_deg = old_deg + deg;

   if new_deg < 0 {
      new_deg += 360;
   }
   if new_deg >= 360 {
      new_deg -= 360;
   }

   println!("new deg: {}", new_deg);

   Cardinal::from_deg(new_deg)
}

impl Ship {
   fn new() -> Ship {
      Ship { pos: (0,0), dir: Cardinal::E }
   }

   fn exec_cmd(&self, cmd: Command) -> Self {
      let add_c = match cmd.act {
         Action::N  => (0, -cmd.val),
         Action::S  => (0, cmd.val),
         Action::E  => (cmd.val, 0),
         Action::W  => (-cmd.val, 0),
         Action::F  => calc_f_coord(self.dir, cmd.val),
         _          => (0,0)
      };
      let dir = match cmd.act {
         Action::L  => calc_new_dir(self.dir, -cmd.val),
         Action::R  => calc_new_dir(self.dir, cmd.val),
         _          => self.dir

      };

      let pos = (self.pos.0 + add_c.0, self.pos.1 + add_c.1);

      Ship { pos, dir }
   }

   fn manhattan(&self) -> i32 {
      self.pos.0.abs() + self.pos.1.abs()
   }
}

impl fmt::Display for Ship {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      let dir = match self.dir {
         Cardinal::N => 'N',
         Cardinal::S => 'S',
         Cardinal::E => 'E',
         Cardinal::W => 'W',
      };
      write!(f, "pos: {},{}  dir: {}", self.pos.0, self.pos.1, dir)
   }
}


fn main() -> io::Result<()> {
   let mut input = String::new();
   io::stdin().read_to_string(&mut input)?;

   let mut ship = Ship::new();

   let cmds = input.lines().map(|x| Command::from(x));

   for c in cmds {
      println!("action: {}, value: {}", c.act, c.val);
      println!("Ship state: {}", ship);
      ship = ship.exec_cmd(c);
   }
   println!("Ship state: {}", ship);


   writeln!(io::stdout(), "Final manhattan: {}", ship.manhattan())?;

   Ok(())
}

