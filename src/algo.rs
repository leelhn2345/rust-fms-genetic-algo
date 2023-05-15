use crate::robot::RobotData;
use std::collections::HashMap;

type Solution = HashMap<String, RobotData>;

pub struct Optimizer {
    home_depot: HashMap<String, String>,
    dist_utility: HashMap<String, HashMap<String, f32>>,
    candidate_num: u8,
}

impl Optimizer {
    pub fn new(
        home_depot: HashMap<String, String>,
        dist_utility: HashMap<String, HashMap<String, f32>>,
        candidate_num: u8,
    ) -> Self {
        Optimizer {
            home_depot,
            dist_utility,
            candidate_num,
        }
    }

    pub fn genetic_algo(
        &self,
        robot_data: HashMap<String, RobotData>,
        active_tasks: Vec<String>,
        mutation_rate: f32,
        offspring_rate: f32,
    ) -> HashMap<String, RobotData> {
        let init_soln = self.init_soln(robot_data, active_tasks);

        let mut best_soln_vec: Vec<Solution> = vec![];
        init_soln
    }

    fn init_soln(
        &self,
        mut robot_data: HashMap<String, RobotData>,
        mut active_tasks: Vec<String>,
    ) -> Solution {
        let mut soln = robot_data.clone();

        while !active_tasks.is_empty() {
            for val in robot_data.values_mut() {
                if let Some(task) = active_tasks.pop() {
                    val.path.push(task)
                }
            }
        }

        for (robot, robot_info) in robot_data.iter() {
            let veh_cap = robot_info.veh_cap;
            let path = &robot_info.path;

            if path.is_empty() {
                soln.remove(robot);
                continue;
            }
            let mut new_path: Vec<String> = vec![];

            for (idx, node) in path.iter().enumerate() {
                if idx % usize::from(veh_cap) == 0 {
                    new_path.push(robot.into());
                }
                new_path.push(node.into());
            }
            if new_path.last().unwrap() != robot {
                new_path.push(robot.into());
            }
            soln.get_mut(robot).unwrap().path = new_path;
        }
        soln
    }
}
