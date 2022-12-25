use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    io::{stdin, Read},
};

struct Movement {
    check_deltas: Vec<(i32, i32)>,
    result_delta: (i32, i32),
}

impl Movement {
    fn should_move(&self, cur_pos: &(i32, i32), positions: &HashSet<(i32, i32)>) -> bool {
        for delta in self.check_deltas.iter() {
            if positions.contains(&(cur_pos.0 + delta.0, cur_pos.1 + delta.1)) {
                return false;
            }
        }
        true
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let grid: Vec<Vec<_>> = input
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().collect())
        .collect();

    let mut elves = HashSet::new();
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '#' {
                elves.insert((i as i32, j as i32));
            }
        }
    }

    let movements = vec![
        Movement {
            check_deltas: vec![(-1, -1), (-1, 0), (-1, 1)],
            result_delta: (-1, 0),
        },
        Movement {
            check_deltas: vec![(1, -1), (1, 0), (1, 1)],
            result_delta: (1, 0),
        },
        Movement {
            check_deltas: vec![(-1, -1), (0, -1), (1, -1)],
            result_delta: (0, -1),
        },
        Movement {
            check_deltas: vec![(-1, 1), (0, 1), (1, 1)],
            result_delta: (0, 1),
        },
    ];

    #[allow(unused)]
    let visualise = |elves: &HashSet<(i32, i32)>| {
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if elves.contains(&(i as i32, j as i32)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
        println!("");
    };

    for round in 0..100000 {
        let mut want_to_move = HashMap::new();
        let temp = elves.clone();

        for elve in temp.iter() {
            let mut skip = true;
            for i in -1..2 {
                for j in -1..2 {
                    if i == 0 && j == 0 {
                        continue;
                    }
                    if temp.contains(&(elve.0 + i, elve.1 + j)) {
                        skip = false;
                    }
                }
            }
            if skip {
                continue;
            }
            for i in 0..movements.len() {
                let mv = &movements[(i + round) % movements.len()];
                if !mv.should_move(elve, &elves) {
                    continue;
                }
                let target = (elve.0 + mv.result_delta.0, elve.1 + mv.result_delta.1);
                let mut old_value = want_to_move
                    .insert(target.clone(), Vec::new())
                    .unwrap_or(Vec::new());
                old_value.push(elve.clone());
                want_to_move.insert(target.clone(), old_value);
                break;
            }
        }

        let mut moved = false;
        // Try to move the elves.
        for (dest, visitors) in want_to_move.iter() {
            if visitors.len() != 1 {
                continue;
            }
            moved = true;
            elves.remove(&visitors[0]);
            elves.insert(dest.clone());
        }
        println!("At the end of round #{}", round + 1);
        if !moved {
            break;
        }
        // visualise(&elves);
    }

    // find are of smallest rectangle.
    let inf = 1000000000;
    let mut min_i = inf;
    let mut max_i = -inf;
    let mut min_j = inf;
    let mut max_j = -inf;

    for elve in elves.iter() {
        min_i = min(min_i, elve.0);
        max_i = max(max_i, elve.0);
        min_j = min(min_j, elve.1);
        max_j = max(max_j, elve.1);
    }

    println!(
        "{}",
        (max_i - min_i + 1) * (max_j - min_j + 1) - elves.len() as i32
    );
}
