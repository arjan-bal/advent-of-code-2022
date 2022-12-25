use std::{
    collections::HashSet,
    io::{stdin, Read},
};

#[derive(PartialEq, Eq, Hash, Clone)]
struct Point(i32, i32);

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut visited = HashSet::new();
    visited.insert(Point(0, 0));

    let mut points = vec![Point(0, 0); 10];

    let lines: Vec<&str> = input.split("\n").collect();

    for line in lines {
        if line.is_empty() {
            break;
        }
        let (dir, steps) = line.split_once(" ").unwrap();
        let steps: usize = steps.parse().unwrap();
        for _ in 0..steps {
            if dir == "U" {
                points[0].0 += 1;
            } else if dir == "D" {
                points[0].0 -= 1;
            } else if dir == "L" {
                points[0].1 -= 1;
            } else if dir == "R" {
                points[0].1 += 1;
            } else {
                panic!("Unexpected direction of movement: {}", dir);
            }
            for i in 1..10 {
                points[i] = translate(&points[i - 1], &points[i]);
            }
            visited.insert(points[9].clone());
        }
    }

    println!("{}", visited.len());
}

fn translate(pos_h: &Point, pos_t: &Point) -> Point {
    let mut pos_t = pos_t.clone();
    let dx = (pos_h.0 - pos_t.0).abs();
    let dy = (pos_h.1 - pos_t.1).abs();

    if dx <= 1 && dy <= 1 {
        return pos_t;
    }

    if pos_h.1 > pos_t.1 {
        pos_t.1 += 1;
    } else if pos_h.1 < pos_t.1 {
        pos_t.1 -= 1;
    }

    if pos_h.0 > pos_t.0 {
        pos_t.0 += 1;
    } else if pos_h.0 < pos_t.0 {
        pos_t.0 -= 1;
    }

    pos_t
}
