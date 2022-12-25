use std::{
    cmp::min,
    collections::VecDeque,
    io::{stdin, Read},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut grid: Vec<Vec<u8>> = input.split("\n").map(|s| Vec::from(s.as_bytes())).collect();

    let n = grid.len();
    let m = grid[0].len();

    let mut src = (0, 0);
    let mut tar = (0, 0);

    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == 'S' as u8 {
                src = (i, j);
            }
            if grid[i][j] == 'E' as u8 {
                tar = (i, j);
            }
        }
    }

    let mut queue = VecDeque::new();
    let di = [-1, 1, 0, 0];
    let dj = [0, 0, -1, 1];
    let mut dist = vec![vec![-1; m]; n];
    dist[src.0][src.1] = 0;
    grid[src.0][src.1] = 'a' as u8;
    grid[tar.0][tar.1] = 'z' as u8;
    println!("Grid dimensions: {}X{}", n, m);
    
    // for part 2
    // queue.push_back(src);
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == 'a' as u8 {
                queue.push_back((i, j));
                dist[i][j] = 0;
            }
        }
    }

    while !queue.is_empty() {
        let cur = queue.pop_front().unwrap();
        for i in 0..4 {
            let ni = cur.0 as i32 + di[i];
            let nj = cur.1 as i32 + dj[i];
            if min(ni, nj) < 0 || ni >= n as i32 || nj >= m as i32 {
                continue;
            }
            let ni = ni as usize;
            let nj = nj as usize;
            if dist[ni][nj] != -1 {
                continue;
            }
            if grid[ni][nj] > grid[cur.0][cur.1] && grid[ni][nj] - grid[cur.0][cur.1] > 1 {
                continue;
            }
            dist[ni][nj] = dist[cur.0][cur.1] + 1;
            queue.push_back((ni, nj));
        }
    }

    println!("{}", dist[tar.0][tar.1]);
}
