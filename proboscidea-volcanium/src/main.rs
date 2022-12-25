use regex::Regex;
use std::{
    cmp::max,
    collections::{HashMap, VecDeque},
    io::{stdin, Read},
    num::ParseIntError,
    ops::{BitOr, BitXor},
    str::FromStr,
};

#[derive(Debug, Clone)]
struct Valve {
    name: String,
    flow_rate: usize,
    idx: Option<usize>,
    neighbours: Vec<String>,
    special_idx: Option<usize>,
    neighbour_idxs: Vec<usize>,
}

impl FromStr for Valve {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(
            r"^Valve ([^\s]*) has flow rate=(\d*); tunnel[s]? lead[s]? to valve[s]? (.*)$",
        )
        .unwrap();
        let cap = re.captures(s).unwrap();
        Ok(Valve {
            name: String::from(&cap[1]),
            flow_rate: cap[2].parse()?,
            idx: None,
            special_idx: None,
            neighbour_idxs: Vec::new(),
            neighbours: cap[3]
                .split(",")
                .map(|s| s.trim())
                .map(|s| String::from(s))
                .collect(),
        })
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut nodes: Vec<Valve> = input
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();

    let mut node_index = HashMap::new();

    for (idx, node) in nodes.iter().enumerate() {
        node_index.insert(node.name.clone(), idx);
    }

    let n = nodes.len();
    let mut special_nodes = Vec::new();

    for i in 0..n {
        nodes[i].idx = Some(i);
        if nodes[i].flow_rate > 0 || i == 0 {
            nodes[i].special_idx = Some(special_nodes.len());
            special_nodes.push(i);
        }
        nodes[i].neighbour_idxs = nodes[i]
            .neighbours
            .iter()
            .map(|s| *node_index.get(s).unwrap())
            .collect();
    }

    // Find all pair shortest paths.
    let mut dists = Vec::new();
    for i in 0..n {
        dists.push(bfs(i, &nodes));
    }

    // Run DP to find max cost.
    // dp[idx][mask][time]
    let m = special_nodes.len();
    
    // For part 1
    // let max_time = 30;
    // let ans = best_pos(0, 1, max_time as i32, &special_nodes, &mut dp, &dists, &nodes);
    // println!("{}", ans);
    
    let mut ans = 0;
    
    let max_mask = (1 << m) as usize;
    let max_time = 26;
    let mut dp = vec![vec![vec![-1; max_time + 1]; 1 << m]; m];
    for cmask in 0..max_mask {
        let complement = cmask.bitxor(max_mask - 1);
        ans = max(
            ans,
            best_pos(
                0,
                1.bitor(cmask),
                max_time as i32,
                &special_nodes,
                &mut dp,
                &dists,
                &nodes,
            ) + best_pos(
                0,
                1.bitor(complement),
                max_time as i32,
                &special_nodes,
                &mut dp,
                &dists,
                &nodes,
            ),
        );
    }
    println!("{ans}");
}

fn best_pos(
    sidx: usize,
    mask: usize,
    time_left: i32,
    special_nodes: &Vec<usize>,
    dp: &mut Vec<Vec<Vec<i32>>>,
    dists: &Vec<Vec<i32>>,
    nodes: &Vec<Valve>,
) -> i32 {
    if time_left <= 0 {
        return 0;
    }
    if dp[sidx][mask][time_left as usize] != -1 {
        return dp[sidx][mask][time_left as usize];
    }

    let mut call_again = |sidx: usize, mask: usize, time_left: i32| {
        best_pos(sidx, mask, time_left, special_nodes, dp, dists, nodes)
    };

    let idx = special_nodes[sidx];
    let mut ans = 0;
    for nidx in special_nodes {
        let snidx = nodes[*nidx].special_idx.unwrap();
        if ((mask >> snidx) & 1) > 0 {
            continue;
        }
        if dists[idx][*nidx] == -1 {
            continue;
        }
        ans = max(
            ans,
            call_again(
                snidx,
                mask.bitor(1 << snidx),
                time_left - dists[idx][*nidx] - 1,
            ),
        );
    }

    dp[sidx][mask][time_left as usize] = ans + nodes[idx].flow_rate as i32 * time_left;
    dp[sidx][mask][time_left as usize]
}

fn bfs(src: usize, nodes: &Vec<Valve>) -> Vec<i32> {
    let n = nodes.len();
    let mut dist = vec![-1; n];
    dist[src] = 0;
    let mut q = VecDeque::new();
    q.push_back(src);

    while !q.is_empty() {
        let cur = q.pop_front().unwrap();
        for child in &nodes[cur].neighbour_idxs {
            if dist[*child] != -1 {
                continue;
            }
            dist[*child] = dist[cur] + 1;
            q.push_back(*child);
        }
    }
    dist
}
