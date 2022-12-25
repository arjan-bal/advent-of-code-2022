use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let j_deltas: Vec<i32> = input
        .chars()
        .filter(|c| *c != '\n')
        .map(|c| match c {
            '>' => 1,
            '<' => -1,
            _ => panic!("Unknown char in stream"),
        })
        .collect();

    let shapes = [
        vec!["####"],
        vec![".#.", "###", ".#."],
        vec!["..#", "..#", "###"],
        vec!["#", "#", "#", "#"],
        vec!["##", "##"],
    ];

    let rounds = 200202;
    let period = 1725;
    // let rounds = (1000000000000 % period) + period;
    let mut grid = vec!["#".repeat(9)];
    let mut iter = 0;
    let mut vec = Vec::new();
    let mut solutions = Vec::new();

    for i in 0..rounds {
        iter = play_tetris(&mut grid, &shapes[i % shapes.len()], &j_deltas, iter);
        // println!("{}", get_highest(&grid));
        // for s in grid.iter().rev() {
        //     println!("{}", s);
        // }
        // println!("");
        let idx = get_highest(&grid);
        if grid[idx].find(".") == None {
            vec.push(i);
        }
        solutions.push(idx);
    }

    let target = 1000000000000 - 1;
    let st = (target % period) + period;
    assert_eq!(solutions[st + 5 * period] - solutions[st + 4 * period], solutions[st + 2 * period] - solutions[st + period]);
    let delta_one_cycle = solutions[st + period] - solutions[st];
    let num_cycles = (target - st) / period;
    dbg!(num_cycles, st + num_cycles * period, delta_one_cycle);
    let answer = solutions[st] + delta_one_cycle * num_cycles;
    println!("{}", answer);
    // println!("{}", solutions[target]);

    // for i in 0..vec.len() - 1 {
    //     println!("{} {}", vec[i + 1] - vec[i], vec[i]);
    // }

    // for s in grid.iter().rev() {
    //     println!("{}", s);
    // }
    // println!("");

}

fn get_highest(grid: &Vec<String>) -> usize {
    for i in (0..grid.len()).rev() {
        for j in 1..grid[i].as_bytes().len() - 1 {
            if grid[i].as_bytes()[j] == '#' as u8 {
                return i;
            }
        }
    }
    panic!("Grid doesn't contain # (or the code is incorrect)");
}

fn play_tetris(
    grid: &mut Vec<String>,
    shape: &Vec<&str>,
    j_deltas: &Vec<i32>,
    mut iter: usize,
) -> usize {
    let highest = get_highest(&grid);
    let mut cur_i = highest + 3 + shape.len();
    let mut cur_j = 3;

    while grid.len() <= cur_i {
        grid.push("#".to_owned() + &".".repeat(7) + "#");
    }

    let is_ok = |cur_i: usize, cur_j| {
        for i in 0..shape.len() {
            for j in 0..shape[i].len() {
                if shape[i].as_bytes()[j] == '#' as u8
                    && grid[cur_i - i].as_bytes()[cur_j as usize + j] == '#' as u8
                {
                    return false;
                }
            }
        }
        true
    };

    loop {
        // move due to jet
        let dj = j_deltas[iter];
        cur_j += dj;
        if !is_ok(cur_i, cur_j) {
            cur_j -= dj
        }

        iter = (iter + 1) % j_deltas.len();

        // move one step down
        cur_i -= 1;
        if !is_ok(cur_i, cur_j) {
            cur_i += 1;
            break;
        }
    }

    // draw the shape onto the grid.
    for i in 0..shape.len() {
        for j in 0..shape[i].len() {
            if grid[cur_i - i].as_bytes()[cur_j as usize + j] == '#' as u8 && shape[i].as_bytes()[j] == '#' as u8 {
                panic!("Overwriting an existing rock!");
            }
            if shape[i].as_bytes()[j] == '.' as u8 {
                continue
            }
            let range = (cur_j as usize + j)..(cur_j as usize + j + 1);
            let replacement: String = shape[i].chars().skip(j).take(1).collect();
            grid[cur_i - i].replace_range(range, &replacement);
        }
    }

    iter
}
