use std::io::{self, Read, Write};
use std::str;

fn parse_parens(chars: &mut str::Chars) -> i64 {
    let mut expr: Vec<char> = Vec::new();

    while let Some(c_next) = chars.next() {
        match c_next {
            '(' => {
                let val = parse_parens(chars);
                expr.append(&mut val.to_string().chars().collect())
            }
            ')' => break,
            x => expr.push(x),
        }
    }
    comp_flat(expr.iter().collect::<String>())
}

fn comp_flat(expr: String) -> i64 {
    let mut mults = Vec::new();
    for mult_str in expr.split("*") {
        let mut acc = 0;
        for add_str in mult_str.split("+") {
            let add: i64 = add_str.trim().parse().unwrap();
            acc += add;
        }
        mults.push(acc);
    }

    mults.iter().fold(1, |acc, x| acc * x)
}

fn compute_expression(expr: String) -> i64 {
    parse_parens(&mut expr.chars())
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut results = Vec::new();
    for line in input.lines() {
        let ret = compute_expression(line.to_string());
        println!("{} = {}", line, ret);
        results.push(ret);
    }

    let sum = results.iter().fold(0, |acc, x| acc + x);
    writeln!(io::stdout(), "Sum of expressions: {}", sum)?;

    Ok(())
}
