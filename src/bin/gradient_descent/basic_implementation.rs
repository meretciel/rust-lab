use std::cell::RefCell;
use std::fs::File;
use std::io::Write;
use std::rc::Rc;
use rand::Rng;

#[derive(Debug)]
enum SolverError {
    NoSolutionFound,
    ExceedRangeDuringSearch(f64, f64),
}

struct GradientCalculator {
    h: f64
}

impl GradientCalculator {
    fn gradient<F>(&self, f: &F, x: f64) -> f64
    where F: Fn(f64) -> f64 {
        (f(x + self.h) - f(x - self.h)) / ( 2. * self.h)
    }

    fn gradient_multi<F>(&self, f: &F, x: &Vec<f64>) -> Vec<f64>
    where F: Fn(&Vec<f64>) -> f64 {
        let mut x1 = x.clone();
        let mut x2 = x.clone();
        let n = x.len();
        let mut res = vec![0.; n];

        for i in 0..n {
            x1[i] -= self.h;
            x2[i] += self.h;
            res[i] = (f(&x2) - f(&x1)) / (2. * self.h);
            x1[i] = x[i];
            x2[i] = x[i];
        }
        return res;
    }
}

struct GradientDescentAlgo {
    gradient_calculator: GradientCalculator,
    writer: Rc<RefCell<File>>
}

impl GradientDescentAlgo {
    fn run_multi<F>(&self, f: &F, init: Vec<f64>, n: usize, beta_1: f64, beta_2: f64, eta: f64, epsilon: f64) -> Vec<f64>
    where F: Fn(&Vec<f64>) -> f64  {
        let d = init.len();
        let mut m = vec![0.; d];
        let mut v = vec![0.; d];
        let mut beta_1_cum = 1.;
        let mut beta_2_cum = 1.;
        let mut x = init;

        for k in 0..n {
            let value = f(&x);
            println!("{:?},{}", x, value);
            self.writer.borrow_mut().write_all(format!("{},{},{}\n", x[0], x[1], value).as_bytes()).unwrap();
            beta_1_cum *= beta_1;
            beta_2_cum *= beta_2;
            let g = self.gradient_calculator.gradient_multi(f, &x);

            for i in 0..d {
                m[i] = beta_1 * m[i] + (1. - beta_1) * g[i];
                v[i] = beta_2 * v[i] + (1. - beta_2) * g[i] * g[i];
                let mh = m[i] / (1. - beta_1_cum);
                let vh = v[i] / (1. - beta_2_cum);
                x[i] = x[i] - (eta / (f64::sqrt(vh) + epsilon)) * mh;
            }
        }
        return x;
    }
}


fn my_func(params: &Vec<f64>) -> f64 {
    let x = params[0];
    let y = params[1];

    x * x - y * y + f64::sin(x) - f64::sin(y * y) - 0.2 * x + f64::powi(y, 4)
}


fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {

    for k in 0..100 {
        println!("=================================");
        let mut rng = rand::thread_rng();
        let x: f64 = rng.gen_range(-1.0..1.0);
        let y: f64 = rng.gen_range(-1.0..1.0);

        let mut writer = File::create(format!("/home/ryan/workspace/tmp/gradient_descent_output_{k}.csv"))?;
        writer.write_all(b"x,y,value\n")?;

        let writer = Rc::new(RefCell::new(writer));
        let gradient_calculator = GradientCalculator{h:1e-4};
        let gd_algo = GradientDescentAlgo{
            gradient_calculator,
            writer,
        };
        let res = gd_algo.run_multi(
            &my_func,
            vec![x, y],
            100,
            0.9,
            0.999,
            0.1,
            1e-8
        );
        println!("optimal parameter is {res:?}");
    }

    Ok(())
}


