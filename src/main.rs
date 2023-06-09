mod algo;
mod config;
mod map;
mod robot;
mod tasks;

use algo::Optimizer;
use config::{MUTATION_RATE, OFFSPRING_RATE};
use map::get_dist_utility;
use rand::seq::SliceRandom;
use robot::{get_robots_home_depot, robot_fleet_data};
use tasks::get_random_task_nodes;
fn main() {
    println!("Hello, world!");

    let depot_num = 3;
    let robot_num = 10;
    let task_num = 50;
    let veh_cap = 1;

    let depot_nodes = ["N501", "N502", "N503", "N504", "N505"];

    assert!(depot_num <= depot_nodes.len(), "depot_num is too large");

    let mut rng = rand::thread_rng();

    let active_depots: Vec<String> = depot_nodes
        .choose_multiple(&mut rng, depot_num)
        .map(|node| node.to_string())
        .collect();

    let robots_per_depot = divide_list(robot_num, depot_num as u8);

    let dist_utility = get_dist_utility().unwrap();

    let active_tasks = get_random_task_nodes(&dist_utility, &mut rng, &active_depots, task_num);

    let robot_data = robot_fleet_data(robot_num, veh_cap);

    let robots_home_depot = get_robots_home_depot(active_depots, robots_per_depot);

    let optimizer = Optimizer::new(robots_home_depot, dist_utility, 50);

    let soln = optimizer.genetic_algo(robot_data, active_tasks, MUTATION_RATE, OFFSPRING_RATE);

    dbg!(soln);
}
fn divide_list(dividend: u8, divisor: u8) -> Vec<u8> {
    let quotient = dividend / divisor;
    let remainder = dividend % divisor;

    let mut result = vec![divisor; quotient.into()];

    result
        .iter_mut()
        .take(remainder.into())
        .for_each(|x| *x += 1);
    result
}
