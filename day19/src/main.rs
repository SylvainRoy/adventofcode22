use std::fs::read_to_string;
use std::collections::HashMap;

enum Robot {
    Ore,
    Clay,
    Obs,
    Geo,
}

#[derive(Debug)]
struct Blueprint {
    id: isize,
    ore_robot_in_ore: isize,
    clay_robot_in_ore: isize,
    obs_robot_in_ore: isize,
    obs_robot_in_clay: isize,
    geo_robot_in_ore: isize,
    geo_robot_in_obs: isize,
}

impl Blueprint {
    fn from(line: &str) -> Self {
        let mut values = line
            .split_whitespace()
            .map(|token| token.replace(':', "").parse::<isize>())
            .filter(|parsed| parsed.is_ok())
            .map(|result| result.unwrap());
        Blueprint {
            id: values.next().unwrap(),
            ore_robot_in_ore: values.next().unwrap(),
            clay_robot_in_ore: values.next().unwrap(),
            obs_robot_in_ore: values.next().unwrap(),
            obs_robot_in_clay: values.next().unwrap(),
            geo_robot_in_ore: values.next().unwrap(),
            geo_robot_in_obs: values.next().unwrap(),
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct State {
    min: isize,
    maxtime: isize,
    ore: isize,
    clay: isize,
    obs: isize,
    geo: isize,
    n_ore_robot: isize,
    n_clay_robot: isize,
    n_obs_robot: isize,
    n_geo_robot: isize,
}

impl State {
    fn new(maxtime: isize) -> Self {
        State {
            min: 0,
            maxtime,
            ore: 0,
            clay: 0,
            obs: 0,
            geo: 0,
            n_ore_robot: 1,
            n_clay_robot: 0,
            n_obs_robot: 0,
            n_geo_robot: 0,
        }
    }

    fn next(&mut self) {
        self.min += 1;
        self.ore += self.n_ore_robot;
        self.clay += self.n_clay_robot;
        self.obs += self.n_obs_robot;
        self.geo += self.n_geo_robot;
    }

    fn buy(&mut self, robot: Robot, blueprint: &Blueprint) {
        loop {
            if self.min == self.maxtime {
                return;
            }
            match robot {
                Robot::Ore => {
                    if self.ore >= blueprint.ore_robot_in_ore {
                        self.ore -= blueprint.ore_robot_in_ore;
                        self.next();
                        self.n_ore_robot += 1;
                        break;
                    }
                }
                Robot::Clay => {
                    if self.ore >= blueprint.clay_robot_in_ore {
                        self.ore -= blueprint.clay_robot_in_ore;
                        self.next();
                        self.n_clay_robot += 1;
                        break;
                    }
                }
                Robot::Obs => {
                    if self.ore >= blueprint.obs_robot_in_ore
                        && self.clay >= blueprint.obs_robot_in_clay
                    {
                        self.ore -= blueprint.obs_robot_in_ore;
                        self.clay -= blueprint.obs_robot_in_clay;
                        self.next();
                        self.n_obs_robot += 1;
                        break;
                    }
                }
                Robot::Geo => {
                    if self.ore >= blueprint.geo_robot_in_ore
                        && self.obs >= blueprint.geo_robot_in_obs
                    {
                        self.ore -= blueprint.geo_robot_in_ore;
                        self.obs -= blueprint.geo_robot_in_obs;
                        self.next();
                        self.n_geo_robot += 1;
                        break;
                    }
                }
            }
            self.next();
        }
    }

    fn explore(&self, blueprint: &Blueprint, cache: &mut HashMap<State, isize>) -> isize {
        if self.min >= self.maxtime {
            return self.geo;
        }
        // if let Some(val) = cache.get(&self) {
        //     // println!("from cache!");
        //     return *val;
        // }
        let mut maxi = 0;
        if self.n_obs_robot > 0 {
            let clone = &mut self.clone();
            clone.buy(Robot::Geo, blueprint);
            maxi = maxi.max(clone.explore(blueprint,    cache));
        }
        if self.n_clay_robot > 0 {
            let clone = &mut self.clone();
            clone.buy(Robot::Obs, blueprint);
            maxi = maxi.max(clone.explore(blueprint, cache));
        }
        let clone = &mut self.clone();
        clone.buy(Robot::Clay, blueprint);
        maxi = maxi.max(clone.explore(blueprint, cache));
        let clone = &mut self.clone();
        clone.buy(Robot::Ore, blueprint);
        maxi = maxi.max(clone.explore(blueprint, cache));
        // cache.insert(self.clone(), maxi);
        maxi
    }
}

fn main() {
    // Read input
    let input = read_to_string("./data/input.txt").unwrap();
    let blueprints: Vec<Blueprint> = input.lines().map(|line| Blueprint::from(line)).collect();

    let mut cache = HashMap::new();
    let state = State::new(24);
    let res: isize = blueprints
        .iter()
        .map(|blueprint| blueprint.id * state.explore(blueprint, &mut cache))
        .sum();
    println!("Part 1: {:?}", res);

    let mut cache = HashMap::new();
    let state = State::new(32);
    let res: isize = blueprints
        .iter()
        .take(3)
        .map(|blueprint| state.explore(blueprint, &mut cache))
        .reduce(|acc, val| acc * val)
        .unwrap();
    println!("Part 2: {:?}", res);
}
