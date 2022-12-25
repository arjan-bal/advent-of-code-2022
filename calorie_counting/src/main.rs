/*
    Author: Arjan Singh Bal
    "Everything in this world is magic, except to the magician"
*/

use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).expect("Failed read");
    let mut pieces: Vec<u32> = input
        .split("\n\n")
        .map(|piece| {
            let sum = piece
                .split("\n")
                .map(|v| {
                    if v.is_empty() {
                        0
                    } else {
                        v.parse::<u32>().unwrap()
                    }
                })
                .reduce(|sum, item| sum + item)
                .unwrap();
            sum
        })
        .collect();

    pieces.sort();
    pieces.reverse();

    dbg!(pieces[0] + pieces[1] + pieces[2]);
}
