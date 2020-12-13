use std::io::{self, Read, Write};
use std::string::String;
use std::fmt;


fn parse_id(id: &str) -> i32 {
   match id.parse::<i32>() {
      Ok(x)     => x,
      Err(_)    => 0
   }
}

fn calc_waiting_time(arrival: i32, bus_id: i32) -> i32 {
   if arrival < bus_id {
      return bus_id - arrival
   }

   let times = (arrival / bus_id) + 1;
   times*bus_id - arrival
}

fn main() -> io::Result<()> {
   let mut input = String::new();
   io::stdin().read_to_string(&mut input)?;


   let lines: Vec<&str> = input.lines().collect();

   let timestamp: i32 = lines[0].parse().unwrap();

   let bus_ids: Vec<i32> = lines[1].split(',').map(|x| parse_id(x)).collect();

   let mut earliest = (0, 1_000_000_000);  // id, wait

   for id in bus_ids {
      if id == 0 {
         continue;
      }

      let wait = calc_waiting_time(timestamp, id);

      if wait < earliest.1  {
         earliest = (id, wait);
      }
   }

   let mult = earliest.0 * earliest.1;

   writeln!(io::stdout(), "id: {}, waiting time: {}. multiplied: {}",
      earliest.0, earliest.1, mult)?;

   Ok(())
}

