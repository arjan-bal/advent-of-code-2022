use std::io::{stdin, Read};

fn main() {
    // solve_tc_1();
    solve_tc_2();
}

fn solve_tc_1() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).expect("Failed read");
    let sum: usize = input.split("\n").map(|s| calc_priority(s)).sum();
    println!("{sum}")
}

fn solve_tc_2() {
    let mut input = String::new();
    stdin()
        .read_to_string(&mut input)
        .expect("Failed to read from stdin");
    let mut iter = input.split("\n").peekable();
    let mut ans = 0;
    while iter.peek().is_some() {
        if iter.peek().unwrap().len() == 0 {
            break;
        }
        ans += find_common(&vec![
            iter.next().unwrap().as_bytes(),
            iter.next().unwrap().as_bytes(),
            iter.next().unwrap().as_bytes(),
        ]);
    }
    println!("{ans}");
}

fn get_code(c: u8) -> usize {
    if c >= 'a' as u8 && c <= 'z' as u8 {
        (c as u8 - 'a' as u8 + 1).into()
    } else {
        (c as u8 - 'A' as u8 + 27).into()
    }
}

fn find_common(vals: &Vec<&[u8]>) -> usize {
    let mut count = vec![0; 53];
    for s in vals {
        let mut seen = vec![false; 53];
        for &c in s.iter() {
            seen[get_code(c) as usize] = true;
        }
        for i in 0..53 {
            if seen[i] {
                count[i] += 1;
            }
        }
    }

    for i in 0..count.len() {
        if count[i] == vals.len() {
            return i;
        }
    }

    return 0;
}

fn calc_priority(s: &str) -> usize {
    let n = s.as_bytes().len();
    find_common(&vec![&s.as_bytes()[0..n / 2], &s.as_bytes()[n / 2..n]])
}
