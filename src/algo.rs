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
        self,
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
        self,
        robot_data: HashMap<String, RobotData>,
        active_tasks: Vec<String>,
    ) -> Solution {
        let mut soln = robot_data.clone();

        soln
    }
}
