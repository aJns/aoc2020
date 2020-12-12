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

fn restrict_deg(mut deg: i32) -> i32 {
      if deg < 0 {
         deg += 360;
      }
      if deg >= 360 {
         deg -= 360;
      }
      deg
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

      match restrict_deg(deg) {
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
   wpt: Coord
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

   println!("new deg: {}", new_deg);

   Cardinal::from_deg(new_deg)
}

fn move_to_wpt(old: Coord, wpt: Coord, times: i32) -> Coord {
   let mut new = old;

   for _ in 0..times {
      new.0 += wpt.0;
      new.1 += wpt.1;
   }

   new
}

fn rotate_wpt(old: Coord, deg: i32) -> Coord {
   print!("Rotating {},{} by {} degrees: ", old.0, old.1, deg);
   let ret = match restrict_deg(deg) {
      0     => old,

      90    => (-old.1, old.0),
      180   => (-old.0, -old.1),
      270   => (old.1, -old.0),

      _     => panic!("cant handle deg")
   };

   println!("{},{}", ret.0, ret.1);

   ret
}

impl Ship {
   fn new() -> Ship {
      Ship { pos: (0,0), wpt: (10,-1) }
   }

   fn exec_cmd(&self, cmd: Command) -> Self {
      let add_wpt = match cmd.act {
         Action:: N => (0, -cmd.val),
         Action:: S => (0, cmd.val),
         Action:: E => (cmd.val, 0),
         Action:: W => (-cmd.val, 0),
         _          => (0, 0)
      };

      let wpt = match cmd.act {
         Action:: L => rotate_wpt(self.wpt, -cmd.val),
         Action:: R => rotate_wpt(self.wpt, cmd.val),
         _          => (self.wpt.0 + add_wpt.0, self.wpt.1 + add_wpt.1)
      };

      let pos = match cmd.act {
         Action::F  => move_to_wpt(self.pos, wpt, cmd.val),
         _          => self.pos
      };

      Ship { pos, wpt }
   }

   fn manhattan(&self) -> i32 {
      self.pos.0.abs() + self.pos.1.abs()
   }
}

impl fmt::Display for Ship {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "pos: {},{}  wpt: {},{}", self.pos.0, self.pos.1, self.wpt.0, self.wpt.1)
   }
}


fn main() -> io::Result<()> {
   let mut input = String::new();
   io::stdin().read_to_string(&mut input)?;

   let mut ship = Ship::new();

   let cmds = input.lines().map(|x| Command::from(x));

   println!("Ship state: {}", ship);
   for c in cmds {
      println!("action: {}, value: {}", c.act, c.val);
      ship = ship.exec_cmd(c);
      println!("Ship state: {}", ship);
   }


   writeln!(io::stdout(), "Final manhattan: {}", ship.manhattan())?;

   Ok(())
}

