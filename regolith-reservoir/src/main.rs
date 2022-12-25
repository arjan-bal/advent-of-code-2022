use std::{io::{stdin, Read}, str::FromStr, num::ParseIntError, cmp::max};


#[derive(Clone)]
enum CellType {
    Empty,
    Rock,
}

#[derive(Debug)]
struct Point(usize, usize);

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
    let (first, second) = s.trim().split_once(",").unwrap();
        Ok(Point(first.parse()?, second.parse()?))
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let lines = input.split("\n");
    let n = 2000;
    let m = 2000;
    let mut max_y = 0;

    let mut grid = vec![vec![CellType::Empty; m]; n];

    for line in lines {
        if line.is_empty() {
            break;
        }
        let mut points = line.split("->").map(|s| s.parse::<Point>().unwrap()).peekable();
        let mut prev = points.next().unwrap();
        
        while points.peek().is_some() {
            let cur = points.next().unwrap();
            max_y = max(max_y, cur.1);
            max_y = max(max_y, prev.1);
            draw(&prev, &cur, &mut grid);
            prev = cur;
        }
    }

    for i in 0..n {
        grid[i][max_y + 2] = CellType::Rock;
    }

    dbg!(max_y);

    let mut time = 0;
    
    loop {
        let dest = translate(Point(500, 0), &grid);
        // println!("Destination at time {} is: {:?}", time, &dest);
        // For part 1
        if dest.1 >= m {
            break;
        }
        time += 1;
        // For part 2
        if dest.1 == 0 {
            break;
        }
        grid[dest.0][dest.1] = CellType::Rock;
    }

    println!("{}", time);
}

fn translate(src: Point, grid: &Vec<Vec<CellType>>) -> Point {
    if src.1 + 1 >= grid[0].len() {
        return Point(src.0, src.1 + 1);
    }
    let dx: [i32; 3] = [0, -1, 1];
    for delta in dx {
        let nx = (src.0 as i32 + delta) as usize;
        if let CellType::Empty = grid[nx][src.1 + 1] {
            return translate(Point(nx, src.1 + 1), grid);
        }
    }
    src
}

fn draw(st: &Point, en: &Point, grid: &mut Vec<Vec<CellType>>) {
    if st.0 == en.0 {
        if st.1 > en.1 {
            draw(en, st, grid);
            return;
        }
        for j in st.1..en.1 + 1 {
            grid[st.0][j] = CellType::Rock;
        }
        return;
    }
    if st.1 == en.1 {
        if st.0 > en.0 {
            draw(en, st, grid);
            return;
        }
        for i in st.0..en.0 + 1 {
            grid[i][st.1] = CellType::Rock;
        }
        return;
    }
    panic!("Points are not in axis aligned line");
}
