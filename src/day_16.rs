use crate::graph::graph::Graph;

pub fn main() {
    let input = include_str!("resources/test/day_16_valves_example.txt");
    let mut cave = Graph::parse(input, "AA");
    cave.eliminate_edges_to_nodes_weighting_zero();
    cave.remove_unreachable_nodes();
    cave.cache_all_edges();
    cave.print_graph_csacademy_format();
    println!("{}", count_released_pressure(cave, vec!["AA", "DD", "BB", "JJ", "HH", "EE", "CC"], 30));
}

fn count_released_pressure(cave: Graph, order: Vec<&str>, time: usize) -> usize {
    let mut releasing_pressure = 0;
    let mut pressure = 0;
    let mut t = time;
    for i in 0..(order.len() - 1) {
        let time_spent = cave.get_shortest_path(order.get(i).unwrap(), order.get(i + 1).unwrap());
        if t as isize - time_spent as isize - 1 < 0 {
            break;
        }
        pressure += (time_spent + 1) * releasing_pressure;
        t -= time_spent + 1;
        releasing_pressure += cave.get_nodes().iter().find(|node| node.get_name() == order.get(i + 1).unwrap().to_string()).unwrap().get_weight();
    }

    if t > 0 {
        return pressure + (t * releasing_pressure);
    }
    pressure
}

//fn heuristic_approach(cave: Graph, time: usize, factor: isize) -> usize {
//    let mut releasing_pressure = 0;
//    let mut pressure = 0;
//    let mut valve_names = self.map.iter().map(|valve| valve.name.clone()).collect::<Vec<String>>();
//    let mut current_valve = self.start.clone();
//    let mut t = time;
//
//    loop {
//        valve_names.retain(|valve| *valve != current_valve);
//
//        if valve_names.is_empty() {
//            break;
//        }
//
//        let mut next_valve = "".to_string();
//        for x in 3..(t as isize) {
//            let mut local_value = vec![];
//
//            for valve in valve_names.clone() {
//                let tmp_max_val = (x - (self.get_shortest_path(current_valve.as_str(), valve.as_str()) + 1) as isize) * self.get_valve_by_name(valve.as_str()).flow as isize;
//                if tmp_max_val <= factor {
//                    continue;
//                } else {
//                    local_value.push((valve.clone(), tmp_max_val))
//                }
//            }
//
//            if !local_value.is_empty() {
//                next_valve = local_value.iter().find(|(valve, val)| val == local_value.clone().iter().map(|(valve, val)| val).max().unwrap()).unwrap().clone().0;
//                break;
//            }
//
//        }
//
//
//        if next_valve == "" {
//            next_valve = valve_names.first().unwrap().clone();
//        }
//
//
//        let time_spent = self.get_shortest_path(current_valve.as_str(), next_valve.as_str());
//        if t as isize - (time_spent as isize + 1) < 0 {
//            break;
//        }
//
//
//        t = t - (time_spent + 1);
//        pressure += (time_spent + 1) * releasing_pressure;
//        let addition = self.get_valve_by_name(next_valve.as_str()).flow;
//        releasing_pressure += addition;
//        current_valve = next_valve;
//    }
//
//    if t > 0 {
//        return pressure + (t * releasing_pressure);
//    }
//    pressure
//}

//fn brute_force_most_pressure(&self) -> usize {
//    let valves = self.get_sorted_valves().iter().cloned().map(|(name, _)| name).filter(|valve| valve.as_str() != self.start).collect::<Vec<String>>();
//    valves.iter().rev().cloned().permutations(valves.len()).par_bridge().fold(|| usize::MIN, |acc, permutation| {
//        let mut base = vec![self.start.clone()];
//        base.extend(permutation.iter().rev().cloned());
//        let p = self.count_released_pressure(base, 30);
//        if p > acc {
//            println!("{:?}", p);
//            return p;
//        }
//        return acc;
//    }).max().unwrap()
//}
