use std::{
    collections::HashMap,
    io::{stdin, Read},
};

#[derive(Debug)]
struct Node {
    name: String,
    children: Vec<String>,
    op: Option<char>,
    value: Option<i64>,
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut monkeys = HashMap::new();

    let lines = input.lines().filter(|s| !s.is_empty());
    let mut edges = 0;

    for line in lines {
        let (name, rhs) = line.split_once(":").unwrap();
        if !monkeys.contains_key(name) {
            monkeys.insert(
                name.to_owned(),
                Node {
                    name: name.to_owned(),
                    children: Vec::new(),
                    op: None,
                    value: None,
                },
            );
        }
        if name == "humn" {
            continue;
        }
        let rhs: Vec<_> = rhs.trim().split_whitespace().collect();
        if rhs.len() == 1 {
            monkeys.get_mut(name).unwrap().value = Some(rhs[0].parse().unwrap());
            continue;
        }
        edges += 2;
        monkeys.get_mut(name).unwrap().children = vec![rhs[0].to_owned(), rhs[2].to_owned()];
        monkeys.get_mut(name).unwrap().op = Some(rhs[1].chars().next().unwrap());
    }

    assert_eq!(edges + 1, monkeys.len());
    let lhs = dfs(
        monkeys.get("root").unwrap().children[0].clone(),
        &mut monkeys,
    );
    let rhs = dfs(
        monkeys.get("root").unwrap().children[1].clone(),
        &mut monkeys,
    );

    // Only one of lhs and rhs will be None as we have a tree.
    if lhs == None {
        solve_for_x(
            monkeys.get("root").unwrap().children[0].to_owned(),
            rhs.unwrap(),
            &mut monkeys,
        );
    } else {
        solve_for_x(
            monkeys.get("root").unwrap().children[1].to_owned(),
            lhs.unwrap(),
            &mut monkeys,
        );
    }
}

fn dfs(cur: String, nodes: &mut HashMap<String, Node>) -> Option<i64> {
    let me = nodes.get(&cur).unwrap();
    if let Some(x) = me.value {
        return Some(x);
    }
    if me.name == "humn" {
        return me.value;
    }
    let mut vals = Vec::new();
    let mut solvable = true;
    for child in &me.children.clone() {
        let val = dfs(child.to_owned(), nodes);
        if val == None {
            solvable = false;
            continue;
        }
        vals.push(val.unwrap());
    }

    if !solvable {
        return None;
    }

    let me = nodes.get_mut(&cur).unwrap();
    let res = match me.op {
        Some('+') => vals[0] + vals[1],
        Some('*') => vals[0] * vals[1],
        Some('-') => vals[0] - vals[1],
        Some('/') => vals[0] / vals[1],
        _ => panic!("Unknown operator {:?}", me.op),
    };

    me.value = Some(res);
    // dbg!(me);
    Some(res)
}

fn solve_for_x(cur: String, result: i64, nodes: &mut HashMap<String, Node>) {
    let me = nodes.get(&cur).unwrap();
    if let Some(_) = me.value {
        panic!("Should nave reached {:?}", me);
    }
    if me.name == "humn" {
        println!("{}", result);
        return;
    }

    let lhs = nodes.get(&me.children[0]).unwrap();
    let rhs = nodes.get(&me.children[1]).unwrap();

    if lhs.value == None {
        let other = rhs.value.unwrap();
        let res = match me.op {
            Some('+') => result - other,
            Some('*') => result / other,
            Some('-') => result + other,
            Some('/') => result * other,
            _ => panic!("Unknown operator {:?}", me.op),
        };
        solve_for_x(lhs.name.clone(), res, nodes);
        return;
    }

    let me = nodes.get(&cur).unwrap();
    let lhs = nodes.get(&me.children[0]).unwrap();
    let other = lhs.value.unwrap();

    let res = match me.op {
        Some('+') => result - other,
        Some('*') => result / other,
        Some('-') => other - result,
        Some('/') => other / result,
        _ => panic!("Unknown operator {:?}", me.op),
    };
    solve_for_x(rhs.name.clone(), res, nodes);
}
