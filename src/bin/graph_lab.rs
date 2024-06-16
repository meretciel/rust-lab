
use std::fs::File;
use std::io::{self, BufRead, BufWriter, Write};

type AdjMatrix = Vec<Vec<i32>>;

struct Graph {
    n: usize,
    curr_t: i32,
    neighbors: AdjMatrix,
    low_points: Vec<i32>,
    visit_time: Vec<i32>,
}


impl Graph {
    fn from_adjacent_matrix(neighbors: AdjMatrix) -> Graph {
        let n = neighbors.len();
        Graph{
            n: n,
            curr_t: 0,
            neighbors: neighbors,
            low_points: vec![0; n],
            visit_time: vec![0; n]
        }
    }
}


fn main() {
    let file = File::open("/home/ryan/workspace/tmp/graph_data/graph_input.txt")
        .expect("Failed to open the file.");
    let output_file = File::create("/home/ryan/workspace/tmp/graph_data/output_1.txt")
        .expect("Unable to create the file.");
    let mut write_buffer = BufWriter::new(output_file);

    let mut lines = io::BufReader::new(file).lines();
    let first_line = lines.next().unwrap().unwrap();

    let n = first_line.parse().unwrap();
    let mut neighbors: Vec<Vec<i32>> = vec![Vec::new(); n+1];
    for i in 1..=n {
        let line = lines.next().unwrap().expect("Failed to read the line");
        let parts = line.split(",");
        let node = &mut neighbors[i];
        for item in parts {
            node.push(item.parse().unwrap())
        }
    }


    let graph = Graph::from_adjacent_matrix(neighbors);



    // for i in 1..=n {
    //     let node = &neighbors[i];
    //     for item in node {
    //         write_buffer.write(item.to_string().as_bytes()).expect("Failed to write to the file.");
    //         write_buffer.write(b" ").expect("Failed to write to the file.");
    //     }
    //     write_buffer.write(b"\n").expect("Failed to write to the file.");
    // }
}