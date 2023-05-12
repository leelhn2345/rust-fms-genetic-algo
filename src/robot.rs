use std::collections::HashMap;

#[derive(Debug)]
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
