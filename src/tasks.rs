use std::collections::HashMap;

use rand::{rngs::ThreadRng, seq::SliceRandom};

pub fn get_random_task_nodes(
    dist_utility: &HashMap<String, HashMap<String, f32>>,
    rng: &mut ThreadRng,
    depot_nodes: &[String],
    task_num: usize,
) -> Vec<String> {
    let mut keys: Vec<&String> = dist_utility.keys().collect();
    keys.retain(|node| !depot_nodes.contains(node));

    let active_tasks: Vec<String> = keys
        .choose_multiple(rng, task_num)
        .map(|node| node.to_string())
        .collect();
    active_tasks
}
