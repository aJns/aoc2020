use std::fs::File;
use std::io::{self, Write, BufRead};
use std::path::Path;
use std::str;
use std::vec;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_num_vec() -> Result<Vec<i32>, io::Error> {
    let mut num_vec = Vec::new();

    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            let num: i32 = match line?.parse()
            {
                Ok(n)  => n,
                Err(_e) => panic!("asdsdf"),
            };
            num_vec.push(num);
        }
    }

    Ok(num_vec)
}

fn main() -> io::Result<()> {

    let num_vec = get_num_vec()?;

    'outer: for a in num_vec.iter() {
        for b in num_vec.iter() {
            for c in num_vec.iter() {
                if (a+b+c) == 2020 {
                    writeln!(io::stdout(), "{}", a*b*c)?;
                    break 'outer;
                }
            }
        }
    }

    Ok(())
}

//writeln!(io::stdout(), "{}", num)?;

