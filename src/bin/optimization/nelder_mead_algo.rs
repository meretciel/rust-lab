use std::error::Error;
use std::fs::File;
use csv::{Writer, WriterBuilder};
use rand::{thread_rng, Rng};

type Vector = Vec<f64>;

fn add(v1: &Vec<f64>, v2: &Vec<f64>) -> Vec<f64> {
    let mut res = vec![0.0; v1.len()];
    for i in 0..v1.len() {
        res[i] = v1[i] + v2[i];
    }
    return res;
}

fn sub(v1: &Vec<f64>, v2: &Vec<f64>) -> Vec<f64> {
    let mut res = vec![0.0; v1.len()];
    for i in 0..v1.len() {
        res[i] = v1[i] - v2[i];
    }
    return res;
}

fn mul(k: f64, v: &Vector) -> Vector {
    let mut res = v.clone();
    for i in 0..res.len() {
        res[i] *= k;
    }
    return res;
}

struct Config {
    max_iteration: i32,
    alpha: f64,
    gamma: f64,
    rho: f64,
    sigma: f64,
}

impl Config {
    fn default() -> Config {
        return Config{
            max_iteration: 100,
            alpha: 1.0,
            gamma: 2.0,
            rho: 0.5,
            sigma: 0.5,
        }
    }
}


fn optimize<F>(func: F, init: Vec<Vec<f64>>, config: &Config, writer: &mut Writer<File>)
               -> Result<(Vec<f64>, f64), Box<dyn Error>>
where
    F: Fn(&Vec<f64>) -> f64,
{
    assert!(init.len() >= 2);
    let n = init.first().unwrap().len();
    assert_eq!(n + 1, init.len());
    let mut header = Vec::new();
    header.push("value".to_owned());
    for i in 1..=n {
        let s = format!("x{i}");
        header.push(s);
    }

    writer.write_record(&header)?;

    let mut points = init;
    let mut optimal_point = Vec::new();
    let mut optimal_value = 0.0;

    for iter in 0..config.max_iteration {
        points.sort_by(|a, b| {
            func(a).total_cmp(&func(b))
        });

        let best = points.first().unwrap();
        optimal_point = best.clone();
        let worst = points.last().unwrap();
        let second_worst = &points[n - 1];

        let best_value = func(best);
        optimal_value = best_value;
        let worst_value = func(worst);
        let second_worst_value = func(second_worst);

        writer.write_field(optimal_value.to_string())?;
        for item in best.iter() {
            writer.write_field(item.to_string())?
        }
        writer.write_record(None::<&[u8]>)?;

        let mut gc = vec![0.0; n];
        for k in 0..n {
            for i in 0..n {
                gc[i] += points[k][i];
            }
        }

        for k in 0..n {
            gc[k] /= n as f64;
        }

        println!("[{iter}] min value: {optimal_value} at {optimal_point:?} | gc={gc:?}, worst_values: {second_worst_value}, {worst_value}");

        let u = sub(&gc, worst);
        let b_reflect = add(&gc, &mul(config.alpha, &u));
        let new_value = func(&b_reflect);

        if new_value < second_worst_value {
            if new_value < best_value {
                // Expansion

                let b_expansion = add(&gc, &mul(config.gamma, &u));
                if func(&b_expansion) < new_value {
                    points[n] = b_expansion;
                } else {
                    points[n] = b_reflect;
                }
            } else {
                points[n] = b_reflect;
            }
        } else {
            // Contraction
            let b_contracted = add(&gc, &mul(config.rho, &u));
            if func(&b_contracted) < worst_value {
                points[n] = b_contracted;
            } else {
                for i in 1..=n {
                    let best = points.first().unwrap();
                    points[i] = add(best, &mul(config.sigma, &sub(&points[i], best)));
                }
            }
        }
    }

    return Ok((optimal_point, optimal_value));
}


fn main() -> Result<(), Box<dyn Error>> {
    let config = Config {
        max_iteration: 500,
        alpha: 1.0,
        gamma: 2.0,
        rho: 0.5,
        sigma: 0.5,
    };

    let mut writer = WriterBuilder::new().from_path("/home/ryan/workspace/tmp/nelder_mead_algo_data_8.csv")?;

    let mut rng = thread_rng();
    const N: usize = 2;
    let mut init = Vec::new();
    for i in 0..N+1 {
        init.push((0..N).map(|_x| rng.gen_range(-1.2..3.5)).collect());
    }

    optimize(|x| {
        100. * (x[1] - x[0] * x[0]).powi(2) + (1. - x[0]).powi(2)
    }, init, &config, &mut writer)?;

    return Ok(());
}