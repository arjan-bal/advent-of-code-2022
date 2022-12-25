use std::{
    io::{stdin, Read},
    num::ParseIntError,
    str::FromStr,
};

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    arg1: String,
    arg2: String,
    operator: String,
    divisor_for_test: usize,
    pass_on_true: usize,
    pass_on_false: usize,
    inspected: usize,
}

impl FromStr for Monkey {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split("\n");
        // First line is "Monkey <idx>"
        iter.next();

        // Second line is "Starting items: 79, 98"
        let (_, item_list) = iter.next().unwrap().split_once(":").unwrap();
        let items: Vec<usize> = item_list
            .split(",")
            .map(|s| s.trim().parse().unwrap())
            .collect();

        // Third line is "Operation: new = old + 6"
        let (_, operation) = iter.next().unwrap().split_once("=").unwrap();
        let mut operation_iter = operation.trim().split(" ");
        let arg1 = String::from(operation_iter.next().unwrap());
        let operator = String::from(operation_iter.next().unwrap());
        let arg2 = String::from(operation_iter.next().unwrap());

        // Fourth line is "Test: divisible by 19"
        let (_, divisor_for_test) = iter.next().unwrap().split_once("by").unwrap();
        let divisor_for_test = divisor_for_test.trim().parse::<usize>()?;

        // Fifth line is "If true: throw to monkey 2"
        let (_, pass_on_true) = iter.next().unwrap().split_once("monkey").unwrap();
        let pass_on_true = pass_on_true.trim().parse::<usize>()?;

        // Sixth line is "If false: throw to monkey 2"
        let (_, pass_on_false) = iter.next().unwrap().split_once("monkey").unwrap();
        let pass_on_false = pass_on_false.trim().parse::<usize>()?;

        Ok(Monkey {
            items,
            arg1,
            arg2,
            operator,
            divisor_for_test,
            pass_on_true,
            pass_on_false,
            inspected: 0,
        })
    }
}

impl Monkey {
    fn get_arg(arg_str: &String, old_val: usize) -> usize {
        if arg_str == "old" {
            old_val
        } else {
            arg_str.parse().unwrap()
        }
    }

    fn get_result(arg1: usize, arg2: usize, operator: &String) -> usize {
        if operator == "+" {
            arg1 + arg2
        } else {
            arg1 * arg2
        }
    }

    fn process_items(&mut self, modulo: usize) -> Vec<(usize, usize)> {
        let mut transfers = Vec::new();

        for item in &self.items {
            self.inspected += 1;
            let res = Monkey::get_result(
                Monkey::get_arg(&self.arg1, *item),
                Monkey::get_arg(&self.arg2, *item),
                &self.operator,
            ) % modulo;
            // let res = res / 3;
            if res % self.divisor_for_test == 0 {
                transfers.push((self.pass_on_true, res));
            } else {
                transfers.push((self.pass_on_false, res));
            }
        }

        self.items.clear();
        transfers
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|s| s.parse::<Monkey>().unwrap())
        .collect();
    dbg!(&monkeys);
    let mut prod = 1;
    for monkey in &monkeys {
        prod *= monkey.divisor_for_test;
    }

    for round in 0..10000 {
        println!("Starting round #{}", round);

        for i in 0..(&monkeys).len() {
            let transfers = monkeys[i].process_items(prod);
            for (to, val) in transfers {
                monkeys[to].items.push(val);
            }
        }
    }

    monkeys.sort_by_key(|m| m.inspected);
    monkeys.reverse();

    println!("{}", monkeys[0].inspected * monkeys[1].inspected);
}
