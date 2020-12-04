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

fn get_pass_vec() -> Result<Vec<String>, io::Error> {
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

struct Password {
    word: String,
    cha: char,
    min: usize,
    max: usize,
}

fn parse_pass(pass: &str) -> Option<Password> {
    let mut split = pass.split(": ");

    let mut rule = split.next()?.split_whitespace();

    let minmax = rule.next();
    let cha = rule.next()?.chars().next()?;
    assert_eq!(None, rule.next());

    let mut mm_i = minmax?.split("-");

    let min = match mm_i.next()?.parse() {
        Ok(x) => x,
        Err(_) => panic!("asldkfjlasdhf")
    };
    let max = match mm_i.next()?.parse() {
        Ok(x) => x,
        Err(_) => panic!("ioaup9q3uur")
    };
    assert_eq!(None, mm_i.next());

    let word = split.next()?.to_string();
    assert_eq!(None, split.next());

    return Some(Password { word, cha, min, max });
}

fn is_valid(pass: &Password) -> bool {
    //println!("\t{}", pass.word);
    //println!("\t{}", pass.cha);
    //println!("\t{}", pass.min);
    //println!("\t{}", pass.max);

    let f: String = pass.word.chars().filter(|x| *x == pass.cha).collect();

    return pass.min <= f.len() && f.len() <= pass.max;
}

fn main() -> io::Result<()> {

    let pass_vec = get_pass_vec()?;

    let mut counter: i32 = 0;

    for pass in pass_vec.iter() {
        let parsed = match parse_pass(pass) {
            Some(x) => x,
            None => panic!("lskdhfoisehf")
        };
        if is_valid(&parsed) {
            counter += 1;
        }
    }

    writeln!(io::stdout(), "{} passwords are valid", counter)?;

    Ok(())
}

//writeln!(io::stdout(), "{}", num)?;

