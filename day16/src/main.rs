use std::{collections::HashMap};
use std::time::{Instant};

#[derive(Clone, Debug)]
struct Valve {
    name: String,
    flow_rate: i32,
    neighbour: Vec<String>,
    list: HashMap<String, i32>
}
fn main() {
    println!("Day16!");
    let lines = include_str!("../day16.txt");
    let mut valve_map: HashMap<String, Valve> = HashMap::new();    
    let mut valves: Vec<Valve> = Vec::new();

    lines.lines().for_each(|s| {
        let mut x: String = s.replace("Valve ", "");
        x = x.replace(" has flow rate=", ",");
        x = x.replace("; tunnels lead to valves ", ",");
        x = x.replace("; tunnel leads to valve ", ",");
        x = x.replace(" ", "");

        let mut name: String = "".to_string();
        let mut flow_rate:i32= 0;
        let mut index = 0;                    
        let mut neighbour: Vec<String> = Vec::new();            
        let str = x.split(",");
        
        for ss in str.into_iter() {
            let string_value:String = ss.to_string();
            if index == 0 {
                name = string_value;
            } else if index == 1 {
                flow_rate = string_value.parse().unwrap();
            } else {
                neighbour.push(string_value);
            }
            index+=1;
        }
        let valve:Valve = Valve { name: (name.clone()), flow_rate: (flow_rate), neighbour: (neighbour), list: (HashMap::new())};
        valves.push(valve.clone());

        valve_map.insert(name.clone(),valve.clone());        
    });     
        
    for mut v in valves.into_iter() {  
        find_valve(v.name.clone(), -1, &mut Vec::new(),&mut v, &mut valve_map); 
    }

    let mut max_pressure_a = 0;
    let now = Instant::now();
    find_best_route_a(&mut valve_map, "AA".to_string(), 30, 0, &mut Vec::new(), 0, 0, &mut max_pressure_a);
    println!("A: {} in time {}", max_pressure_a, now.elapsed().as_millis());

    // let mut max_pressure_b = 0;
    // let now_b = Instant::now();
    // find_best_route_b(&mut valve_map, "AA".to_string(), 0, "AA".to_string(), 0, 26, 0, &mut Vec::new(), 0, 0, &mut max_pressure_b, &mut Vec::new());
    // println!("B: {} in time {}", max_pressure_b, now_b.elapsed().as_millis());

}


fn find_best_route_b(valve_map: &mut HashMap<String, Valve>, me_target: String, mut me_steps_till_target: i32, elephant_target: String, mut elephant_steps_till_target: i32, max_steps: i32, mut current_step: i32, visited_list: &mut Vec<String>, pressure: i32, mut total_pressure: i32, max_pressure_found: &mut i32, pressure_steps: &mut Vec<String>) {
    let mut me_pressure = 0;
    let mut elephant_pressure = 0;
    //println!("Find best route! m - {} <{}> e - {} <{}>", me_target, me_steps_till_target, elephant_target, elephant_steps_till_target);
    if me_target == elephant_target {
        println!("WARNING!! Step {} and step {}", me_steps_till_target, elephant_steps_till_target);
    }
    //temp_list  to next
    while me_steps_till_target != 0 && elephant_steps_till_target != 0 && current_step < max_steps {
        //pressure_steps.push(format!("{} - {}", current_step+1, pressure));
        total_pressure = total_pressure + pressure;

        me_steps_till_target-=1;
        elephant_steps_till_target-=1;
        current_step+=1;   

        
        if me_steps_till_target == 1 {
            if visited_list.contains(&me_target) {
                me_steps_till_target-=1;
            }
        }

        if elephant_steps_till_target == 1 {
            if visited_list.contains(&me_target) {
                elephant_steps_till_target-=1;
            }            
        }
    }

    if me_steps_till_target == 0 && elephant_steps_till_target == 0 {

        let m_valve = valve_map.get(&me_target).unwrap().clone();
        let e_valve = valve_map.get(&elephant_target).unwrap().clone();

        if me_target == "AA" && elephant_target == "AA" {
            m_valve.list.into_iter().for_each(| (k,v)| {      
                e_valve.list.clone().into_iter().for_each(| (elephant_k,elephant_v)| {     
                    if k != elephant_k {     
                        find_best_route_b(valve_map, k.clone(), v+1, elephant_k, elephant_v+1, max_steps, current_step, &mut visited_list.clone(), pressure + me_pressure,total_pressure.clone(), max_pressure_found, &mut pressure_steps.clone());
                    }
                });
            });
        } else {
            if !visited_list.contains(&me_target) {
                me_pressure = m_valve.flow_rate;
                visited_list.push(me_target.clone());       
            }

            if !visited_list.contains(&elephant_target) {
                elephant_pressure = e_valve.flow_rate;
                visited_list.push(elephant_target.clone());       
            }

            m_valve.list.clone().into_iter().for_each(| (k,v)| {   
                if !visited_list.contains(&k) {   
                    e_valve.list.clone().into_iter().for_each(| (elephant_k,elephant_v)| {    
                        if !visited_list.contains(&elephant_k) { 
                            if k != elephant_k {     
                                find_best_route_b(valve_map, k.clone(), v+1, elephant_k, elephant_v+1, max_steps, current_step, &mut visited_list.clone(), pressure + me_pressure + elephant_pressure,total_pressure.clone(), max_pressure_found, &mut pressure_steps.clone());
                            } else if visited_list.len() == m_valve.list.len() {
                                if v < elephant_v {
                                    find_best_route_b(valve_map, k.clone(), v+1, "".to_string(), -1, max_steps, current_step, &mut visited_list.clone(), pressure + me_pressure + elephant_pressure,total_pressure.clone(), max_pressure_found, &mut pressure_steps.clone());
                                } else {
                                    find_best_route_b(valve_map, "".to_string(), -1, elephant_k.clone(), elephant_v+1, max_steps, current_step, &mut visited_list.clone(), pressure + me_pressure + elephant_pressure,total_pressure.clone(), max_pressure_found, &mut pressure_steps.clone());
                                }
                            }
                        }
                    });
                }
            });
        }
    } else if me_steps_till_target == 0 {
        let valve = valve_map.get(&me_target).unwrap().clone();

        if !visited_list.contains(&me_target) {
            me_pressure = valve.flow_rate;
            visited_list.push(me_target.clone());       
        }
        
        let mut has_send = false;
        valve.list.into_iter().for_each(| (k,v)| {            
            if visited_list.contains(&k) == false {     
                if !(k == elephant_target) {
                    has_send = true;
                    find_best_route_b(valve_map, k, v+1, elephant_target.clone(), elephant_steps_till_target, max_steps, current_step, &mut visited_list.clone(), pressure + me_pressure,total_pressure.clone(), max_pressure_found, &mut pressure_steps.clone());
                }
            }            
        });

        if elephant_steps_till_target > 0 && !has_send{
            find_best_route_b(valve_map, "".to_string(), -1, elephant_target.clone(), elephant_steps_till_target, max_steps, current_step, &mut visited_list.clone(), pressure + me_pressure,total_pressure.clone(), max_pressure_found, &mut pressure_steps.clone());
        }
    } else if elephant_steps_till_target == 0 {
        let valve = valve_map.get(&elephant_target).unwrap().clone();

        if !visited_list.contains(&elephant_target) {
            elephant_pressure = valve.flow_rate;
            visited_list.push(elephant_target.clone());       
        }

        let mut has_send = false;
        valve.list.into_iter().for_each(| (k,v)| {            
            if visited_list.contains(&k) == false {                
                if !(k == me_target) {
                    has_send = true;
                    find_best_route_b(valve_map, me_target.clone(),  me_steps_till_target, k, v+1, max_steps, current_step, &mut visited_list.clone(), pressure + elephant_pressure,total_pressure, max_pressure_found, &mut pressure_steps.clone());
                }
            }            
        });

        if me_steps_till_target > 0 && !has_send {
            find_best_route_b(valve_map, me_target.clone(), me_steps_till_target, "".to_string(), -1, max_steps, current_step, &mut visited_list.clone(), pressure + me_pressure,total_pressure.clone(), max_pressure_found, &mut pressure_steps.clone());
        }
    }
    
    let left = max_steps - current_step;

    total_pressure+= left * (pressure + me_pressure + elephant_pressure);

    if total_pressure > *max_pressure_found {
    let left = max_steps - current_step;
        //pressure_steps.push(format!("Last Multi = {} - {} * {} = {}", current_step, (pressure + me_pressure + elephant_pressure), left, total_pressure));
        *max_pressure_found = total_pressure;
        println!("current max pressure: {}", max_pressure_found);
        //println!("{:?}", pressure_steps);
    }
}

fn find_best_route_a(valve_map: &mut HashMap<String, Valve>, current_valve: String, max_steps: i32, current_step: i32, visited_list: &mut Vec<String>, current_pressure: i32, total_pressure: i32, max_pressure_found: &mut i32) {
    let valve = valve_map.get(&current_valve).unwrap().clone();
    
    valve.list.into_iter().for_each(| (k,v)| {
        if (current_step + v + 1) <= max_steps {
            if visited_list.contains(&k) == false {
                let mut clone_list = visited_list.clone();
                clone_list.push(k.clone());

                let new_total_pressure: i32 = (current_pressure * (v+1)) + total_pressure;
                let target = valve_map.get(&k.clone()).unwrap();
                let new_current_pressure = current_pressure + target.flow_rate;

                if new_total_pressure >  *max_pressure_found {
                    *max_pressure_found = new_total_pressure;
                }

                find_best_route_a(valve_map, k, max_steps, current_step + v + 1, &mut clone_list, new_current_pressure,new_total_pressure, max_pressure_found);
            }
        }
    });

    let steps_left = max_steps - current_step;
    let result: i32 = (current_pressure * steps_left) + total_pressure;

    if result > *max_pressure_found {
        *max_pressure_found = result;
    }
}

fn find_valve(source_name: String, mut step: i32, tunnels: &mut Vec<String>, valve: &mut Valve, valve_map: &mut HashMap<String, Valve>) {
    let valve_name = &valve.name;
    step+=1;
    tunnels.push(valve_name.to_string());
    let n = &valve.neighbour;
    
    if valve.flow_rate != 0 && !source_name.eq(valve_name)  {
        let source_valve = valve_map.get_mut(&source_name).unwrap();

        let current_val: Option<&i32> = source_valve.list.get(&valve_name.to_string());
        if current_val.is_none() || step <  *current_val.unwrap() {
            source_valve.list.insert(valve_name.to_string(), step);
        }
    }
    
    for v in n.into_iter() {
        let mut target_v = valve_map.get_mut(v).unwrap().clone();
        if !tunnels.contains(&target_v.name) {
            find_valve(source_name.clone(), step.clone(), &mut tunnels.clone(), &mut target_v, valve_map);
        }
    }    
}
