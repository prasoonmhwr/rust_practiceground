use std::collections::{HashMap, VecDeque};
use std::fs;
use std::io;


type DeviceGraph = HashMap<String, Vec<String>>;


fn parse_input(input: &str) -> DeviceGraph {
    let mut graph = DeviceGraph::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        
        let parts: Vec<&str> = line.split(": ").collect();
        if parts.len() != 2 {
            continue; 
        }

        let device = parts[0].trim().to_string();
        let outputs: Vec<String> = parts[1]
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        graph.insert(device, outputs);
    }
    graph
}


fn count_paths_recursive(
    graph: &DeviceGraph,
    current_device: &str,
    target_device: &str,
    memo: &mut HashMap<String, u64>,
) -> u64 {
    
    if let Some(&count) = memo.get(current_device) {
        return count;
    }

    
    if current_device == target_device {
        return 1;
    }

   
    let mut total_paths = 0;
    
    if let Some(outputs) = graph.get(current_device) {
        for next_device in outputs {
            total_paths += count_paths_recursive(
                graph,
                next_device,
                target_device,
                memo,
            );
        }
    }
    
    
    memo.insert(current_device.to_string(), total_paths);

    total_paths
}




fn get_paths(graph: &DeviceGraph, start: &str, end: &str) -> u64 {
    
    let mut memo = HashMap::new();
    count_paths_recursive(graph, start, end, &mut memo)
}


fn solve_part2(input: &str, start: &str, end: &str, node_a: &str, node_b: &str) -> u64 {
    let graph = parse_input(input);
    
    
    let paths_a1 = get_paths(&graph, start, node_a);   
    let paths_a2 = get_paths(&graph, node_a, node_b);  
    let paths_a3 = get_paths(&graph, node_b, end);     

    let scenario_a_total = paths_a1 * paths_a2 * paths_a3;
    
   
    let paths_b1 = get_paths(&graph, start, node_b);   
    let paths_b2 = get_paths(&graph, node_b, node_a);  
    let paths_b3 = get_paths(&graph, node_a, end);    
    
    let scenario_b_total = paths_b1 * paths_b2 * paths_b3;

   
    scenario_a_total + scenario_b_total
}


fn main() -> Result<(), io::Error> {
   
    const FILENAME: &str = "input_puzzle11.txt";
    const START_DEVICE: &str = "svr";
    const END_DEVICE: &str = "out";
    const NODE_A: &str = "dac";
    const NODE_B: &str = "fft";

    let puzzle_input = fs::read_to_string(FILENAME)?;


    let final_answer = solve_part2(
        &puzzle_input, 
        START_DEVICE, 
        END_DEVICE, 
        NODE_A, 
        NODE_B
    );

    println!(
        "The total number of paths from '{}' to '{}' that visit both '{}' and '{}' is: {}", 
        START_DEVICE, 
        END_DEVICE,
        NODE_A,
        NODE_B,
        final_answer
    );
    
    Ok(())
}