use itertools::Itertools;
use pathfinding::directed::dijkstra::dijkstra_all;
use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct InputLine {
    name: String,
    flow: usize,
    succ: Vec<String>,
}

#[derive(Debug, Clone)]
struct Valve {
    index: usize,
    oldindex: usize,
    flow: usize,
    successors: Vec<(usize, usize)>,
    name: String,
}

fn main() {
    // Read input
    let input = read_to_string("./data/input.txt").unwrap();
    let inputlines: Vec<InputLine> = input
        .lines()
        .map(|line| {
            let mut tokens = line.split_ascii_whitespace();
            let name: String = tokens.nth(1).unwrap().to_string();
            let flow = tokens.nth(2).unwrap()[5..]
                .replace(";", "")
                .parse::<usize>()
                .unwrap();
            let succ: Vec<String> = tokens.skip(4).map(|token| token.replace(",", "")).collect();
            InputLine { name, flow, succ }
        })
        .collect();

    let name_to_index: HashMap<String, usize> = inputlines
        .iter()
        .enumerate()
        .map(|(index, inputline)| (inputline.name.to_owned(), index))
        .collect();

    let valves: Vec<Valve> = inputlines
        .iter()
        .enumerate()
        .map(|(index, inputline)| Valve {
            index: index,
            oldindex: 0,
            flow: inputline.flow,
            successors: inputline
                .succ
                .iter()
                .map(|succname| (name_to_index[succname], 1))
                .collect::<Vec<(usize, usize)>>(),
            name: inputline.name.clone(),
        })
        .collect();

    let reduced = reduce(&valves);
    let start = reduced
        .iter()
        .find_position(|valve| valve.name == "AA")
        .unwrap()
        .0;

    // Part 1:
    let mut states = vec![false; reduced.len()];
    let pressure = explore(&reduced, &mut states, start, 1);
    println!("Part 1: {:?}", pressure);

    // Part 2:
    let mut states = vec![false; reduced.len()];
    let pressure = explore2(&reduced, &mut states, start, start, 0, 0, 0, 0);
    println!("Part 2: {:?}", pressure);
}

fn explore2(
    reduced: &Vec<Valve>,
    states: &mut Vec<bool>,
    ipos1: usize,
    ipos2: usize,
    timearrival1: usize,
    timearrival2: usize,
    time: usize,
    hint: usize,
) -> usize {
    // Shameless heuristics to speed up things dramatically.
    // Would probably not work with another graph.
    let deep = 13;
    if time > 25
        || (time == 4 && hint < 92)
        || (time == 6 && hint < 466)
        || (time == 8 && hint < 886)
        || (time == 14 && hint < 1914)
        || (time == 17 && hint < 2289)
        || (time == 25 && hint < 2584)
    {
        // && time > 25 { // this last test is an optim to save a bit of time...
        return 0;
    }

    if time > 26 {
        return 0;
    }
    let mut maxirest = 0;

    if time == timearrival1 && time != timearrival2 {
        let mut count = 0;
        for (inext, length) in &reduced[ipos1].successors {
            if count > deep {
                break;
            }
            if !states[*inext] {
                states[*inext] = true;
                let timearrival1 = time + length + 1;
                maxirest = maxirest.max(explore2(
                    reduced,
                    states,
                    *inext,
                    ipos2,
                    timearrival1,
                    timearrival2,
                    timearrival1.min(timearrival2),
                    hint + (26 - time) * reduced[ipos1].flow,
                ));
                states[*inext] = false;
                count += 1;
            }
        }
        (26 - time) * reduced[ipos1].flow + maxirest
    } else if time != timearrival1 && time == timearrival2 {
        let mut count = 0;
        for (inext, length) in &reduced[ipos2].successors {
            if count > deep {
                break;
            }
            if !states[*inext] {
                states[*inext] = true;
                let timearrival2 = time + length + 1;
                maxirest = maxirest.max(explore2(
                    reduced,
                    states,
                    ipos1,
                    *inext,
                    timearrival1,
                    timearrival2,
                    timearrival1.min(timearrival2),
                    hint + (26 - time) * reduced[ipos2].flow,
                ));
                states[*inext] = false;
                count += 1;
            }
        }
        (26 - time) * reduced[ipos2].flow + maxirest
    } else if time == timearrival1 && time == timearrival2 {
        let mut count1 = 0;
        for (inext1, length1) in &reduced[ipos1].successors {
            if count1 > deep {
                break;
            }
            let mut count2 = 0;
            for (inext2, length2) in &reduced[ipos2].successors {
                if count2 > deep {
                    break;
                }
                if inext1 == inext2 || states[*inext1] || states[*inext2] {
                    continue;
                }
                states[*inext1] = true;
                states[*inext2] = true;
                let timearrival1 = time + length1 + 1;
                let timearrival2 = time + length2 + 1;
                maxirest = maxirest.max(explore2(
                    reduced,
                    states,
                    *inext1,
                    *inext2,
                    timearrival1,
                    timearrival2,
                    timearrival1.min(timearrival2),
                    hint + (26 - time) * reduced[ipos1].flow + (26 - time) * reduced[ipos2].flow,
                ));
                states[*inext1] = false;
                states[*inext2] = false;
                count2 += 1;
            }
            count1 += 1;
        }
        (26 - time) * reduced[ipos1].flow + (26 - time) * reduced[ipos2].flow + maxirest
    } else {
        panic!("This should never happen!")
    }
}

fn explore(reduced: &Vec<Valve>, states: &mut Vec<bool>, ipos: usize, time: usize) -> usize {
    if time > 30 {
        return 0;
    }
    let mut isucc = 0;
    let mut maxirest = 0;
    while isucc < reduced[ipos].successors.len() {
        let (inext, length) = reduced[ipos].successors[isucc];
        if inext != ipos && !states[inext] {
            states[inext] = true;
            maxirest = maxirest.max(explore(reduced, states, inext, time + length + 1));
            states[inext] = false;
        }
        isucc += 1;
    }
    (31 - time) * reduced[ipos].flow + maxirest
}

// Build reduced graph with relevant valves only
fn reduce(valves: &Vec<Valve>) -> Vec<Valve> {
    // Init the new graph. The successors will be updated later.
    let mut reduced: Vec<Valve> = valves
        .iter()
        .filter(|valve| valve.name == "AA" || valve.flow != 0)
        .enumerate()
        .map(|(index, valve)| Valve {
            index: index,
            oldindex: valve.index,
            flow: valve.flow,
            successors: vec![],
            name: valve.name.clone(),
        })
        .collect();
    // Create old to new index translator
    let mut old_to_new_index = HashMap::new();
    for valve in &reduced {
        old_to_new_index.insert(valve.oldindex, valve.index);
    }
    // Update the successors of the new graph
    for iorigin in 0..reduced.len() {
        let destinations = dijkstra_all(&reduced[iorigin].oldindex, |valve| {
            valves[*valve].successors.to_owned()
        });
        // Add all reached valve to the successors of origin
        for (ito, (_ipred, cost)) in &destinations {
            if !old_to_new_index.contains_key(ito) {
                continue;
            }
            reduced[iorigin]
                .successors
                .push((old_to_new_index[ito], *cost));
        }
    }
    // Sort successor from most to less relevant
    for i in 0..reduced.len() {
        let mut successors = reduced[i].successors.clone();
        successors.sort_by_key(|(succ, length)| length * reduced[*succ].flow);
        successors.reverse();
        reduced[i].successors = successors;
    }
    reduced
}
