
use std::fs::File;
use std::io::{self, BufRead, BufWriter, Write};
use std::path::Path;
use std::cmp::min;
use std::ptr::write;


type AdjList = Vec<Vec<usize>>;

#[derive(Eq, PartialEq)]
struct Edge {
    start: usize,
    end: usize,
}

struct Graph {
    n: usize,
    curr_t: i32,
    output_counter: i32,
    neighbors: AdjList,
    parents: Vec<usize>,
    low_points: Vec<i32>,
    visit_time: Vec<i32>,
    edge_stack: Vec<Edge>,
    colors: Vec<String>,
    visited_edges: Vec<Edge>,
}


impl Graph {
    const COLOR_VISITED_NODE: &'static str = "#c1fbd2";
    const COLOR_PENDING_NODE: &'static str = "#def8fb";

    const COLOR_NEW_NODE: &'static str = "white";
    const COLOR_HIGHLIGHTED_EDGE: &'static str = "orange";

    const WIDTH_HIGHLIGHTED_EDGE: &'static str = "2";
    const WIDTH_NORMAL_EDGE: &'static str = "0.5";

    fn from_adjacent_matrix(neighbors: AdjList) -> Graph {
        let n = neighbors.len();
        Graph{
            n: n-1,
            curr_t: 0,
            output_counter: 0,
            neighbors: neighbors,
            parents: vec![0; n],
            low_points: vec![0; n],
            visit_time: vec![0; n],
            edge_stack: Vec::new(),
            colors: vec![Graph::COLOR_NEW_NODE.to_string(); n],
            visited_edges: Vec::new(),
        }
    }

    fn reset(&mut self) {
        self.curr_t = 0;
        self.parents = vec![0; self.n];
        self.low_points = vec![0; self.n];
        self.visit_time = vec![0; self.n];
        self.edge_stack = Vec::new();
        self.colors = vec![Graph::COLOR_NEW_NODE.to_string(); self.n];
        self.visited_edges = Vec::new();
    }

    fn generate_visual(&self, suffix: &str, highlighted_edges: Vec<Edge>) {
        let output_file = format!("/home/ryan/workspace/tmp/graph_data/output_{}.txt", suffix);
        let mut writer =
            BufWriter::new(File::create(output_file).expect("Failed to create the output file"));

        writer.write("strict Graph {\n".as_bytes()).expect("Failed to write the output");
        writer.write("\tnode[shape=\"circle\", style=\"filled\", fillcolor=\"white\"];\n".as_bytes())
            .expect("Failed to write the output");

        // Create nodes
        for i in 1..=self.n {
            let content = format!("\tx{k}[label=\"{k}\", fillcolor=\"{color}\"];\n", k=i, color=self.colors[i]);
            writer.write(content.as_bytes()).expect("Failed to write node data.");
        }

        // Create edges
        for u in 1..=self.n {
            for &v in self.neighbors[u].iter() {
                let content = format!("\tx{} -- x{} [penwidth=\"{}\"];\n", u, v, Graph::WIDTH_NORMAL_EDGE);
                writer.write(content.as_bytes()).expect("Failed to write edge data.");
            }
        }

        for edge in self.visited_edges.iter() {
            let content = format!("\tx{} -- x{} [penwidth=\"{}\"];\n", edge.start, edge.end, Graph::WIDTH_HIGHLIGHTED_EDGE);
            writer.write(content.as_bytes()).expect("Failed to write edge data");
        }

        for Edge{start:u, end:v} in highlighted_edges {
            let content = format!("\tx{} -- x{} [penwidth=\"{}\", color=\"{}\"];\n", u, v,
                                  Graph::WIDTH_HIGHLIGHTED_EDGE,
                                  Graph::COLOR_HIGHLIGHTED_EDGE
            );
            writer.write(content.as_bytes()).expect("Failed to write highlighted edge data.");
        }

        writer.write("}".as_bytes()).expect("Failed to write to the output file.");
    }

    fn build_dsf_tree(&mut self, root: usize) {
        self.low_points[root] = 1;
        self.generate_visual("0", Vec::new());
        self.build_dsf_tree_helper(root, 0);
        self.output_counter += 1;
        self.generate_visual(self.output_counter.to_string().as_str(), Vec::new());
    }

    fn build_dsf_tree_helper(&mut self, curr: usize, prev: usize) {
        self.curr_t += 1;
        self.visit_time[curr] = self.curr_t;
        self.colors[curr] = Graph::COLOR_PENDING_NODE.to_string();

        self.output_counter += 1;
        self.generate_visual(self.output_counter.to_string().as_str(), Vec::new());

        for next_node in self.neighbors[curr].to_owned(){
            if self.visit_time[next_node] == 0 {
                println!("Follow the edge {} -> {}", curr, next_node);

                self.parents[next_node] = curr;
                self.edge_stack.push(Edge{start: curr, end: next_node});
                self.visited_edges.push(Edge{start: curr, end: next_node});

                self.low_points[next_node] = self.visit_time[curr];

                self.build_dsf_tree_helper(next_node, curr);

                self.low_points[curr] = min(self.low_points[curr], self.low_points[next_node]);

                if self.low_points[next_node] == self.visit_time[curr] {
                    let mut block_edges = Vec::new();
                    while let Some(edge) = self.edge_stack.pop() {
                        let u = edge.start;
                        let v = edge.end;
                        block_edges.push(edge);

                        if u == curr && v == next_node {
                            break;
                        } else {
                            println!("Block edge {} -> {}", u, v);
                        }
                    }
                    self.output_counter += 1;
                    self.generate_visual(self.output_counter.to_string().as_str(), block_edges);
                }
            } else if next_node != prev && self.visit_time[next_node] < self.visit_time[curr] {
                self.edge_stack.push(Edge{start: curr, end: next_node});
                self.low_points[curr] = min(self.low_points[curr], self.visit_time[next_node]);
            }
        }

        self.colors[curr] = Graph::COLOR_VISITED_NODE.to_string();
    }
}


fn main() {
    let file = File::open("/home/ryan/workspace/tmp/graph_data/graph_5.txt")
        .expect("Failed to open the file.");

    let mut lines = io::BufReader::new(file).lines();
    let first_line = lines.next().unwrap().unwrap();

    let n = first_line.parse().unwrap();
    let mut neighbors: Vec<Vec<usize>> = vec![Vec::new(); n+1];
    for i in 1..=n {
        let line = lines.next().unwrap().expect("Failed to read the line");
        let parts = line.split(",");
        let node = &mut neighbors[i];
        for item in parts {
            node.push(item.parse().unwrap())
        }
    }

    let mut graph = Graph::from_adjacent_matrix(neighbors);
    graph.build_dsf_tree(1);



}