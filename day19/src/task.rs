use std::ops::{Add, Sub};
use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum Error {
    #[error("Internal Error")]
    #[allow(unused)]
    Internal,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
struct Resource {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

impl Add for Resource {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output{ore: self.ore + rhs.ore, clay: self.clay + rhs.clay, obsidian: self.obsidian + rhs.obsidian, geode: self.geode + rhs.geode}
    }
}

impl Sub for Resource {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output{ore: self.ore - rhs.ore, clay: self.clay - rhs.clay, obsidian: self.obsidian - rhs.obsidian, geode: self.geode - rhs.geode}
    }
}

impl Resource {
    fn new() -> Self {
        Default::default()
    }
    fn ge(&self, rhs: &Self) -> bool {
        self.ore >= rhs.ore && self.clay >= rhs.clay && self.obsidian >= rhs.obsidian
    }
}

impl From<(u32, u32, u32, u32)> for Resource {
    fn from(tuple: (u32, u32, u32, u32)) -> Self {
        Self {ore: tuple.0,  clay: tuple.1, obsidian: tuple.2, geode: tuple.3}
    }
}

fn parse_schema(line: &str) -> Result<Vec<Resource>, Error> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)").unwrap();
    }
    let grep = RE.captures_iter(line).map(|cap| cap[1].parse::<u32>().unwrap()).collect::<Vec<_>>();
    Ok(vec![
        Resource::from((grep[1], 0, 0, 0)),
        Resource::from((grep[2], 0, 0, 0,)),
        Resource::from((grep[3], grep[4], 0, 0)),
        Resource::from((grep[5], 0, grep[6], 0)),
    ])
}

fn dfs(step: u32, costs: &[Resource], storage: Resource, robots: Resource, max_needed: &Resource
    , cache: &mut HashMap<(u32, Resource, Resource), u32>) -> u32
{
    if step == 0 {
        return storage.geode;
    }
    let key = (step, storage, robots);
    if let Some(cache_val) = cache.get(&key) {
        return *cache_val;
    }
    let mut max_geode = 0;
    for i in (0..4).rev() {
        if storage.ge(&costs[i]) {
            match i {
                0 => if max_needed.ore <= robots.ore { continue; },
                1 => if max_needed.clay <= robots.clay { continue; },
                2 => if max_needed.obsidian <= robots.obsidian { continue; },
                _ => {},
            }
            let mut new_robots = robots;
            match i {
                0 => new_robots.ore += 1,
                1 => new_robots.clay += 1,
                2 => new_robots.obsidian += 1,
                3 => new_robots.geode += 1,
                _ => unreachable!()
            }
            max_geode = max_geode.max(dfs(step - 1, costs, storage - costs[i] + robots, new_robots, max_needed, cache));
        }
    }
    max_geode = max_geode.max(dfs(step - 1, costs, storage + robots, robots, max_needed, cache));
    cache.insert(key, max_geode);
    max_geode
}

fn calculate(costs: &[Resource], n: u32) -> u32 {
    let robots = Resource{ore: 1, clay: 0, obsidian: 0, geode: 0};
    let storage = Resource{ore: 0, clay: 0, obsidian: 0, geode: 0};
    let max_needed = costs.iter().fold(Resource::new(), |r, cost|
        Resource{ ore: r.ore.max(cost.ore), clay: r.clay.max(cost.clay), obsidian: r.obsidian.max(cost.obsidian), geode: u32::MAX}
    );
    let res = dfs(n, &costs, storage, robots, &max_needed, &mut HashMap::new());
    res
}

pub fn task1<S: AsRef<str>>(lines: &[S]) -> Result<u32, Error> {
    let costs = lines.iter().map(|l| parse_schema(l.as_ref()).unwrap()).collect::<Vec<_>>();
    Ok(costs.iter().enumerate().fold(0, |total, (i, cost)| total + calculate(cost, 24) * (i as u32 + 1)))
}

pub fn task2<S: AsRef<str>>(lines: &[S]) -> Result<u32, Error> {
    let costs = lines.iter().map(|l| parse_schema(l.as_ref()).unwrap()).collect::<Vec<_>>();
    Ok(costs.iter().take(3).map(|cost| calculate(cost,32)).product())
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &[&str] = &[
        "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.",
        "Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian."
    ];

    #[test]
    fn test1() {
        assert_eq!(Ok(33), task1(&DATA));
    }

    #[test]
    fn test2() {
        assert_eq!(Ok(62), task2(&DATA));
    }
}
