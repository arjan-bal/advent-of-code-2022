use std::{
    cmp::max,
    io::{stdin, Read},
};

#[derive(Clone, PartialEq, Eq, Debug)]
struct Pair(usize, usize);

#[derive(Clone, PartialEq, Eq, Debug)]
struct PairInt(i32, i32);

// Represents the 2
#[derive(Debug)]
struct Position {
    rect_no: usize,
    coords: Pair,
    facing: usize,
}

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
    let n = map.len();
    let m = map[0].len();
    dbg!(n, m);
    assert!(n % 4 == 0);
    assert!(m % 3 == 0);
    assert_eq!(n / 4, m / 3);
    let side = n / 4;

    // let rects_top_left = vec![
    //     Pair(0, 2 * side),
    //     Pair(side, 0),
    //     Pair(side, side),
    //     Pair(side, 2 * side),
    //     Pair(2 * side, 2 * side),
    //     Pair(2 * side, 3 * side),
    // ];
    let rects_top_left = vec![
        Pair(0, side),
        Pair(0, 2 * side),
        Pair(side, side),
        Pair(2 * side, 0),
        Pair(2 * side, side),
        Pair(3 * side, 0),
    ];

    let get_2d_coords = |pos: &Position| {
        let top_left = &rects_top_left[pos.rect_no];
        Pair(top_left.0 + pos.coords.0, top_left.1 + pos.coords.1)
    };

    let get_char_at_pos = |pos: &Position| {
        let dim_2_pos = get_2d_coords(pos);
        map[dim_2_pos.0][dim_2_pos.1]
    };

    // Neighbours
    //     [R,D,L,U]
    // 1:  [2,3,4,6]
    //     [+,+,-,+]
    // 2:  [5,3,1,6]
    //     [-,+,+,+]
    // 3:  [2,5,4,1]
    //     [+,+,+,+]
    // 4:  [5,6,1,3]
    //     [+,+,-,+]
    // 5:  [2,6,4,3]
    //     [-,+,+,+]
    // 6:  [5,2,1,4]
    //     [+,+,+,+]

    // let neighbours = [
    //     //  [R, D, L, U]
    //     [5, 3, 2, 1],
    //     [2, 4, 5, 0],
    //     [3, 4, 1, 0],
    //     [5, 4, 2, 0],
    //     [5, 1, 2, 3],
    //     [0, 1, 4, 3],
    // ];

    // let reflect_on_transition = [
    //     //  [R, D, L, U]
    //     [false, true, true, false],
    //     [true, false, false, false],
    //     [true, false, true, true],
    //     [false, true, true, true],
    //     [true, false, false, true],
    //     [false, false, true, false],
    // ];

    let neighbours = [
        //  [R, D, L, U]
        [1, 2, 3, 5],
        [4, 2, 0, 5],
        [1, 4, 3, 0],
        [4, 5, 0, 2],
        [1, 5, 3, 2],
        [4, 1, 0, 3],
    ];

    let reflect_on_transition = [
        //  [R, D, L, U]
        [true, true, false, true],
        [false, true, true, true],
        [true, true, true, true],
        [true, true, false, true],
        [false, true, true, true],
        [true, true, true, true],
    ];

    let next_hop = |pos: &Position| {
        let coords_int = PairInt(pos.coords.0 as i32, pos.coords.1 as i32);
        let nxt_pos = PairInt(coords_int.0 + di[pos.facing], coords_int.1 + dj[pos.facing]);
        // check if you're about to fall off the side.
        let neighbour_idx = match nxt_pos {
            PairInt(_, j) if j >= side as i32 => Some(0),
            PairInt(i, _) if i >= side as i32 => Some(1),
            PairInt(_, j) if j < 0 => Some(2),
            PairInt(i, _) if i < 0 => Some(3),
            _ => None,
        };

        if neighbour_idx == None {
            return Position {
                coords: Pair(nxt_pos.0 as usize, nxt_pos.1 as usize),
                ..*pos
            };
        }

        let neighbour_idx = neighbour_idx.unwrap();
        let nxt_rect = neighbours[pos.rect_no][neighbour_idx];

        // find which edge you'll enter from.
        let mut entry_edge_idx = 0;
        for idx in 0..neighbours[0].len() {
            if neighbours[nxt_rect][idx] == pos.rect_no {
                entry_edge_idx = idx;
                break;
            }
        }

        let mut nxt_coords = Pair(0, 0);
        let mut nxt_facing = 0;
        let mut oth_coord = 0;

        // dbg!(entry_edge_idx);
        // Entry from right
        if entry_edge_idx == 0 {
            nxt_coords.1 = side - 1;
            oth_coord = 0;
            nxt_facing = 2;
        } else if entry_edge_idx == 1 {
            // Down
            nxt_coords.0 = side - 1;
            oth_coord = 1;
            nxt_facing = 3;
        } else if entry_edge_idx == 2 {
            // Left
            nxt_coords.1 = 0;
            oth_coord = 0;
            nxt_facing = 0;
        } else if entry_edge_idx == 3 {
            // Up
            nxt_coords.0 = 0;
            oth_coord = 1;
            nxt_facing = 1;
        }

        let mut other_coord_val = match neighbour_idx {
            0 | 2 => pos.coords.0,
            1 | 3 => pos.coords.1,
            _ => panic!("Unknown value of neighbour: {}!", neighbour_idx),
        };

        // dbg!(other_coord_val);
        // dbg!(oth_coord);

        if reflect_on_transition[pos.rect_no][neighbour_idx] == false {
            other_coord_val = side - other_coord_val - 1;
        }

        if oth_coord == 0 {
            nxt_coords.0 = other_coord_val;
        } else {
            nxt_coords.1 = other_coord_val;
        }

        Position {
            rect_no: nxt_rect,
            coords: nxt_coords,
            facing: nxt_facing,
        }
    };
    // dbg!(next_hop(&Position{
    //     coords: Pair(side - 1, 2),
    //     rect_no: 0,
    //     facing: 1,
    // }));
    // return;

    let mut cur_pos = Position {
        rect_no: 0,
        coords: Pair(0, 0),
        facing: 0,
    };

    for i in 0..steps.len() {
        // Traverse in this direction.
        for _ in 0..steps[i] {
            let nxt_pos = next_hop(&cur_pos);
            let ch = get_char_at_pos(&nxt_pos);
            assert_ne!(ch, ' ');
            if ch == '#' {
                break;
            }
            cur_pos = nxt_pos;
        }

        println!("pos: {:?}", get_2d_coords(&cur_pos));
        if i + 1 == steps.len() {
            break;
        }
        cur_pos.facing = match direction[i] {
            'R' => (cur_pos.facing + 1) % 4,
            'L' => (cur_pos.facing + 4 - 1) % 4,
            _ => panic!("Unknown direction: {}!", direction[i]),
        };
    }

    let dim_2_pos = get_2d_coords(&cur_pos);
    println!(
        "{}",
        1000 * (dim_2_pos.0 + 1) + 4 * (dim_2_pos.1 + 1) + cur_pos.facing
    );
}
