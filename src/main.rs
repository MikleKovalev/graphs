mod graph;
mod counter;

use graph::Graph;
use counter::Counter;

fn main() {
    let content = match std::fs::read_to_string("/home/mikle/Code/test.txt") {
        Ok(content) => content,
        Err(_) => panic!("Can not read file with graphs!"),
    };
    let mut graph = Graph::with_size(0);
    let mut counter = Counter::new(0);
    let mut last_graph_size = 0;
    let mut answer = vec![vec![0; 11]; 11];
    let mut graphs_counter = 0;
    for graph6 in content.lines() {
        graphs_counter += 1;
        let mut chars = graph6.chars();
        let current_graph_size = (chars.nth(0).unwrap() as usize) - 63;
        if current_graph_size != last_graph_size {
            last_graph_size = current_graph_size;
            graph = Graph::with_size(current_graph_size);
            counter = Counter::new(current_graph_size);
        }
        if current_graph_size == 11 {
            break;
        }
        graph.initialize(&graph6);
        let cnt = counter.count(&graph);
        answer[cnt.0][cnt.1] += 1;
    }
    println!("{}", graphs_counter);
    for row in answer.iter() {
        for cell in row.iter() {
            print!("{}\t", cell);
        }
        println!();
        println!();
    }
}
