use std::collections::VecDeque;
use std::error::Error;
use csv::WriterBuilder;
use rand::{thread_rng, Rng};
use rust_lab::numerical_utils as nu;
use rust_lab::numerical_utils::{Vector};


fn banana_function(x: &Vector) -> f64 {
    100. * (x[1] - x[0] * x[0]).powi(2) + (1. - x[0]).powi(2)
}

fn line_search<F>(f: F, a: f64, b: f64, n: usize) -> f64
where
    F: Fn(f64) -> f64,
{

    let d: f64 = (b - a) / (n as f64);
    let optimal_point = (0..=n).map(|i| -> f64 { a + (i as f64) * d }).min_by(
        |x, y| {
            let res = f(*x).partial_cmp(&f(*y)).expect(format!("x={}, y={}, v1={}, v2={}, v3={}", x, y, f(*x), f(*y), f(0.1)).as_str());
            // println!("x={}, y={}, f(x)={}, f(y)={}, ordering={:?}", *x,  *y, f(*x), f(*y), res);
            res

        }
    ).unwrap();
    return optimal_point;
}

fn line_func_gen<'a, F>(f: &'a F, x_0: &'a Vector, u: &'a Vector) -> impl Fn(f64) -> f64 + 'a
where
    F: Fn(&Vector) -> f64
{
    |t| {
        let z = nu::add(x_0, &nu::mul(t, u));
        f(&z)
    }
}


fn optimizer_power<F>(f: F, init_x: &Vector, n_iterations: usize, search_radius: f64, n_buckets: usize, records: &mut Vec<Vec<String>>) -> (Vector, f64)
where
    F: Fn(&Vector) -> f64
{
    let mut curr_estimate = init_x.clone();
    let mut func_value = f(&curr_estimate);
    let mut n_buckets = n_buckets;
    let mut search_radius = search_radius;
    let input_n_buckets = n_buckets;
    let input_search_radius = search_radius;
    let max_rework = 10;
    let mut count = 0;
    let jitter = 1e-5;
    let mut rng = thread_rng();
    let mut to_increase_granularity = true;

    let n = init_x.len();
    let mut direction_sets = VecDeque::new();
    for i in 0..n {
        let mut v = vec![0.; n];
        v[i] = 1.;
        direction_sets.push_back(v);
    }


    for i in 0..n_iterations {
        let old_estimate = curr_estimate.clone();


        println!("iter={i}. starting point={old_estimate:?}, h={search_radius}, n_bucket={n_buckets}, count={count}");
        for k in 0..n {
            let u = nu::add(&direction_sets[k], &vec![rng.gen_range(-jitter..jitter), rng.gen_range(-jitter..jitter)]);
            println!("\tselected direction: {u:?}");

            let line_func = line_func_gen(&f, &curr_estimate, &u);
            let optimal_t = line_search(line_func, -search_radius, search_radius, 2 * n_buckets);
            println!("\toptimal_t={optimal_t}");
            curr_estimate =nu::add(&curr_estimate, &nu::mul(optimal_t, &u));
            println!("\tupdated point={curr_estimate:?}");
        }

        let d = nu::sub(&curr_estimate, &old_estimate);
        let norm_d = nu::norm(&d);

        if norm_d < 1e-6 {
            println!("Reached a local minimal.");
            if count >= max_rework {
                if to_increase_granularity {
                    to_increase_granularity = false;
                    count = 0; // reset the counter
                    search_radius = input_search_radius * 2.;
                    n_buckets = input_n_buckets * 2;


                } else {
                    println!("Terminate the iteration early.");
                    let value = f(&curr_estimate);
                    return (curr_estimate, value);
                }

            } else {
                count += 1;
                if to_increase_granularity {
                    search_radius *= 0.8333;
                } else {
                    search_radius *= 1.2;
                }
                n_buckets = (n_buckets * 2).min(5000);

            }

            if norm_d == 0. {
                continue;
            }

        } else {
            count = 0;
            if to_increase_granularity {
                search_radius = input_search_radius.min(search_radius * 1.2);
                n_buckets = input_n_buckets.max(n_buckets / 2);

            } else {
                search_radius = input_search_radius;
                n_buckets = input_n_buckets;
                to_increase_granularity = true;
            }
        }
        
        let v = nu::mul(1. / norm_d, &d);
        let optimal_t = line_search(line_func_gen(&f, &curr_estimate, &v), -search_radius, search_radius, 2 * n_buckets);
        curr_estimate = nu::add(&curr_estimate, &nu::mul(optimal_t, &v));

        direction_sets.pop_front().unwrap();
        direction_sets.push_back(v);
        func_value = f(&curr_estimate);

        let mut row = Vec::new();
        curr_estimate.iter().for_each(|x| { row.push(x.to_string()) });
        row.push(func_value.to_string());
        records.push(row);
        println!("value={func_value}, point={curr_estimate:?}")
    }

    return (curr_estimate, func_value)
}


fn main() -> Result<(), Box<dyn Error>> {
    for tt in 100..121 {
        const N: usize = 2;
        let mut rng = thread_rng();
        let init_guess = (0..N).map(|_x| {rng.gen_range(-1.2..3.5)}).collect();
        let n_iteration = 100;
        let search_radius= 0.5;
        let n_bucket = 30;
        let mut records: Vec<Vec<String>> = Vec::new();
        optimizer_power(
            banana_function, &init_guess, n_iteration, search_radius, n_bucket, &mut records
        );
        let mut writer = WriterBuilder::new().from_path(format!("/home/ryan/workspace/tmp/powell_algo_v{tt}.csv"))?;
        let _ = writer.write_record(&["x1", "x2", "best_value"]);
        records.iter().for_each(|v| {writer.write_record(v).unwrap()});
    }

    Ok(())
}


#[cfg(test)]
mod test {
    use super::*;
    use rust_lab::compare_utils::{assert_eq_f64};

    #[test]
    fn test_line_search() {
        let f = |x| { (x - 1.) * (x) };
        let res = line_search(f, -1., 1., 100);
        assert_eq_f64(res, 0.5, 1e-4);
    }
}