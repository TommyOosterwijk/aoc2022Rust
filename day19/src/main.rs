use scanf::sscanf;
fn main() {
    println!("Day19!");

    let lines: Vec<String> = include_str!("../day19.txt").lines().map(|s|s.to_string()).collect();

    let result_a: i32 = lines.clone().into_iter().map(|s|{
        let mut blueprint_id: i32 = 0;
        let mut ore_robot: i32 = 0;
        let mut clay_robot_ore: i32 = 0;
        let mut obsidian_ore: i32 = 0;
        let mut obsidian_clay: i32 = 0;
        let mut geode_ore: i32 = 0;
        let mut geode_obsidian: i32 = 0;
        let mut max_geode = 0;

        sscanf!(&s,"Blueprint {}: Each ore robot costs {} ore. Each clay robot costs {} ore. Each obsidian robot costs {} ore and {} clay. Each geode robot costs {} ore and {} obsidian.", blueprint_id, ore_robot, clay_robot_ore, obsidian_ore, obsidian_clay, geode_ore, geode_obsidian)
            .expect("error parsing instruction");

            println!("ID {} ore_robot {} {} {} {} {} {}", blueprint_id, ore_robot, clay_robot_ore, obsidian_ore, obsidian_clay, geode_ore, geode_obsidian);
            let mut factory:Vec<i32> = Vec::new();
            factory.push(ore_robot);
            factory.push(clay_robot_ore);
            factory.push(obsidian_ore);
            factory.push(obsidian_clay);
            factory.push(geode_ore);
            factory.push(geode_obsidian);
            
            let mut robots:Vec<i32> = Vec::from([1,0,0,0]);
            let mut resources:Vec<i32> = Vec::from([0,0,0,0]);

        do_work(&mut max_geode, 24, &mut factory, &mut robots, &mut resources);
        return max_geode * blueprint_id;
    }).sum();

    println!("A: {}", result_a);

    let result_b:Vec<i32> = lines.into_iter().take(3).map(|s|{
        let mut blueprint_id: i32 = 0;
        let mut ore_robot: i32 = 0;
        let mut clay_robot_ore: i32 = 0;
        let mut obsidian_ore: i32 = 0;
        let mut obsidian_clay: i32 = 0;
        let mut geode_ore: i32 = 0;
        let mut geode_obsidian: i32 = 0;
        let mut max_geode = 0;

        sscanf!(&s,"Blueprint {}: Each ore robot costs {} ore. Each clay robot costs {} ore. Each obsidian robot costs {} ore and {} clay. Each geode robot costs {} ore and {} obsidian.", blueprint_id, ore_robot, clay_robot_ore, obsidian_ore, obsidian_clay, geode_ore, geode_obsidian)
            .expect("error parsing instruction");

            println!("ID {} ore_robot {} {} {} {} {} {}", blueprint_id, ore_robot, clay_robot_ore, obsidian_ore, obsidian_clay, geode_ore, geode_obsidian);
            let mut factory:Vec<i32> = Vec::new();
            factory.push(ore_robot);
            factory.push(clay_robot_ore);
            factory.push(obsidian_ore);
            factory.push(obsidian_clay);
            factory.push(geode_ore);
            factory.push(geode_obsidian);
            
            let mut robots:Vec<i32> = Vec::from([1,0,0,0]);
            let mut resources:Vec<i32> = Vec::from([0,0,0,0]);

        do_work(&mut max_geode, 32, &mut factory, &mut robots, &mut resources);
        return max_geode;
    }).collect();

    let mut b = 1;

    for v in result_b.into_iter() {
        b*=v;
    }

    println!("B: {}", b);

}

fn do_work(max_geode: &mut i32, mut turns_left: i32, f_b: &Vec<i32>, robots: &mut Vec<i32>, resources: &mut Vec<i32>) {

    let mut s_0: bool = true;
    let mut s_1: bool = true;
    let mut s_2: bool = true;

    while turns_left > 0 {        
        turns_left-=1;

        //ORE
        if resources[0] >= f_b[0] && s_0 && s_0 && robots[0] < 5 {
            let mut ro_c = robots.clone();
            let mut re_c = resources.clone();

            for i in 0.. ro_c.len() {
                re_c[i] += ro_c[i];
            }

            ro_c[0]+=1;
            re_c[0]-= f_b[0];
            do_work(max_geode, turns_left.clone(), f_b, &mut ro_c, &mut re_c);
            s_0 = false;
        } 

        //CLAY
        if resources[0] >= f_b[1] && s_1 && robots[1] <= f_b[3] && resources[1] <= (f_b[3] - robots[1]) * f_b[3] {
            let mut ro_c = robots.clone();
            let mut re_c = resources.clone();

            for i in 0.. ro_c.len() {
                re_c[i] += ro_c[i];
            }

            ro_c[1]+=1;
            re_c[0]-= f_b[1];
            do_work(max_geode, turns_left.clone(), f_b, &mut ro_c, &mut re_c);
            s_1 = false;
        }

        if resources[0] >= f_b[2] && resources[1] >= f_b[3] && s_2 && robots[2] <= f_b[5] && resources[2] <= (f_b[5] - robots[2]) * f_b[5] {
            let mut ro_c = robots.clone();
            let mut re_c = resources.clone();

            for i in 0.. ro_c.len() {
                re_c[i] += ro_c[i];
            }

            ro_c[2]+=1;
            re_c[0]-= f_b[2];
            re_c[1]-= f_b[3];
            do_work(max_geode, turns_left.clone(), f_b, &mut ro_c, &mut re_c);
            s_2 = false;
        }

        if resources[0] >= f_b[4] && resources[2] >= f_b[5] {
            let mut ro_c = robots.clone();
            let mut re_c = resources.clone();

            for i in 0.. ro_c.len() {
                re_c[i] += ro_c[i];
            }

            ro_c[3]+=1;
            re_c[0]-= f_b[4];
            re_c[2]-= f_b[5];
            do_work(max_geode, turns_left.clone(), f_b, &mut ro_c, &mut re_c);
        } 
        
        for i in 0.. robots.len() {
            resources[i] += robots[i];
        }
    }
    if *max_geode < resources[3] {
        *max_geode = resources[3];
        //println!("{:?}", robots);

    }
}