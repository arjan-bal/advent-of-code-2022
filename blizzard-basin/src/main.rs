use std::{
    collections::{HashSet, VecDeque},
    io::{stdin, Read}, cmp::min,
};

#[derive(Clone, Hash, PartialEq, Eq)]
struct Pos {
    time: usize,
    i: i32,
    j: i32,
    stages: usize,
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let grid: Vec<Vec<_>> = input
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().collect())
        .collect();

    let n = grid.len();
    let m = grid[0].len();
    let mut up_winds = HashSet::new();
    let mut down_winds = HashSet::new();
    let mut right_winds = HashSet::new();
    let mut left_winds = HashSet::new();
    let mut start_j = 0;

    for j in 0..m {
        if grid[0][j] == '.' {
            start_j = j as i32;
            break;
        }
    }

    let start = Pos {
        time: 0,
        i: 0,
        j: start_j,
        stages: 0,
    };

    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == '>' {
                right_winds.insert((i - 1, j - 1));
            } else if grid[i][j] == '<' {
                left_winds.insert((i - 1, j - 1));
            } else if grid[i][j] == '^' {
                up_winds.insert((i - 1, j - 1));
            } else if grid[i][j] == 'v' {
                down_winds.insert((i - 1, j - 1));
            }
        }
    }

    let is_safe = |ci: i32, cj: i32, time: usize| {
        if ci == 0 {
            return true;
        }
        assert_ne!(cj, 0);
        let ci = ci - 1;
        let cj = cj - 1;
        let mod_i = n  as i32 - 2;
        let mod_j = m as i32 - 2;
        let t = time as i32;
        // i + t = ci
        // i = ci - t
        if down_winds.contains(&(
            (((ci - t) % mod_i + mod_i) % mod_i) as usize,
            cj as usize,
        )) {
            return false;
        }

        // i - t = ci
        // i = ci + t
        if up_winds.contains(&(
            ((ci + t) % mod_i) as usize,
            cj as usize,
        )) {
            return  false;
        }

        // j + t = cj
        // j = cj - t
        if right_winds.contains(&(
            ci as usize,
            (((cj - t) % mod_j + mod_j) % mod_j) as usize,
        )) {
            return false;
        }

        // j - t = cj
        // j = cj + t
        if left_winds.contains(&(
            ci as usize,
            ((cj + t) % mod_j) as usize,
        )) {
            return false;
        }
        true
    };

    let mut q = VecDeque::new();
    let mut seen = vec![HashSet::new(); 3];
    let di = [-1, 1, 0, 0, 0];
    let dj = [0, 0, -1, 1, 0];

    q.push_back(start.clone());
    seen[0].insert(start);

    while !q.is_empty() {
        let cur = q.pop_front().unwrap();
        if cur.stages == 3 {
            println!("{}", cur.time);
            break;
        }
        seen[(cur.time + 2) % 3].clear();
        for i in 0..di.len() {
            let ni = cur.i + di[i];
            let nj = cur.j + dj[i];
            if min(ni, nj) < 0 || ni >= n as i32 || nj >= m as i32 || grid[ni as usize][nj as usize] == '#' {
                continue;
            }
            if !is_safe(ni, nj, cur.time + 1) {
                continue;
            }
            let mut nstages = cur.stages;
            if cur.stages % 2 == 0 {
                if ni == n as i32 - 1 {
                    nstages += 1;
                }
            } else if cur.stages == 1 {
                if ni == 0 {
                    nstages += 1;
                }
            }

            let npos = Pos{
                i: ni,
                j: nj,
                time: cur.time + 1,
                stages: nstages,
            };
            if seen[npos.time % 3].contains(&npos) {
                continue;
            }
            seen[npos.time % 3].insert(npos.clone());
            q.push_back(npos);
        }
    }
}
