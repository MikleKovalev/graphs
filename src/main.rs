mod graph;
mod counter;

use graph::Graph;
use counter::Counter;

use std::io::Write;

fn main() {
    let content = match std::fs::read_to_string("/home/mikle/Code/test-7.txt") {
        Ok(content) => content,
        Err(_) => panic!("Can not read file with graphs!"),
    };
    let mut graph = Graph::with_size(0);
    let mut counter = Counter::new(0);
    let mut last_graph_size = 0;
    let mut answer = vec![vec![0; 12]; 12];
    for graph6 in content.lines() {
        let mut chars = graph6.chars();
		let current_graph_size = (chars.nth(0).unwrap() as usize) - 63;
		if current_graph_size != last_graph_size {
			last_graph_size = current_graph_size;
			graph = Graph::with_size(current_graph_size);
			counter = Counter::new(current_graph_size);
		}
        graph.initialize(&graph6);
        let cnt = counter.count(&graph);
        answer[cnt.0][cnt.1] += 1;
    }
	let mut file = std::fs::File::create("/home/mikle/Code/result-7.txt").unwrap();
    for i in 1..answer.len() {
		for j in 1..answer.len() {
			write!(&mut file, "{}\t", &answer[i][j]);
			print!("{}\t", &answer[i][j]);
		}
		writeln!(&mut file, "\n");
		println!();
	}
}
