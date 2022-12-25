use std::{
    cmp::max,
    io::{stdin, Read},
};

struct Node {
    val: usize,
    score: usize,
}

fn main() {
    // solve1();
    solve2();
}

fn solve1() {
    let mut input = String::new();
    stdin()
        .read_to_string(&mut input)
        .expect("Can't read from stdin");

    let mut lines: Vec<&str> = input.split("\n").collect();
    while lines.last().unwrap().is_empty() {
        lines.pop();
    }
    let n = lines.len();
    let m = lines[0].len();
    let mut grid = Vec::new();

    for i in 0..n {
        grid.push(Vec::new());
        for j in 0..m {
            grid[i].push(Node {
                val: lines[i].as_bytes()[j] as usize - '0' as usize,
                score: 0,
            });
        }
    }

    for i in 0..n {
        grid[i][0].score += 1;
        let mut largest = grid[i][0].val;
        for j in 1..m {
            if grid[i][j].val > largest {
                grid[i][j].score += 1;
                largest = grid[i][j].val;
            }
        }
    }

    for i in 0..n {
        grid[i][m - 1].score += 1;
        let mut largest = grid[i][m - 1].val;
        for j in (0..m - 1).rev() {
            if grid[i][j].val > largest {
                grid[i][j].score += 1;
                largest = grid[i][j].val;
            }
        }
    }

    for j in 0..m {
        grid[0][j].score += 1;
        let mut largest = grid[0][j].val;
        for i in 1..n {
            if grid[i][j].val > largest {
                grid[i][j].score += 1;
                largest = grid[i][j].val;
            }
        }
    }

    for j in 0..m {
        grid[n - 1][j].score += 1;
        let mut largest = grid[n - 1][j].val;
        for i in (0..n - 1).rev() {
            if grid[i][j].val > largest {
                grid[i][j].score += 1;
                largest = grid[i][j].val;
            }
        }
    }

    let mut ans = 0;
    for i in 0..n {
        for j in 0..m {
            if grid[i][j].score > 0 {
                ans += 1;
            }
        }
    }

    println!("{ans}");
}

fn solve2() {
    let mut input = String::new();
    stdin()
        .read_to_string(&mut input)
        .expect("Can't read from stdin");

    let mut lines: Vec<&str> = input.split("\n").collect();
    while lines.last().unwrap().is_empty() {
        lines.pop();
    }
    let n = lines.len();
    let m = lines[0].len();
    let mut grid = Vec::new();

    for i in 0..n {
        grid.push(Vec::new());
        for j in 0..m {
            grid[i].push(Node {
                val: lines[i].as_bytes()[j] as usize - '0' as usize,
                score: 1,
            });
        }
    }

    for i in 0..n {
        let mut stak = Vec::new();
        stak.push((100, 0));
        for j in 0..m {
            while grid[i][j].val > stak.last().unwrap().0 {
                stak.pop();
            }
            grid[i][j].score *= j - stak.last().unwrap().1;
            stak.push((grid[i][j].val, j));
        }
    }

    for i in 0..n {
        let mut stak = Vec::new();
        stak.push((100, m - 1));
        for j in (0..m).rev() {
            while grid[i][j].val > stak.last().unwrap().0 {
                stak.pop();
            }
            grid[i][j].score *= stak.last().unwrap().1 - j;
            stak.push((grid[i][j].val, j));
        }
    }

    for j in 0..m {
        let mut stak = Vec::new();
        stak.push((100, 0));
        for i in 0..n {
            while grid[i][j].val > stak.last().unwrap().0 {
                stak.pop();
            }
            grid[i][j].score *= i - stak.last().unwrap().1;
            stak.push((grid[i][j].val, i));
        }
    }

    for j in 0..m {
        let mut stak = Vec::new();
        stak.push((100, n - 1));
        for i in (0..n).rev() {
            while grid[i][j].val > stak.last().unwrap().0 {
                stak.pop();
            }
            grid[i][j].score *= stak.last().unwrap().1 - i;
            stak.push((grid[i][j].val, i));
        }
    }

    let mut ans = 0;

    for i in 0..n {
        for j in 0..m {
            ans = max(ans, grid[i][j].score);
        }
    }
    println!("{ans}");
}
