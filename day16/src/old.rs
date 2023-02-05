/*
Notes:

TODO:
- replace valve name by int.
- replace HashMap by [bool] & [isize]
- go becomes "mv & open"

- when adding pressure, it should consider than N minutes have passed.
- at the end, if above 30 min, N-30 should be considered for the pressure added.
- the open flow add pressure straight away.

DONE:
- only keep path between nodes with flow != 0
- reduce path by summing cost
*/

use std::fs::read_to_string;
use std::collections::{HashMap, HashSet};
use pathfinding::prelude::dijkstra_all;
use std::iter::repeat;

#[derive(Debug, Clone)]
struct Valve {
    name: String,
    flow: isize,
    succ: Vec<(String, isize)>,
}

fn main() {
    // Read input
    let input = read_to_string("./data/example.txt").unwrap();
    let valves: HashMap<String, Valve> = input
        .lines()
        .map(|line| {
            let mut tokens = line.split_ascii_whitespace();
            let name: String = tokens.nth(1).unwrap().to_string();
            let flow = tokens.nth(2).unwrap()[5..].replace(";", "").parse::<isize>().unwrap();
            let succ: Vec<(String, isize)> = tokens.skip(4).map(|token| (token.replace(",", ""), 1)).collect();
            (name.clone(), Valve { name, flow, succ })})
        .collect();

    // Part 1
    // First, let's simpligy the graph and only keep the valves with non-zero flow + the first one.
    let mut nonzerovalves: HashMap<String, Valve> = valves
            .values()
            .filter(|v| v.flow > 0)
            .map(|v| (v.name.clone(), Valve{ name: v.name.clone(), flow: v.flow, succ: vec![] }))
            .collect();
    let mut start = valves["AA"].clone();
    start.succ.clear();
    nonzerovalves.insert("AA".into(), start);
    // Then, Recompute all the successors in the simplified graph.
    let nonzeronames: Vec<String> = nonzerovalves.keys().map(|k| k.to_owned()).collect();
    for orgname in nonzeronames {
        // println!("From {:?}", orgname);
        // Compute all shortest paths from orgname
        let reachables = dijkstra_all(
            &orgname,
            |valve| valves[valve].succ.to_owned()
        );
        // For all reached non-zero valves, add it as a successor of 'orgname'.
        'nextreached: for (destname, (_pred, _cost)) in &reachables {
            if !nonzerovalves.contains_key(destname) { continue }
            // Rebuild path from 'origin' to 'destination'
            let mut sumcost = 0;
            let mut org = destname.to_owned();
            while org != orgname {
                // println!("    Going up to {:?}", org);
                if org != *destname && nonzerovalves.contains_key(&org) { continue 'nextreached }
                // println!("        which is not a non-zero");
                let (pred, cost) = &reachables[&org];
                org = pred.to_owned();
                sumcost += cost;
            }
            // Update valve with newly found path
            nonzerovalves
                .entry(orgname.to_owned())
                .and_modify(|v| v.succ.push((destname.to_owned(), sumcost)));
        }
    }
    println!("nonserovalves: {:?}", nonzerovalves);

    let mut opened: HashSet<String> = HashSet::new();
    let maxi = go(&nonzerovalves, &mut opened, &String::from("AA"), 0, 0, 0);

    // 1772 is too low
    println!("Part1: {:?}", maxi);
}

fn go(valves: &HashMap<String, Valve>, opened: &mut HashSet<String>, valvename: &String, time: isize, pressure: isize, flow: isize) -> isize {

    let curr = &valves[valvename]; 
    let indent = repeat(" ").take(time as usize).collect::<String>();
    println!("{:?}Min:{:?} valve:{:?}[{:?}] flow:{:?} pressure:{:?}", indent, time, valvename, curr.flow, flow, pressure);
    if time >= 30 {
        return pressure;
    }
    let mut maxi = 0;
    for (succ, dist) in &curr.succ {
        // Open the valve and move.
        if curr.flow > 0 && !opened.contains(&curr.name) {
            opened.insert(curr.name.clone());
            maxi = maxi.max(go(valves, opened, &succ, time + dist + 1, pressure + dist * (flow + curr.flow), flow + curr.flow));
            opened.remove(&curr.name);
        }
        // Do not open the valve and move.
        maxi = maxi.max(go(valves, opened, &succ, time + dist, pressure + dist * flow, flow));
    }
    maxi
}



    // let indexes: Vec<usize> = reduced.iter().filter(|valve| valve.name != "AA").map(|valve| valve.index).collect();
    // let start = reduced.iter().find(|valve| valve.name == "AA").unwrap().index;
    // let maxi = indexes.iter().permutations(indexes.len()).map(|perm| score(&reduced, start, &perm)).max();
    // println!("Part 1: {:?}", maxi.unwrap());





fn score(reduced: &Vec<Valve>, start: usize, indexes: &Vec<&usize>) -> usize {
    
    // Open all non-zero valves from biggest to smallest
    let mut time = 1;
    let mut pressure = 0;
    let mut icurvalve = start;
    for inext in indexes {
        // Todo: this is ugly: the successors should be in order...
        let length = reduced[icurvalve].successors.iter().find(|(succ, _)| *succ == **inext).unwrap().1;
        time += length + 1;
        if time > 30 { break }
        pressure += (31 - time) * reduced[**inext].flow;
        // println!(
        //     "At min {:?} moved&opened to {:?}[{:?}]: pressure:{:?} total:{:?}",
        //     time, next.index, reduced[next.index].flow, (30 - time) * reduced[next.index].flow, pressure
        // );
        icurvalve = **inext;
    }
    pressure
}



fn greedy(valves: &Vec<Valve>, start: usize) -> usize {

    // Retrieve all non-zero valves
    let mut nonzeros: Vec<&Valve> = valves.iter().filter(|valve| valve.flow != 0).collect();
    nonzeros.sort_by_key(|valve| 1000 - valve.flow);
    // println!("Non-zeros: {:?}", nonzeros);

    // Solution of example (gives 1651):
    // let nonzeros: Vec<&Valve> = [3,1,9,7,4,2].iter().map(|i| &valves[*i]).collect();

    // Open all non-zero valves from biggest to smallest
    let mut time = 1;
    let mut pressure = 0;
    let mut icurvalve = start;
    for next in nonzeros {
        // len of path from current to nonzero
        let (_, length): (Vec<usize>, usize) = dijkstra(
            &icurvalve,
            |ivalve| valves[*ivalve].successors.to_owned(),
            |ivalve| *ivalve == next.index,
        ).unwrap();
        time += length + 1;
        pressure += (31 - time) * valves[next.index].flow;
        // println!(
        //     "At min {:?} moved&opened to {:?}[{:?}]: pressure:{:?} total:{:?}",
        //     time, next.index, valves[next.index].flow, (30 - time) * valves[next.index].flow, pressure
        // );
        icurvalve = next.index;
    }
    pressure
}




    // let mut opened = vec![false; valves.len()];
    // let maxi = runthrough(&valves, &mut opened, name_to_index["AA"], 0, 0, 0, num_nonzero);
    // println!("Part 1: {:?}", maxi);


// fn runthrough(
//     valves: &Vec<Valve>,
//     opened: &mut Vec<bool>,
//     index: usize,
//     time: usize,
//     pressure: usize,
//     num_opened: usize,
//     num_nonzero: usize,
// ) -> usize {

//     println!("{:?}{:?} v:{:?} p:{:?} o:{:?} t:{:?}", repeat(" ").take(time as usize).collect::<String>(), time, index, pressure, num_opened, num_nonzero);

//     let valve = &valves[index];
//     if time >= 30 || num_opened == num_nonzero {
//         return pressure;
//     }
//     let mut maxi = 0;
//     for successor in &valve.successors {
//         // Open the valve and move.
//         if valve.flow > 0 && !opened[index] {
//             opened[index] = true;
//             maxi = maxi.max(runthrough(
//                 valves,
//                 opened,
//                 *successor,
//                 time + 2,
//                 pressure + valve.flow * (30 - time),
//                 num_opened + 1,
//                 num_nonzero,
//             ));
//             opened[index] = false;
//         }
//         // Do not open the valve and move.
//         maxi = maxi.max(runthrough(
//             valves,
//             opened,
//             *successor,
//             time + 1,
//             pressure + valve.flow * (30 - time),
//             num_opened,
//             num_nonzero,
//         ));
//     }
//     maxi
// }





fn explore3(
    reduced: &Vec<Valve>,
    states: &mut Vec<bool>,
    ipos1: usize,
    ipos2: usize,
    timearrival1: usize,
    timearrival2: usize,
    time: usize,
    score: usize,
) -> usize {

    let indent = repeat(" ").take(time).collect::<String>();

    println!(
        "{}{:?}: posx:{:?} arrival:{:?} flowx:{:?} score:{:?}",
        indent,
        time,
        (ipos1, ipos2),
        (timearrival1, timearrival2),
        (reduced[ipos1].flow, reduced[ipos2].flow),
        score
    );

    if time > 26 {
        println!("{}=> SCORE(time limit): {:?}", indent, score);
        return score;
    }
    let mut newscore = score;
    let days = 26;

    if time == timearrival1 && time != timearrival2 {
        for (inext, length) in &reduced[ipos1].successors {
            if !states[*inext] {
                states[*inext] = true;
                let timearrival1 = time + length + 1;
                newscore = newscore.max(explore3(
                    reduced,
                    states,
                    *inext,
                    ipos2,
                    timearrival1,
                    timearrival2,
                    timearrival1.min(timearrival2),
                    score + (days - time) * reduced[ipos1].flow,
                ));
                states[*inext] = false;
            }
        }
    } else if time != timearrival1 && time == timearrival2 {
        for (inext, length) in &reduced[ipos2].successors {
            if !states[*inext] {
                states[*inext] = true;
                let timearrival2 = time + length + 1;
                newscore = newscore.max(explore3(
                    reduced,
                    states,
                    ipos1,
                    *inext,
                    timearrival1,
                    timearrival2,
                    timearrival1.min(timearrival2),
                    score + (days - time) * reduced[ipos2].flow,
                ));
                states[*inext] = false;
            }
        }
    } else if time == timearrival1 && time == timearrival2 {
        let mut successors1 = reduced[ipos1].successors.clone();
        successors1.push((0, 1000));
        let mut successors2 = reduced[ipos2].successors.clone();
        successors2.push((0, 1000));
        for (inext1, length1) in &successors1 {
            for (inext2, length2) in &successors2 {
                if inext1 == inext2 || states[*inext1] || states[*inext2] {
                    continue;
                }
                states[*inext1] = true;
                states[*inext2] = true;
                let timearrival1 = time + length1 + 1;
                let timearrival2 = time + length2 + 1;
                newscore = newscore.max(explore3(
                    reduced,
                    states,
                    *inext1,
                    *inext2,
                    timearrival1,
                    timearrival2,
                    timearrival1.min(timearrival2),
                    score
                        + (days - time) * reduced[ipos1].flow
                        + (days - time) * reduced[ipos2].flow,
                ));
                states[*inext1] = false;
                states[*inext2] = false;
            }
        }
    } else {
        panic!("This should never happen!")
    }
    println!("{}=> SCORE(way back): {:?}", indent, newscore);
    newscore
}
