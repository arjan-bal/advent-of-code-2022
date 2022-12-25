use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin()
        .read_to_string(&mut input)
        .expect("Error while reading from stdin");
    let (stacks, operations) = input.split_once("\n\n").unwrap();
    println!("stacks:\n{}\nOperations:\n{}", stacks, operations);

    let mut stack_string: Vec<String> = stacks
        .split("\n")
        .map(|s| {
            let mut x = s.chars();
            x.next();
            x.step_by(4).collect()
        })
        .collect();

    // Discard the last line of stacks as it contains indexes.
    stack_string.pop();
    let mut stacks = vec![Vec::new(); stack_string[0].len()];
    for val in stack_string.iter().rev() {
        for (idx, ch) in val.chars().enumerate() {
            if ch != ' ' {
                stacks[idx].push(ch);
            }
        }
    }
    dbg!(&stacks);

    for op in operations.split("\n") {
        if op.is_empty() {
            break;
        }
        let mut parts = op.split(" ");
        parts.next();
        let qty: usize = parts.next().unwrap().parse().unwrap();
        parts.next();
        let from: usize = parts.next().unwrap().parse::<usize>().unwrap() - 1;
        parts.next();
        let to: usize = parts.next().unwrap().parse::<usize>().unwrap() - 1;
        println!("Transferring {} items from {} to {}", qty, from, to);
        let mut tmp = Vec::new();
        for _ in 0..qty {
            let val = stacks[from].pop().unwrap();
            tmp.push(val);
        }
        stacks[to].extend(tmp.iter().rev());
    }

    for stak in stacks {
        print!("{}", stak.last().unwrap());
    }
    print!("\n");
}
