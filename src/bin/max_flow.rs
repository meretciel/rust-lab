use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::rc::Rc;


const HL_NODE_COLOR: &str = "#def8fb";
const HL_NODE_COLOR_SOURCE_SET: &str = "#def8fb";
const HL_NODE_COLOR_SINK_SET: &str = "#E6E6FA";
const HL_EDGE_COLOR_FORWARD: &str = "#66CDAA";
const HL_EDGE_COLOR_REVERSED: &str = "#f5aa68";

#[derive(Debug)]
struct Edge {
    start: usize,
    end: usize,
    capacity: f64,
    flow: f64,
}

#[derive(Clone)]
struct ResidualGraphEdge {
    edge: Rc<RefCell<Edge>>,
    is_reversed: bool,

}

impl ResidualGraphEdge {
    fn get_residual_value(&self) -> f64 {
        if self.is_reversed {
            return self.edge.borrow().flow;
        } else {
            let e = self.edge.borrow();
            return e.capacity - e.flow;
        }
    }

    fn get_start(&self) -> usize {
        if !self.is_reversed {
            return self.edge.borrow().start;
        } else {
            return self.edge.borrow().end;
        }
    }

    fn get_end(&self) -> usize {
        if !self.is_reversed {
            return self.edge.borrow().end;
        } else {
            return self.edge.borrow().start;
        }
    }
}

struct Record {
    node: usize,
    residual: f64,
    level: i64,
}

struct VisualInfo {
    iteration: i64,
    bfs_step: i64,
    hl_nodes: HashMap<usize, &'static str>,
    hl_edges: HashMap<(usize, usize), &'static str>,
}

impl VisualInfo {
    fn new(iteration: i64) -> VisualInfo {
        VisualInfo{
            iteration,
            bfs_step: 0,
            hl_nodes: HashMap::new(),
            hl_edges: HashMap::new(),
        }
    }
}

struct Graph {
    n: usize,
    flow_value: f64,
    prev_nodes: Vec<Option<ResidualGraphEdge>>,
    neighbors: Vec<Vec<ResidualGraphEdge>>,
    visual_info: VisualInfo,
    output_dir: &'static Path,
}

impl Graph {

    fn new(n: usize, output_dir: &'static Path) -> Graph {
        Graph {
            n,
            flow_value: 0.0,
            prev_nodes: vec![None; n+1],
            neighbors: vec![Vec::new(); n+1],
            visual_info: VisualInfo::new(0),
            output_dir,
        }
    }

    /// Add a directional edge
    fn add_edge(&mut self, start: usize, end: usize, capacity: f64) {
        let edge = Rc::new(RefCell::new(Edge{start, end, capacity, flow: 0.0}));

        self.neighbors[start].push(ResidualGraphEdge{edge: edge.clone(), is_reversed: false});
        // self.neighbors[start].push(ResidualGraphEdge{edge: edge.clone(), on_residual_graph: true, is_reversed: false});

        self.neighbors[end].push(ResidualGraphEdge{edge: edge.clone(), is_reversed: true});
        // self.neighbors[end].push(ResidualGraphEdge{edge: edge.clone(), on_residual_graph: true, is_reversed: true});
    }

    fn pre_iteration_reset(&mut self) {
        for i in 0..self.prev_nodes.len() {
            self.prev_nodes[i] = None;
        }
    }

    fn generate_graph_file(&self) {
        println!("Generate graph file: iteration={}, bfs_step={}", self.visual_info.iteration, self.visual_info.bfs_step);

        let suffix = format!("{}.{}", self.visual_info.iteration, self.visual_info.bfs_step);
        let output_file = self.output_dir.join(format!("output_{}.txt", suffix));

        let mut writer =
            BufWriter::new(File::create(output_file).expect("Failed to create the output file"));

        writer.write("strict digraph {\n".as_bytes()).expect("Failed to write the output");
        writer.write("\tnode[shape=\"circle\", style=\"filled\", fillcolor=\"white\"];\n".as_bytes())
            .expect("Failed to write the output");
        writer.write("rankdir=LR;\n".as_bytes()).unwrap();
        writer.write("{rank=same; x2;x3;}\n".as_bytes()).unwrap();
        writer.write("{rank=same; x4;x5;}\n".as_bytes()).unwrap();

        // Create nodes
        for i in 1..=self.n {
            let mut color = "white";

            if self.visual_info.hl_nodes.contains_key(&i) {
                color = self.visual_info.hl_nodes.get(&i).unwrap();
            }

            let content = format!("\tx{k}[label=\"{k}\", fillcolor=\"{color}\"];\n", k=i, color=color);
            writer.write(content.as_bytes()).expect("Failed to write node data.");
        }

        // Create edges
        for start in 1..=self.n {
            for v in self.neighbors[start].iter() {
                if !v.is_reversed {
                    let end = v.edge.borrow().end;
                    let color = self.visual_info.hl_edges.get(&(start, end)).unwrap_or(&"black");
                    let label = format!("{}[{}]", v.edge.borrow().capacity, v.edge.borrow().flow);
                    let content = format!("\tx{} -> x{} [label=\"{}\", color=\"{}\"];\n", start, end, label, color);
                    writer.write(content.as_bytes()).expect("Failed to write edge data.");
                }
            }
        }

        writer.write("}".as_bytes()).expect("Failed to write to the output file.");
    }

    fn max_flow(&mut self, source: usize, sink: usize) -> f64 {
        println!("Calculate the max flow for source={}, sink={}", source, sink);

        let mut iteration = 0;
        loop {
            iteration += 1;
            self.visual_info = VisualInfo::new(iteration);

            self.pre_iteration_reset();
            let mut augmentation_path_exists = false;
            let mut augmentation_residual = 0.0;

            // Search for an augmentation path
            let mut queue = VecDeque::from([Record{node: source, residual: f64::MAX, level: 0}]);
            let mut visited = HashSet::new();
            visited.insert(source);

            let mut watermark = 1;

            println!("Iteration {}. Start the BFS.", iteration);

            while !queue.is_empty() && !augmentation_path_exists {
                let head = queue.pop_front().unwrap();

                // For visualization
                if head.level >= watermark {
                    self.visual_info.bfs_step = watermark;
                    watermark += 1;
                    self.generate_graph_file();
                    self.visual_info = VisualInfo::new(iteration);
                }

                for res_graph_edge in self.neighbors[head.node].iter() {
                    let next_node = res_graph_edge.get_end();
                    if visited.contains(&next_node) {
                        continue;
                    }

                    let residual = res_graph_edge.get_residual_value();
                    if residual > 0.0 {
                        println!("Find an adjacent node with positive residual: {:?}, current level: {}", res_graph_edge.edge.borrow(), head.level);
                        visited.insert(next_node);

                        let raw_edge = res_graph_edge.edge.borrow();
                        self.visual_info.hl_edges.insert((raw_edge.start, raw_edge.end), {
                            if res_graph_edge.is_reversed {
                                HL_EDGE_COLOR_REVERSED
                            } else {
                                HL_EDGE_COLOR_FORWARD
                            }
                        });

                        let new_residual = f64::min(residual, head.residual);
                        self.prev_nodes[next_node] = Some(res_graph_edge.clone());

                        if next_node == sink {
                            println!("Found an augmentation path with residual: {}", new_residual);

                            augmentation_path_exists = true;
                            augmentation_residual = new_residual;

                            self.visual_info.hl_nodes.insert(sink, HL_NODE_COLOR);
                            self.visual_info.bfs_step = watermark;
                            self.generate_graph_file();
                            self.visual_info = VisualInfo::new(iteration);

                            break;
                        } else {
                            queue.push_back(Record{node: next_node, residual: new_residual, level: head.level + 1});
                            self.visual_info.hl_nodes.insert(next_node, HL_NODE_COLOR);
                        }
                    }
                }
            }

            if augmentation_path_exists {
                // Update the flow value
                self.flow_value += augmentation_residual;
                let mut curr = sink;
                // Update the flow graph, starting from the sink.
                let mut hl_edges = HashMap::new();

                println!("Construct the augmentation path");
                while let Some(residual_graph_edge) = &self.prev_nodes[curr] {
                    let end = curr;
                    curr = residual_graph_edge.get_start();
                    let mut edge = residual_graph_edge.edge.borrow_mut();

                    if residual_graph_edge.is_reversed {
                        edge.flow -= augmentation_residual;
                        hl_edges.insert((curr, end), HL_EDGE_COLOR_REVERSED);
                    } else {
                        edge.flow += augmentation_residual;
                        hl_edges.insert((curr, end), HL_EDGE_COLOR_FORWARD);
                    }

                    println!("{} -> {} | {:?}", curr, end, edge);
                }

                self.visual_info.hl_edges = hl_edges;
                self.generate_graph_file();

            } else {
                println!("No more augmentation path. Reached the max flow value: {}", self.flow_value);
                self.visual_info = VisualInfo::new(iteration);
                self.visual_info.iteration = 999;

                for node in 0..=self.n {
                    self.visual_info.hl_nodes.insert(node, HL_NODE_COLOR_SINK_SET);
                }

                for node in visited {
                    self.visual_info.hl_nodes.insert(node, HL_NODE_COLOR_SOURCE_SET);
                }
                self.generate_graph_file();
                break;
            }

        }

        return self.flow_value;
    }
}

fn main() {
    let path = Path::new("/home/ryan/workspace/tmp/graph_data/max_flow/exp_3_large");
    if !path.exists() {
        std::fs::create_dir(path).expect("Failed to create the directory.");
    }

    let mut graph = Graph::new(8, path);
    graph.add_edge(1, 2, 8.);
    graph.add_edge(1, 3, 9.);
    graph.add_edge(1, 4, 7.);
    graph.add_edge(2, 5, 2.);
    graph.add_edge(2, 6, 6.);
    graph.add_edge(3, 5, 4.);
    graph.add_edge(3, 6, 6.);
    graph.add_edge(3, 7, 4.);
    graph.add_edge(4, 6, 1.);
    graph.add_edge(4, 7, 5.);
    graph.add_edge(5, 8, 8.);
    graph.add_edge(6, 8, 7.);
    graph.add_edge(7, 8, 9.);

    graph.max_flow(1, 8);



    // let mut graph = Graph::new(6, path);
    // graph.add_edge(1, 2, 16.);
    // graph.add_edge(1, 3, 13.);
    // graph.add_edge(2, 4, 12.);
    // graph.add_edge(3, 2, 4.);
    // graph.add_edge(4, 3, 9.);
    // graph.add_edge(3, 5, 14.);
    // graph.add_edge(5, 4, 7.);
    // graph.add_edge(4, 6, 20.);
    // graph.add_edge(5, 6, 4.);
    //
    // graph.max_flow(1,6);
}