use std::{
    cmp::{max, min},
    collections::HashSet,
    io::{stdin, Read}, thread,
};

#[derive(Clone, Debug)]
struct Pt(i64, i64);

#[derive(Clone, Debug)]
struct Range(i64, i64);

#[derive(Debug, Clone)]
struct ManhattanCircle {
    center: Pt,
    radius: i64,
    point_on_radius: Pt,
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let circles: Vec<ManhattanCircle> = input
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| {
            let (lhs, rhs) = s.split_once(":").unwrap();
            let (pt1, pt2) = (get_point(lhs), get_point(rhs));
            ManhattanCircle {
                center: pt1.clone(),
                radius: manhattan_dist(&pt1, &pt2),
                point_on_radius: pt2.clone(),
            }
        })
        .collect();

    // part_1(circles);
    part_2(circles);
}

#[allow(unused)]
fn part_1(circles: Vec<ManhattanCircle>) {
    // let y = 11;
    let y = 2000000;
    let inf = 1000000000;
    let ranges = get_ranges(&circles, y, -inf, inf);
    dbg!(&ranges);
    let mut ans: i64 = ranges.iter().map(|r| r.1 - r.0 + 1).sum();
    let mut seen = HashSet::new();
    // subtract the beacons that are already on the line
    for c in circles {
        if c.point_on_radius.1 == y {
            seen.insert(c.point_on_radius.0);
        }
    }
    dbg!(&seen);
    ans -= seen.len() as i64;
    println!("{}", ans);
}

// Faster but more complicated solution, probably O(|sensors|^2):
// Take line segments that are parallel to sides of a diamond,
// but one unit further from the center.
// The solution point should lie on one such line segments.
fn part_2(circles: Vec<ManhattanCircle>) {
    // let max_val = 20;
    let max_val = 4000000;

    let num_threads = 100;
    let piece = max_val / num_threads;
    let mut handles = Vec::new();

     for t_idx in 0..num_threads {
        let circles_tmp = circles.clone();
        let handle = thread::spawn(move || {
            solve_range(&circles_tmp, t_idx * piece, (t_idx + 1) * piece, max_val);
        });
        handles.push(handle);
     }
    
     for h in handles {
        h.join().unwrap();
     }
}

fn solve_range(circles: &Vec<ManhattanCircle>, st: i64, en: i64, max_val: i64) {
    for y in st..en {
        let ranges = get_ranges(&circles, y, 0, max_val);
        let unavailable: i64 = (&ranges).iter().map(|r| r.1 - r.0 + 1).sum();
        if unavailable == max_val + 1 {
            continue;
        }
        println!("row = {}, ans = {}", y, unavailable);
        let x = match ranges {
            ranges if ranges.len() == 2 => ranges[0].1 + 1,
            ranges if ranges[0].0 > 0 => 1,
            ranges if ranges[0].1 < max_val => max_val,
            _ => panic!("Not a known condition!"),
        };
        let ans = 4000000 * x + y;
        println!("{ans}");
    }
}

fn get_ranges(circles: &Vec<ManhattanCircle>, y: i64, min_val: i64, max_val: i64) -> Vec<Range> {
    let mut ranges = Vec::new();
    for c in circles {
        let slope = if c.center.1 > y { -1 } else { 1 };

        // x - p.0 / y - p.1 = m
        // x = m * (y - p.1) + p.0

        let p = Pt(c.center.0 - c.radius, c.center.1);
        let x_min = slope * (y - p.1) + p.0;

        let p = Pt(c.center.0 + c.radius, c.center.1);
        let slope = -slope;
        let x_max = slope * (y - p.1) + p.0;

        let x_min = max(x_min, min_val);
        let x_max = min(x_max, max_val);

        if x_min > x_max {
            continue;
        }
        // println!("For {:?}, range: '[{}, {}]", &c, x_min, x_max);
        ranges.push(Range(x_min, x_max));
    }
    non_overlapping(ranges)
}

fn non_overlapping(mut ranges: Vec<Range>) -> Vec<Range> {
    if ranges.is_empty() {
        return Vec::new();
    }

    ranges.sort_by(|r1, r2| {
        if r1.0 != r2.0 {
            r1.0.cmp(&r2.0)
        } else {
            r1.1.cmp(&r2.1)
        }
    });

    let mut stak = vec![ranges[0].clone()];

    for r in ranges.iter() {
        let lst = stak.last().unwrap();
        if r.0 > lst.1 {
            stak.push(r.clone());
        } else if r.1 > lst.1 {
            let mut tmp = stak.pop().unwrap();
            tmp.1 = r.1;
            stak.push(tmp);
        }
    }
    stak
}

fn get_point(s: &str) -> Pt {
    let (x, y) = s.split_once(",").unwrap();
    Pt(get_coord(x), get_coord(y))
}

fn get_coord(s: &str) -> i64 {
    let (_, x) = s.trim().split_once("=").unwrap();
    let s: String = x
        .chars()
        .filter(|c| c.is_ascii_digit() || c == &'-')
        .collect();
    s.parse().unwrap()
}

fn manhattan_dist(p1: &Pt, p2: &Pt) -> i64 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}
