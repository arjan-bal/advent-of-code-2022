use std::io::stdin;

fn main() {
    let mut input = String::new();
    let req_distinct = 14;

    stdin()
        .read_line(&mut input)
        .expect("cannot read from stdin");
    let mut in_window = vec![0; 26];
    let mut uniq = 0;

    let add = |val: usize, window: &mut Vec<i32>| -> i32 {
        let val = val - ('a' as usize);
        window[val] += 1;
        if window[val] == 1 {
            1
        } else if window[val] == 2 {
            -1
        } else {
            0
        }
    };

    let rem = |val: usize, window: &mut Vec<i32>| -> i32 {
        let val = val - ('a' as usize);
        window[val] -= 1;
        if window[val] == 1 {
            1
        } else if window[val] == 0 {
            -1
        } else {
            0
        }
    };

    for i in 0..req_distinct - 1 {
        uniq += add(input.as_bytes()[i] as usize, &mut in_window);
    }

    for i in req_distinct - 1..input.len() {
        uniq += add(input.as_bytes()[i] as usize, &mut in_window);
        if uniq as usize >= req_distinct {
            println!("{}", i + 1);
            return;
        }
        uniq += rem(input.as_bytes()[i + 1 - req_distinct] as usize, &mut in_window);
    }
}
