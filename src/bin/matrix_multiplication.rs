use std::cell::RefCell;
use std::rc::Rc;
use rand;
use rand::Rng;
use std::time::Instant;

fn main() {
    const N: usize = 1200;
    const M: usize = 600;
    const K: usize = 2000;

    let mat_a= Rc::new(RefCell::new(vec!([0; M]; N)));
    let mat_b= Rc::new(RefCell::new(vec!([0; K]; M)));
    let mut mat_c = vec!([0; K]; N);
    let mut rng = rand::thread_rng();

    for i in 0..N {
        for j in 0..M {
            mat_a.borrow_mut()[i][j] = rng.gen_range(0..20)
        }
    }

    for i in 0..M {
        for j in 0..K {
            mat_b.borrow_mut()[i][j] = rng.gen_range(0..20)
        }
    }

    let start = Instant::now();
    let mat_a = mat_a.take();
    let mat_b = mat_b.take();

    for i in 0..N {
        for j in 0..K {
            for s in 0..M {
                mat_c[i][j] += mat_a[i][s] * mat_b[s][j];
            }
        }
    }

    let elapsed = start.elapsed();
    println!("Millis: {} ms", elapsed.as_millis());
}