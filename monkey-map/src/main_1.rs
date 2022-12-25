use std::{
    cmp::max,
    io::{stdin, Read},
};

#[derive(Clone, PartialEq, Eq, Debug)]
struct Pair(usize, usize);

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let (map, path) = input.split_once("\n\n").unwrap();
    let mut map: Vec<Vec<_>> = map.lines().map(|s| s.chars().collect()).collect();
    // fix the map to make implementation simpler.
    let mut max_j = 0;
    for l in map.iter() {
        max_j = max(max_j, l.len());
    }

    for l in map.iter_mut() {
        l.resize(max_j, ' ');
    }

    let path = path.lines().filter(|s| !s.is_empty()).next().unwrap();
    let steps: Vec<usize> = path
        .split(|s: char| s.is_ascii_alphabetic())
        .map(|s| s.parse().unwrap())
        .collect();
    let direction: Vec<_> = path.chars().filter(|s| !s.is_ascii_digit()).collect();

    assert_eq!(steps.len(), direction.len() + 1);

    let di = [0, 1, 0, -1];
    let dj = [1, 0, -1, 0];

    let mut cur_facing = 0;
    let mut cur = Pair(0, 0);
    cur = wrap_around(cur, 0, 1, &map);

    for i in 0..steps.len() {
        // Traverse in this direction.
        for _ in 0..steps[i] {
            let mut nxt = Pair(
                (cur.0 as i32 + map.len() as i32 + di[cur_facing]) as usize % map.len(),
                (cur.1 as i32 + map[0].len() as i32 + dj[cur_facing]) as usize % map[0].len(),
            );
            nxt = wrap_around(nxt, di[cur_facing], dj[cur_facing], &map);
            if nxt == cur || map[nxt.0][nxt.1] != '.' {
                break;
            }
            cur = nxt;
        }
        // println!("{:?}: {}", &cur, &map[cur.0][cur.1]);
        if i + 1 == steps.len() {
            break;
        }
        cur_facing += match direction[i] {
            'R' => 1,
            'L' => (di.len() as i32 - 1) as usize,
            _ => panic!("Unknown direction {}", direction[i]),
        };
        cur_facing %= di.len();
    }

    println!("{}", 1000 * (cur.0 + 1) + 4 * (cur.1 + 1) + cur_facing);
}

fn wrap_around(cur: Pair, di: i32, dj: i32, map: &Vec<Vec<char>>) -> Pair {
    assert_ne!(di.abs() > 0, dj.abs() > 0);
    assert!(max(di.abs(), dj.abs()) == 1);

    // Try to determine the next tile that is non space.
    // If it turns out to be blocked, we don't move.
    let mut nxt = cur.clone();
    while map[nxt.0][nxt.1] == ' ' {
        if di.abs() > 0 {
            nxt.0 = (nxt.0 as i32 + map.len() as i32 + di) as usize % map.len();
        } else {
            nxt.1 = (nxt.1 as i32 + map[0].len() as i32 + dj) as usize % map[0].len();
        }
    }
    if map[nxt.0][nxt.1] == '#' {
        cur
    } else {
        nxt
    }
}
