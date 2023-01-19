#![feature(test)]

// use std::{fs::File, io::BufWriter, io::Write};

use nom::{
    bytes::complete::tag,
    character::complete as cc,
    combinator::{all_consuming, map},
    sequence::{preceded, tuple},
    Finish, IResult,
};

const MINUTES: i32 = 24;

#[derive(Debug)]
struct Blueprint {
    ore_cost: i32,
    clay_cost: i32,
    // (ore, clay)
    obsidian_costs: (i32, i32),
    // (ore, obsidian)
    geode_costs: (i32, i32),

    max_ore: i32,
}

impl Blueprint {
    fn parse(i: &str) -> IResult<&str, Self> {
        map(
            tuple((
                preceded(tag("Blueprint "), cc::i32),
                preceded(tag(": Each ore robot costs "), cc::i32),
                preceded(tag(" ore. Each clay robot costs "), cc::i32),
                tuple((
                    preceded(tag(" ore. Each obsidian robot costs "), cc::i32),
                    preceded(tag(" ore and "), cc::i32),
                )),
                tuple((
                    preceded(tag(" clay. Each geode robot costs "), cc::i32),
                    preceded(tag(" ore and "), cc::i32),
                )),
                tag(" obsidian."),
            )),
            |(_, ore_cost, clay_cost, obsidian_costs, geode_costs, _)| Self {
                ore_cost,
                clay_cost,
                obsidian_costs,
                geode_costs,
                max_ore: [ore_cost, clay_cost, obsidian_costs.0, geode_costs.0]
                    .iter()
                    .max()
                    .copied()
                    .unwrap(),
            },
        )(i)
    }
}

#[derive(
    Default,
    Clone,
    Copy,
    derive_more::Add,
    derive_more::AddAssign,
    derive_more::Mul,
    derive_more::Neg,
)]
struct Elements {
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
}

#[derive(Default)]
struct State {
    global_max: i32,
}

impl State {
    fn get_total(
        &mut self,
        bp: &Blueprint,
        inventory: Elements,
        rates: Elements,
        mut minutes_left: i32,
    ) -> i32 {
        let mut geode_rate = rates.geode;
        let mut upper_limit = inventory.geode;
        for _ in 0..minutes_left {
            upper_limit += geode_rate;
            geode_rate += 1;
        }
        if upper_limit < self.global_max {
            return 0;
        }

        if minutes_left == 0 {
            self.global_max = self.global_max.max(inventory.geode);
            return self.global_max;
        }

        minutes_left -= 1;

        let mut total = 0;

        // Geode robot
        let (geode_ore, geode_obsidian) = bp.geode_costs;
        if inventory.ore >= geode_ore && inventory.obsidian >= geode_obsidian {
            // Buying geode-collecting
            let mut new_inv = inventory + rates;
            let mut new_rates = rates;
            new_inv.ore -= geode_ore;
            new_inv.obsidian -= geode_obsidian;
            new_rates.geode += 1;

            total = total.max(self.get_total(bp, new_inv, new_rates, minutes_left));
        }
        if rates.ore >= geode_ore && rates.obsidian >= geode_obsidian {
            return total;
        }

        // Obsidian robot
        let (obsidian_ore, obsidian_clay) = bp.obsidian_costs;
        if rates.obsidian < geode_obsidian
            && inventory.ore >= obsidian_ore
            && inventory.clay >= obsidian_clay
        {
            // Buying obsidian-collecting
            let mut new_inv = inventory + rates;
            let mut new_rates = rates;
            new_inv.ore -= obsidian_ore;
            new_inv.clay -= obsidian_clay;
            new_rates.obsidian += 1;

            total = total.max(self.get_total(bp, new_inv, new_rates, minutes_left));
        }

        // Clay robot
        if rates.clay < bp.obsidian_costs.1 && inventory.ore >= bp.clay_cost {
            // Buying clay-collecting
            let mut new_inv = inventory + rates;
            let mut new_rates = rates;
            new_inv.ore -= bp.clay_cost;
            new_rates.clay += 1;

            total = total.max(self.get_total(bp, new_inv, new_rates, minutes_left));
        }

        // Ore robot
        if rates.ore < bp.max_ore && inventory.ore >= bp.ore_cost {
            let mut new_inv = inventory + rates;
            let mut new_rates = rates;
            new_inv.ore -= bp.ore_cost;
            new_rates.ore += 1;

            total = total.max(self.get_total(bp, new_inv, new_rates, minutes_left));
        }

        // Do nothing if it can be useful
        if rates.ore < bp.max_ore && rates.clay < obsidian_clay && rates.obsidian < geode_obsidian {
            let new_inv = inventory + rates;

            total = total.max(self.get_total(bp, new_inv, rates, minutes_left));
        }

        total
    }
}

pub fn run() -> i32 {
    let bps: Vec<_> = include_str!("input.txt")
        .lines()
        .map(|b| all_consuming(Blueprint::parse)(b).finish().unwrap().1)
        .collect();

    let mut ret = 0;

    let inventory = Elements::default();
    let rates = Elements {
        ore: 1,
        ..Default::default()
    };

    let mut i = 0;
    for bp in &bps {
        i += 1;
        let mut state = State::default();
        let total = state.get_total(bp, inventory, rates, MINUTES);
        ret += i * total;
        // println!("{i}: {total} ({})", i * total);
    }

    return ret;
}

mod benchmarks {
    extern crate test;

    #[bench]
    fn bench_run(b: &mut test::Bencher) {
        b.iter(|| {
            let res = crate::run();
            assert!(res == 33);
        })
    }
}
