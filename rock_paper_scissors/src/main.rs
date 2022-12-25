use std::io::{stdin, Read};

fn main() {
    let results = [[3, 0, 6], [6, 3, 0], [0, 6, 3]];
    let score_for_move = [1, 2, 3];
    let score_for_result = [0, 3, 6];

    let find_for_result = |opponent, score| {
        for i in 0..3 {
            if results[i][opponent] == score {
                return i;
            }
        }
        3
    };

    let mut input = String::new();
    stdin()
        .read_to_string(&mut input)
        .expect("Failed to read stdin");

    let res = input
        .split("\n")
        .map(|line| {
            if line.is_empty() {
                return 0;
            }
            let line = line.as_bytes();
            let opponent = (line[0] - 'A' as u8) as usize;
            // let me = (line[2] - 'X' as u8) as usize;
            let result = (line[2] - 'X' as u8) as usize;
            let me = find_for_result(opponent, score_for_result[result]);
            results[me][opponent] + score_for_move[me]
        })
        .reduce(|sum, item| sum + item)
        .unwrap();
    dbg!(res);
}
