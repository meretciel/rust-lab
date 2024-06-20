use std::cell::RefCell;
use std::rc::Rc;
use rand;
use rand::{Rng, SeedableRng};
use std::time::Instant;
use rand::prelude::StdRng;
use rust_lab::io_utils::save_matrix_i32;

fn main() {
    const N: usize = 1200;
    const M: usize = 600;
    const K: usize = 2000;

    let mut mat_a= vec!(vec!(0; M); N);
    let mut mat_b= vec!(vec!(0; K); M);
    let mut mat_c = vec!(vec!(0; K); N);
    let mut rng = StdRng::seed_from_u64(42);

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

    let start = Instant::now();

    for i in 0..N {
        for j in 0..K {
            for s in 0..M {
                mat_c[i][j] += mat_a[i][s] * mat_b[s][j];
            }
        }
    }

    let elapsed = start.elapsed();
    println!("Millis: {} ms", elapsed.as_millis());

    save_matrix_i32(&mat_c, "/home/ryan/workspace/tmp/rust_output/matrix_multiplication_simple_output.csv")
}