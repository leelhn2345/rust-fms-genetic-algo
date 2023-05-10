mod config;
mod map;

use map::get_dist_utility;
use rand::seq::SliceRandom;

fn main() {
    println!("Hello, world!");

    let depot_num = 3;
    let robot_num = 10;
    let task_num = 50;

    let depot_nodes = ["N501", "N502", "N503", "N504", "N505"];

    assert!(depot_num <= depot_nodes.len(), "depot_num is too large");

    let mut rng = rand::thread_rng();

    let active_depots: Vec<_> = depot_nodes.choose_multiple(&mut rng, depot_num).collect();

    let robots_per_depot = divide_list(robot_num, depot_num as u8);

    // parse_map_json_data();
    get_dist_utility();
}
fn divide_list(dividend: u8, divisor: u8) -> Vec<u8> {
    let quotient = dividend / divisor;
    let remainder = dividend % divisor;

    let mut result = vec![divisor; quotient as usize];

    result
        .iter_mut()
        .take(remainder as usize)
        .for_each(|x| *x += 1);
    result
}
