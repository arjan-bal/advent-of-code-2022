use std::io::{stdin, Read};

#[derive(Debug)]
struct Element {
    original_idx: usize,
    value: i64,
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let k = 811589153;

    let mut nums: Vec<_> = input
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .enumerate()
        .map(|(idx, f): (_, i64)| Element {
            original_idx: idx,
            value: f * k,
        })
        .collect();

    let n = nums.len();

    for _ in 0..10 {
        for idx in 0..n {
            let mut cur_idx = nums
                .iter()
                .enumerate()
                .find(|(_, v)| v.original_idx == idx)
                .unwrap()
                .0;
            let moves = &nums[cur_idx].value % (n as i64 - 1);

            for _ in 0..moves.abs() {
                if moves < 0 {
                    nums.swap(cur_idx, (cur_idx + n - 1) % n);
                    cur_idx = (cur_idx + n - 1) % n;
                } else {
                    nums.swap(cur_idx, (cur_idx + 1) % n);
                    cur_idx = (cur_idx + 1) % n;
                }
            }

        }
        // let tmp: Vec<_> = nums.iter().map(|v| v.value).collect();
        // println!("{:?}", tmp);
    }

    let special = [1000, 2000, 3000];
    let idx_0 = nums
        .iter()
        .enumerate()
        .find(|(_, v)| v.value == 0)
        .unwrap()
        .0;
    let mut ans = 0;
    for del in special {
        let final_idx = (del + idx_0) % n;
        ans += nums[final_idx].value;
    }

    println!("{ans}");
}
