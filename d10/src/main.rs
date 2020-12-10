use std::collections::HashSet;
use std::io::{self, Read, Write};
use std::string::String;

fn fact(num: u128) -> u128 {
   match num {
      0 => 1,
      1 => 1,
      _ => fact(num-1)*num,
   }
}

fn perm(n: u128, r:  u128) -> u128 {
   let nom = fact(n);
   let denom = fact(n-r);

   nom/denom
}

fn calc_perms(n: u128) -> u128 {
   let mut perms = 0;

   for r in 0..n+1 {
      let c = perm(n, r);
      perms += c;
      println!("r/n: {}/{} : permutations: {}", r, n, c);
   }

   let ret = match perms {
      0 => 0,
      1 => 0,
      x => x
   };

   println!("ret comb: {}", ret);
   ret
}

fn find_diff1_streaks(input: &[u64]) -> Vec<Vec<u64>> {
    let mut p_dist = input[1] - input[0];

    let mut streaks: Vec<Vec<u64>> = Vec::new();

    let mut start = 0;
    let mut end = 0;
    let mut in_streak = false;

    for i in 1..input.len()-1 {
        let n_dist = input[i+1] - input[i];
        let p_dist = input[i] - input[i-1];
        let dist_sum = n_dist + p_dist;

        if dist_sum <= 3 {
            if !in_streak {
                in_streak = true;
                start = i;
            }
        } else {
            if in_streak {
                in_streak = false;
                end = i-1;
                let streak = &input[start..end+1];
                streaks.push(Vec::from(streak));
            }
        }
    }

    streaks
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let input: Vec<u64> = input.lines().map(|x| x.parse().unwrap()).collect();

    let mut sorted = input.clone();

    sorted.push(0);
    sorted.sort();
    sorted.push(sorted.last().unwrap()+3);

    let streaks = find_diff1_streaks(&sorted);

    let mut total_perms = 1;

    for s in streaks {
        println!("streak");
        for i in &s {
            println!("\t{}", i);
        }

        let perms = calc_perms(s.len() as u128);
        total_perms *= perms;

        println!("permutations: {}", perms);
    }

    writeln!(io::stdout(), "total perms: {}", total_perms)?;

    Ok(())
}

