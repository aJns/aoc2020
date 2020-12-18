use std::fmt;
use std::io::{self, Read, Write};
use std::str;

#[derive(Clone, PartialEq)]
enum Op {
    Add,
    Mult,
    None,
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Op::Add => write!(f, "Add"),
            Op::Mult => write!(f, "Mult"),
            Op::None => write!(f, "None"),
        }
    }
}

#[derive(Clone)]
struct CalcFSM {
    acc: i64,
    elem_str: Vec<char>,
    op: Op,
}

impl fmt::Display for CalcFSM {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "acc: {}, op: {}, elem_str: {}",
            self.acc,
            self.op,
            self.elem_str.iter().collect::<String>()
        )
    }
}

impl CalcFSM {
    fn new() -> Self {
        CalcFSM {
            acc: 0,
            elem_str: Vec::new(),
            op: Op::None,
        }
    }

    fn parse_str(&mut self) -> Option<i64> {
        if self.elem_str.len() > 0 {
            let arg = self.elem_str.iter().collect::<String>().parse().unwrap();
            self.elem_str.clear();
            Some(arg)
        } else {
            None
        }
    }

    fn push_op(&mut self, op: Op) {
        if let Some(arg) = self.parse_str() {
            self.push_arg(arg);
        }

        self.op = op;
    }

    fn push_char(&mut self, c: char) {
        self.elem_str.push(c);
    }

    fn push_arg(&mut self, arg: i64) {
        match self.op {
            Op::Add => self.acc += arg,
            Op::Mult => self.acc *= arg,
            Op::None => self.acc = arg,
        }
    }

    fn finish(&mut self) -> i64 {
        if let Some(arg) = self.parse_str() {
            self.push_arg(arg);
        }
        self.acc
    }
}

fn compute_expression(chars: &mut str::Chars, fsm_start: CalcFSM) -> i64 {
    let mut fsm = fsm_start;
    loop {
        let c_next = chars.next();

        match c_next {
            Some(' ') => (),
            Some('+') => fsm.push_op(Op::Add),
            Some('*') => fsm.push_op(Op::Mult),
            Some('(') => fsm.push_arg(compute_expression(chars, CalcFSM::new())),
            Some(')') => return fsm.finish(),
            Some(x) => fsm.push_char(x),
            None => return fsm.finish(),
        };

        // println!("char: {}, fsm: {}", c_next.unwrap(), fsm);
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut results = Vec::new();
    for line in input.lines() {
        let ret = compute_expression(&mut line.chars(), CalcFSM::new());
        println!("{} = {}", line, ret);
        results.push(ret);
    }

    let sum = results.iter().fold(0, |acc, x| acc + x);
    writeln!(io::stdout(), "Sum of expressions: {}", sum)?;

    Ok(())
}
