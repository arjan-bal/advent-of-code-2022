use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let nums: Vec<_> = input.lines().filter(|s| !s.is_empty()).collect();
    let mut coeffs = vec![0];

    for val in nums.iter() {
        if val.len() > coeffs.len() {
            coeffs.resize(val.len(), 0);
        }
        for (idx, digit) in val.chars().rev().enumerate() {
            let numeric = match digit {
                digit if digit.is_ascii_digit() => digit.to_digit(10).unwrap() as i32,
                '-' => -1,
                '=' => -2,
                _ => panic!("Unknown char in number: {}", digit),
            };
            coeffs[idx] += numeric;
        }
    }

    // normalize coeffs.
    for i in 0..coeffs.len() {
        let mut carry = coeffs[i] / 5;
        coeffs[i] %= 5;
        if coeffs[i] > 2 {
            carry += 1;
            coeffs[i] = coeffs[i] - 5;
        } else if coeffs[i] < -2 {
            carry -= 1;
            coeffs[i] = coeffs[i] + 5;
        }
        if carry == 0 {
            continue;
        }
        if i + 1 >= coeffs.len() {
            coeffs.push(0);
        }
        coeffs[i + 1] += carry;
    }

    // print out the num.
    for digit in coeffs.iter().rev() {
        if *digit >= 0 {
            print!("{}", *digit);
        } else if *digit == -1 {
            print!("-");
        } else {
            print!("=");
        }
    }
    println!("");
}
