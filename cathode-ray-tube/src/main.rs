use std::{
    io::{stdin, Read},
    num::ParseIntError,
    str::FromStr,
};

enum OperationKind {
    Add,
    Noop,
}

struct Operation {
    kind: OperationKind,
    time_left: usize,
    operand: Option<i32>,
}

impl FromStr for Operation {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Ok(Operation {
                kind: OperationKind::Noop,
                time_left: 1,
                operand: None,
            });
        }
        let mut iter = s.split(" ");
        let kind = iter.next().unwrap();
        if kind == "noop" {
            return Ok(Operation {
                kind: OperationKind::Noop,
                time_left: 1,
                operand: None,
            });
        }
        let operand = iter.next().unwrap().parse()?;
        Ok(Operation {
            kind: OperationKind::Add,
            time_left: 2,
            operand: Some(operand),
        })
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut register = 1;
    let mut time = 1;
    let mut ans = 0;
    let mut ops = input.split("\n").map(|s| s.parse::<Operation>().unwrap());
    let mut current_op = ops.next();
    let mut crt = 0;

    print!("#");
    while current_op.is_some() {
        let mut op = current_op.as_mut().unwrap();
        op.time_left -= 1;
        if op.time_left == 0 {
            if let OperationKind::Add = op.kind {
                register += op.operand.unwrap();
            }
            current_op = ops.next();
        }

        time += 1;
        crt += 1;
        if crt % 40 == 0 {
            println!("");
        }
        if ((crt % 40) - register).abs() <= 1 {
            print!("#");
        } else {
            print!(".");
        }

        if is_interesting(time) {
            ans += register * time;
        }
    }

    println!("{ans}");
}

fn is_interesting(time: i32) -> bool {
    if time <= 20 {
        time == 20
    } else {
        (time - 20) % 40 == 0
    }
}
