use std::{
    cmp::{max, min},
    collections::{HashSet, VecDeque},
    io::{stdin, Read},
    num::ParseIntError,
    str::FromStr,
};

#[derive(Hash, Clone, PartialEq, Eq, Debug)]
struct Pt(i32, i32, i32);

impl FromStr for Pt {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cords = s.split(",");
        Ok(Pt(
            cords.next().unwrap().parse()?,
            cords.next().unwrap().parse()?,
            cords.next().unwrap().parse()?,
        ))
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let pts: Vec<Pt> = input
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();

    let mut store = HashSet::new();
    let mut mx = 30;
    let mut mn = -5;
    for pt in &pts {
        store.insert(pt.clone());
        mx = max(mx, pt.0);
        mx = max(mx, pt.1);
        mx = max(mx, pt.2);
        mn = min(mn, pt.0);
        mn = min(mn, pt.1);
        mn = min(mn, pt.2);
    }

    dbg!(mx, mn);

    let di = [-1, 1, 0, 0, 0, 0];
    let dj = [0, 0, -1, 1, 0, 0];
    let dk = [0, 0, 0, 0, 1, -1];

    let mut ans = 0;

    // For part 1
    // for pt in &pts {
    //     let mut area = 0;
    //     for i in 0..6 {
    //         let pt2 = Pt(pt.0 + di[i], pt.1 + dj[i], pt.2 + dk[i]);
    //         if !store.contains(&pt2) {
    //             area += 1;
    //         }
    //     }
    //     ans += area;
    // }

    let mut q = VecDeque::new();
    // Could use a 3d vector here.
    let mut seen = HashSet::new();
    q.push_back(Pt(mn, mn, mn));
    seen.insert(Pt(mn, mn, mn));

    let is_ok = |x: i32| {
        x >= mn && x <= mx
    };

    while !q.is_empty() {
        let cur = q.pop_front().unwrap();
        for i in 0..6 {
            let pt2 = Pt(cur.0 + di[i], cur.1 + dj[i], cur.2 + dk[i]);
            if !is_ok(pt2.0) || !is_ok(pt2.1) || !is_ok(pt2.2) {
                continue;
            }
            if store.contains(&pt2) {
                ans += 1;
                continue;
            }
            if seen.contains(&pt2) {
                continue;
            }
            seen.insert(pt2.clone());
            q.push_back(pt2);
        }
    }

    println!("{}", ans);
}
