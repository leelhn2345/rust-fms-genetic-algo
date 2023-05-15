use rand::{seq::SliceRandom, thread_rng};

use crate::robot::RobotData;
use std::collections::HashMap;

type Solution = HashMap<String, RobotData>;
type Population = Vec<Specimen>;

pub struct Optimizer {
    home_depot: HashMap<String, String>,
    dist_utility: HashMap<String, HashMap<String, f32>>,
    candidate_num: u8,
}
#[derive(Clone, PartialEq)]
struct Specimen {
    soln: Solution,
    utility: u32,
    tasks: Vec<String>,
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
    ) -> (HashMap<String, RobotData>, u32) {
        let mut rng = thread_rng();
        let init_soln = self.init_soln(robot_data, active_tasks);
        let init_cost = self.soln_utility(&init_soln);

        let mut best_soln = init_soln;
        let mut best_cost = init_cost;

        let mut best_soln_vec: Vec<Solution> = vec![];
        let num_offsprings = offspring_rate as u32 * self.candidate_num as u32;

        best_soln_vec.push(best_soln.clone());

        let mut threshold = 0;
        let mut count = 0;

        let (tasks, clean_soln) = self.reset_data(&best_soln);

        let mut task_vec: Vec<Vec<String>> = vec![];
        (0..=self.candidate_num).for_each(|_| {
            let mut new_tasks = tasks.clone();
            new_tasks.shuffle(&mut rng);
            task_vec.push(new_tasks);
        });

        let population = self.init_population(task_vec, &clean_soln);

        let count_limit: u16 = 50;
        while count != count_limit {
            let mating_pool = self.selection(&population, num_offsprings);
            let mut child_tasks = self.crossover(mating_pool);
            let child_tasks = self.mutation(child_tasks, mutation_rate);
            count += 1;
        }
        (best_soln, best_cost)
    }

    fn selection(&self, popln: &Population, num_offsprings: u32) -> Vec<Population> {
        let mut mating_pool: Vec<Population> = vec![];
        let mut rng = thread_rng();

        // let mut alpha: Option<Specimen> = None;
        // let mut beta: Option<Specimen> = None;

        for _ in 0..=num_offsprings {
            let mut batch_pool: Population = vec![];
            let mut specimens = popln.choose_multiple(&mut rng, 2).cloned();

            let spec_a = match specimens.next() {
                Some(specimen) => specimen,
                None => panic!("iterator didn't yield enough specimen"),
            };

            let spec_b = match specimens.next() {
                Some(specimen) => specimen,
                None => panic!("iterator didn't yield enough specimen"),
            };

            if spec_a.utility > spec_b.utility {
                batch_pool.push(spec_a);
            } else {
                batch_pool.push(spec_b);
            }

            let mut incest = true;
            while incest {
                let mut specimens = popln.choose_multiple(&mut rng, 2).cloned();

                let spec_c = match specimens.next() {
                    Some(specimen) => specimen,
                    None => panic!("iterator didn't yield enough specimen"),
                };

                let spec_d = match specimens.next() {
                    Some(specimen) => specimen,
                    None => panic!("iterator didn't yield enough specimen"),
                };
                if spec_c.utility > spec_d.utility {
                    batch_pool.push(spec_c);
                } else {
                    batch_pool.push(spec_d);
                }
                if batch_pool[0] != batch_pool[1] {
                    incest = false;
                }
            }

            mating_pool.push(batch_pool);
        }
        mating_pool
    }

    fn crossover(&self, mating_pool: Vec<Population>) -> Vec<Vec<String>> {
        todo!()
    }

    fn mutation(&self, child_tasks: Vec<Vec<String>>, mutation_rate: f32) -> Vec<Vec<String>> {
        todo!()
    }

    fn init_population(&self, mut task_vec: Vec<Vec<String>>, clean_soln: &Solution) -> Population {
        let mut popln = vec![];

        for set_of_tasks in task_vec.iter_mut() {
            let specimen = self.get_specimen(set_of_tasks, clean_soln.clone());

            popln.push(specimen);
        }

        popln
    }
    fn get_specimen(&self, set_of_tasks: &mut Vec<String>, mut soln: Solution) -> Specimen {
        let tasks = set_of_tasks.clone();
        while !set_of_tasks.is_empty() {
            for robot_info in soln.values_mut() {
                robot_info.path.push(set_of_tasks.pop().unwrap());
                if set_of_tasks.is_empty() {
                    break;
                }
            }
        }
        for (robot, robot_info) in soln.iter_mut() {
            let veh_cap = robot_info.veh_cap as usize;
            let path = &robot_info.path;

            let mut new_path: Vec<String> = vec![];

            for (idx, node) in path.iter().enumerate() {
                if idx % veh_cap == 0 {
                    new_path.push(robot.into());
                }
                new_path.push(node.into());
            }
            if new_path.last().unwrap() != robot {
                new_path.push(robot.into());
            }
            robot_info.path = new_path;
        }
        let utility = self.soln_utility(&soln);

        Specimen {
            soln,
            tasks,
            utility,
        }
    }
    fn reset_data(&self, soln: &Solution) -> (Vec<String>, Solution) {
        let mut task_list = vec![];
        let mut clean_soln: Solution = HashMap::new();

        for (robot, robot_info) in soln.iter() {
            let path = &robot_info.path;
            let veh_cap = robot_info.veh_cap;

            let tasks: Vec<String> = path.iter().filter(|node| *node != robot).cloned().collect();

            task_list.extend_from_slice(&tasks);

            clean_soln.insert(
                robot.to_string(),
                RobotData {
                    veh_cap,
                    path: vec![],
                },
            );
        }

        (task_list, clean_soln)
    }
    fn soln_utility(&self, soln: &Solution) -> u32 {
        let mut total_dist_cost: f32 = 0.0;
        for (robot, robot_info) in soln.iter() {
            let home_depot = &self.home_depot[robot];
            let path = &robot_info.path;

            for (idx, node) in path.iter().enumerate() {
                if idx == 0 {
                    continue;
                }
                let mut destination = node;
                if node == robot {
                    destination = home_depot;
                }
                let origin = match *path[idx - 1] {
                    _ if (*node == *robot) => home_depot,
                    _ => &path[idx - 1],
                };
                if origin != node {
                    total_dist_cost += self.dist_utility[origin][destination];
                }
            }
        }
        total_dist_cost as u32
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
