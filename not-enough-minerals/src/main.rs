use std::{
    cmp::{max, min},
    collections::HashMap,
    io::{stdin, Read},
    num::ParseIntError,
    str::FromStr,
    sync::mpsc,
    thread,
};

use regex::Regex;

#[derive(Debug, Clone)]
struct Blueprint {
    id: i32,
    ore_robot_cost_ore: i32,
    clay_robot_cost_ore: i32,
    obsidian_robot_cost_ore: i32,
    obsidian_robot_cost_clay: i32,
    geode_robot_cost_ore: i32,
    geode_robot_cost_obsidian: i32,
}

impl FromStr for Blueprint {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (lhs, rhs) = s.split_once(":").unwrap();
        let (_, id) = lhs.split_once(" ").unwrap();
        let id = id.parse()?;
        let mut lines = rhs.split(".");

        let re = Regex::new(r"Each ore robot costs (\d+) ore").unwrap();
        let text = lines.next().unwrap().trim();
        let cap = re.captures(text).unwrap();
        let ore_rob_cost_ore = cap[1].parse()?;

        let re = Regex::new(r"Each clay robot costs (\d+) ore").unwrap();
        let text = lines.next().unwrap().trim();
        let cap = re.captures(text).unwrap();
        let clay_rob_cost_ore = cap[1].parse()?;

        let re = Regex::new(r"Each obsidian robot costs (\d+) ore and (\d+) clay").unwrap();
        let text = lines.next().unwrap().trim();
        let cap = re.captures(text).unwrap();
        let obs_rob_cost_ore = cap[1].parse()?;
        let obs_rob_cost_clay = cap[2].parse()?;

        let re = Regex::new(r"Each geode robot costs (\d+) ore and (\d+) obsidian").unwrap();
        let text = lines.next().unwrap().trim();
        let cap = re.captures(text).unwrap();
        let geode_rob_cst_ore = cap[1].parse()?;
        let geode_rob_cost_obs = cap[2].parse()?;

        Ok(Blueprint {
            id,
            ore_robot_cost_ore: ore_rob_cost_ore,
            clay_robot_cost_ore: clay_rob_cost_ore,
            obsidian_robot_cost_ore: obs_rob_cost_ore,
            obsidian_robot_cost_clay: obs_rob_cost_clay,
            geode_robot_cost_ore: geode_rob_cst_ore,
            geode_robot_cost_obsidian: geode_rob_cost_obs,
        })
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let bps: Vec<Blueprint> = input
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();

    let (tx, rx) = mpsc::channel();
    let n = bps.len();

    for bp in bps {
        let tx_tmp = tx.clone();
        let bpt = bp.clone();
        thread::spawn(move || {
            let qual = get_quality(&bpt);
            tx_tmp.send(qual).unwrap();
        });
    }

    // let mut ans = 0;
    let mut ans = 1;
    for _ in 0..n {
        ans *= rx.recv().unwrap();
    }
    println!("{ans}");
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct Node {
    ore_pm: i32,
    clay_pm: i32,
    obs_pm: i32,
    geo_pm: i32,
    ore: i32,
    clay: i32,
    obs: i32,
    tme: i32,
}

fn get_quality(bp: &Blueprint) -> i32 {
    println!("Running for BP #{}", bp.id);

    let mut cache = HashMap::new();
    let node = Node {
        ore_pm: 1,
        clay_pm: 0,
        obs_pm: 0,
        geo_pm: 0,
        ore: 0,
        clay: 0,
        obs: 0,
        tme: 1,
    };
    let ans = max_pos(node, &mut cache, bp);

    println!("Ans for BP #{}: {}", bp.id, ans);
    // ans * bp.id
    ans
}

fn max_pos(cur: Node, cache: &mut HashMap<Node, i32>, bp: &Blueprint) -> i32 {
    // if cur.tme >= 24 {
    if cur.tme >= 32 {
        return cur.geo_pm;
    }
    // Guessing some constants
    let x = 20;
    let y = 20;

    if let Some(x) = cache.get(&cur) {
        return *x;
    }

    let mut ans = 0;

    for tak_ore in 0..2 {
        if tak_ore * bp.ore_robot_cost_ore > cur.ore || tak_ore + cur.ore_pm > x {
            break;
        }
        let lft_ore_1 = cur.ore - tak_ore * bp.ore_robot_cost_ore;
        for tak_clay in 0..2 {
            if tak_clay * bp.clay_robot_cost_ore > lft_ore_1 || tak_clay + cur.clay_pm > 15 {
                break;
            }
            let lft_ore_2 = lft_ore_1 - tak_clay * bp.clay_robot_cost_ore;
            for tak_obs in 0..2 {
                if tak_obs * bp.obsidian_robot_cost_ore > lft_ore_2
                    || tak_obs * bp.obsidian_robot_cost_clay > cur.clay
                    || tak_obs + cur.obs_pm > 20
                {
                    break;
                }
                let lft_ore_3 = lft_ore_2 - tak_obs * bp.obsidian_robot_cost_ore;
                for tak_geo in 0..2 {
                    if tak_geo * bp.geode_robot_cost_ore > lft_ore_3
                        || tak_geo * bp.geode_robot_cost_obsidian > cur.obs
                        || tak_geo + cur.geo_pm > 15
                    {
                        break;
                    }
                    if tak_ore + tak_clay + tak_obs + tak_geo > 1 {
                        continue;
                    }
                    let mut tmp = cur.clone();

                    // Subtract the resources that are spent in this min to build robots.
                    tmp.ore -= tak_ore * bp.ore_robot_cost_ore
                        + tak_clay * bp.clay_robot_cost_ore
                        + tak_obs * bp.obsidian_robot_cost_ore
                        + tak_geo * bp.geode_robot_cost_ore;
                    tmp.clay -= tak_obs * bp.obsidian_robot_cost_clay;
                    tmp.obs -= tak_geo * bp.geode_robot_cost_obsidian;

                    // Add the robots obtained at the end of this minute.
                    tmp.ore_pm += tak_ore;
                    tmp.clay_pm += tak_clay;
                    tmp.obs_pm += tak_obs;
                    tmp.geo_pm += tak_geo;

                    // Add resources obtained at end of this min.
                    tmp.ore = min(2 * y, cur.ore_pm + tmp.ore);
                    tmp.clay = min(30, cur.clay_pm + tmp.clay);
                    tmp.obs = min(y, cur.obs_pm + tmp.obs);

                    // Increase the min.
                    tmp.tme += 1;
                    ans = max(ans, cur.geo_pm + max_pos(tmp, cache, bp));
                }
            }
        }
    }

    cache.insert(cur.clone(), ans);
    ans
}
