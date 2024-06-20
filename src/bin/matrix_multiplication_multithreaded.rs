use std::sync::Arc;
use std::thread;
use rand;
use rand::{Rng, SeedableRng};
use std::time::Instant;
use rand::rngs::StdRng;
use rust_lab::io_utils::save_matrix_i32;

fn main() {
    const N: usize = 1200;
    const M: usize = 600;
    const K: usize = 2000;

    let mut mat_a= vec!(vec!(0; M); N);
    let mut mat_b= vec!(vec!(0; K); M);
    let mut mat_c = vec!(vec!(0; K); N);
    let mut rng = StdRng::seed_from_u64(42);
    // let mut rng = rand::thread_rng();

    for i in 0..N {
        for j in 0..M {
            mat_a[i][j] = rng.gen_range(0..20)
        }
    }

    for i in 0..M {
        for j in 0..K {
            mat_b[i][j] = rng.gen_range(0..20)
        }
    }

    let mat_a = Arc::new(mat_a);
    let mat_b = Arc::new(mat_b);
    let start = Instant::now();
    let num_threads = 8;
    let mut handlers = vec!();

    for k in 0..num_threads {
        let ma = mat_a.clone();
        let mb = mat_b.clone();

        let handler = thread::spawn(move || {
            let n_rows = (N as f64 / num_threads as f64).ceil() as usize;
            let mut mc = vec!(vec!(0; K); n_rows);
            let mut index = 0;
            for i in 0..N {
                if i % num_threads == k {
                    for j in 0..K {
                        for s in 0..M {
                            mc[index][j] += ma[i][s] * mb[s][j];
                        }
                    }
                    index += 1;
                }
            }
            mc
        });

        handlers.push(handler);
    }

    for (k, item) in handlers.into_iter().enumerate() {
        for (i, v) in item.join().unwrap().into_iter().enumerate() {
            let row_number = k + i * num_threads;
            mat_c[row_number] = v;
        }
    }

    let elapsed = start.elapsed();
    println!("Millis: {} ms", elapsed.as_millis());

    save_matrix_i32(&mat_c, "/home/ryan/workspace/tmp/rust_output/matrix_multiplication_multithreaded_output.csv")
}