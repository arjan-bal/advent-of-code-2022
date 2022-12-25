use std::{
    cmp::{min, Ordering},
    io::{stdin, Read},
    mem,
    num::ParseIntError,
    str::FromStr,
};

#[derive(PartialEq, Debug, Eq, Clone)]
enum ListElement {
    Int(usize),
    List(Vec<ListElement>),
}

impl ListElement {
    fn to_list(&self) -> &Vec<ListElement> {
        if let ListElement::List(x) = self {
            return x;
        }
        panic!("Self is {:?}, can't convert to List", self);
    }
}

impl FromStr for ListElement {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stacks = Vec::new();
        stacks.push(Vec::new());
        let mut iter = s.chars().peekable();

        while iter.peek().is_some() {
            let nxt = iter.next().unwrap();
            match nxt {
                '[' => stacks.push(Vec::new()),
                ']' => {
                    let back = stacks.pop().unwrap();
                    let mut back_2 = stacks.pop().unwrap();
                    back_2.push(ListElement::List(back));
                    stacks.push(back_2);
                }
                ',' => (),
                nxt if nxt.is_ascii_digit() => {
                    let mut number = String::from(nxt);
                    while iter.peek().unwrap().is_ascii_digit() {
                        let digit = iter.next().unwrap();
                        number.push(digit);
                    }
                    let mut back = stacks.pop().unwrap();
                    back.push(ListElement::Int(number.parse()?));
                    stacks.push(back);
                }
                _ => panic!("Unknown character {}!", nxt),
            };
        }
        // dbg!(stacks.len());
        let ret = mem::replace(&mut stacks[0], Vec::new());
        Ok(ListElement::List(ret))
    }
}

impl Ord for ListElement {
    fn cmp(&self, other: &Self) -> Ordering {
        // If both are integers, compare integers
        if let (ListElement::Int(x), ListElement::Int(y)) = (self, other) {
            return x.cmp(y);
        }
        // If other is Int, make it first argument.
        if let ListElement::Int(_) = other {
            return other.cmp(self).reverse();
        }
        // If self is Int, make it a list
        if let ListElement::Int(x) = self {
            return ListElement::List(vec![ListElement::Int(*x)]).cmp(other);
        }

        // Now both have to be lists.
        let l1 = self.to_list();
        let l2 = other.to_list();

        let n = min(l1.len(), l2.len());
        for i in 0..n {
            let res = l1[i].partial_cmp(&l2[i]).unwrap();
            if res != Ordering::Equal {
                return res;
            }
        }

        l1.cmp(l2)
    }
}

impl PartialOrd for ListElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[allow(dead_code)]
fn part_1(input: String) {
    let pairs = input.split("\n\n");
    let mut ans = 0;

    for (idx, pair_str) in pairs.enumerate() {
        let (l1, l2) = pair_str.split_once("\n").unwrap();
        let (l1, l2): (ListElement, ListElement) = (l1.parse().unwrap(), l2.parse().unwrap());

        if l1 < l2 {
            println!("Found index: {}", idx + 1);
            ans += idx + 1;
        }
    }
    println!("{ans}");
}

fn part_2(input: String) {
    let mut ans = 1;
    let special_ele_1 = ListElement::Int(2);
    let special_ele_2 = ListElement::Int(6);
    let mut lists = vec![special_ele_1.clone(), special_ele_2.clone()];
    
    let mut list2: Vec<ListElement> = input
    .split("\n")
    .filter(|s| !s.is_empty())
    .map(|s| s.parse::<ListElement>().unwrap())
    .collect();

    lists.append(&mut list2);
    lists.sort();

    for (idx, list) in lists.iter().enumerate() {
        if list == &special_ele_1 || list == &special_ele_2 {
            println!("Found special element on index: {}", idx + 1);
            ans *= idx + 1;
        }
    }
    println!("{ans}");
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    // part_1(input);
    part_2(input);
}
