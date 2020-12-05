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

fn get_input_lines() -> Result<Vec<String>, io::Error> {
    let mut vec = Vec::new();

    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            let pass: String = match line?.parse()
            {
                Ok(n)  => n,
                Err(_e) => panic!("asdsdf"),
            };
            vec.push(pass);
        }
    }

    Ok(vec)
}

fn get_map_table(lines: Vec<String>) -> Vec<Vec<char>> {
    let vec = lines.into_iter().map(|x| x.chars().collect()).collect();

    return vec;
}

fn main() -> io::Result<()> {

    let input_vec = get_input_lines()?;
    let map_table = get_map_table(input_vec);

    let mut counter: i32 = 0;

    let mut i = map_table.iter();
    i.next();   // first row not needed

    let x_add = 3;
    let y_add = 1;

    let mut x_i = x_add;

    loop {
        let row = match i.nth(y_add-1) {
            Some(x) => x,
            None    => break
        };

        let len = row.len();

        x_i = match x_i < len {
            true    => x_i,
            false   => x_i - len
        };

        let is_tree = row[x_i] == '#';

        counter += match is_tree {
            true    => 1,
            false   => 0
        };

        let m = match is_tree {
            true    => 'X',
            false   => 'O'
        };

        let mut i = 0;
        for c in row {
            if i == x_i {
                print!("{}", m);
            }
            else
            {
            print!("{}", c);
            }
            i += 1;
        }
        println!("");

        x_i += x_add
    }


    writeln!(io::stdout(), "We hit {} trees", counter)?;

    Ok(())
}

