use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct RobotData {
    veh_cap: u8,
    path: Vec<String>,
}

pub fn robot_fleet_data(robot_num: u8, veh_cap: u8) -> HashMap<String, RobotData> {
    let mut robot_data = HashMap::new();
    for i in 1..=robot_num {
        let robot_tag = String::from("R") + &i.to_string();
        robot_data.entry(robot_tag).or_insert(RobotData {
            veh_cap,
            path: vec![],
        });
    }

    robot_data
}

pub fn get_robots_home_depot(
    active_depots: Vec<String>,
    robots_per_depot: Vec<u8>,
) -> HashMap<String, String> {
    let mut home_depot = HashMap::new();

    let mut robot_idx = 0;

    for (depot_idx, depot) in active_depots.into_iter().enumerate() {
        for _ in 0..=robots_per_depot[depot_idx] {
            let robot_tag = format!("R{}", &robot_idx.to_string());
            home_depot.entry(robot_tag).or_insert(depot.clone());
            robot_idx += 1;
        }
    }
    home_depot
}
