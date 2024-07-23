use std::fs::File;
use libm::{exp, sqrt, log};
use rand::distributions::WeightedIndex;
use rand::rngs::ThreadRng;
use rand::{Rng, thread_rng};
use rand::distributions::Distribution;
use std::io::{BufWriter, Write};

struct MultiplicativeWeightsUpdateAlgo {
    n_experts: usize,
    n_steps: usize,
    epsilon: f64,
    weights: Vec<f64>,
    rng: ThreadRng,
}

impl MultiplicativeWeightsUpdateAlgo {
    fn new(n_experts: usize, n_steps: usize) -> MultiplicativeWeightsUpdateAlgo {
        return MultiplicativeWeightsUpdateAlgo{
            n_experts,
            n_steps,
            epsilon: sqrt(log(n_experts as f64) / (n_steps as f64)),
            weights: vec![1.; n_experts],
            rng: thread_rng()
        };
    }

    fn update_weights(&mut self, loss: &Vec<f64>) {
        for i in 0..self.n_experts {
            self.weights[i] = self.weights[i] * exp(-self.epsilon * loss[i]);
        }
    }

    fn produce_answer(&mut self, expert_answers: &Vec<f64>) -> f64 {
        let dist = WeightedIndex::new(&self.weights).unwrap();
        return expert_answers[dist.sample(&mut self.rng)]
    }

    fn select_random_answer(&mut self, expert_answers: &Vec<f64>) -> f64 {
        return expert_answers[self.rng.gen_range(0..self.n_experts)];
    }
}


fn main() {
    let n = 15;
    let T = 80;
    let num_experiments = 600;
    let suffix = "simulation_n15t80exp600_lucky";
    let output_file = format!("/home/ryan/workspace/tmp/rust_output/multiplicative_weights_update_{}.txt", suffix);

    let mut rng = thread_rng();

    let mut result = Vec::new();

    for exp in 0..num_experiments {
        println!(">>> experiment {}", exp);

        let mut mwv = MultiplicativeWeightsUpdateAlgo::new(n, T);
        let mut experts_error_counts = vec![0; n];
        let mut mwv_error_counts = 0;
        let mut random_answer_error_counts = 0;



        for t in 1..=T {
            let mut expert_answers: Vec<f64> = (0..n).map(|x| rng.gen_range(0..2) as f64).collect();
            let correct_answer = rng.gen_range(0..2) as f64;

            if rng.gen_range(0.0..1.0) < 0.8 {
                expert_answers[0] = correct_answer;
            }

            let mwv_answer = mwv.produce_answer(&expert_answers);
            let random_answer = mwv.select_random_answer(&expert_answers);


            let mut loss = vec![0.0; n];

            for i in 0..n {
                if expert_answers[i] != correct_answer {
                    experts_error_counts[i] += 1;
                    loss[i] = 1.0;
                }
            }

            if mwv_answer != correct_answer {
                mwv_error_counts += 1;
            }

            if random_answer != correct_answer {
                random_answer_error_counts += 1;
            }


            mwv.update_weights(&loss);
        }

        let mut min_error = i32::MAX;

        for i in 0..n {
            min_error = i32::min(min_error, experts_error_counts[i]);
        }
        println!("min error: {}, MWV error: {}, random error: {}", min_error, mwv_error_counts, random_answer_error_counts);
        result.push((min_error, mwv_error_counts, random_answer_error_counts));
    }




    let mut writer =
        BufWriter::new(File::create(output_file).expect("Failed to create the output file"));

    writer.write("ErrorCountOfBestExpert,ErrorCountOfMwv,ErrorCountOfRandomSelection\n".as_bytes());
    for i in 0..num_experiments {
        let (a, b, c) = result[i];
        writer.write(format!("{},{},{}\n", a, b, c).as_bytes()).unwrap();
    }
}