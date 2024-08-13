use std::marker::PhantomData;
use std::ops::Deref;

fn multivariate_func(params: &Vec<f64>) -> f64 {
    let x = params[0];
    let y = params[1];

    x * x - y * y
}

fn single_variable_func(xp: &f64) -> f64 {
    let x = *xp;
    x * x
}

trait Differentiable {
    type Args;
    fn gradient(&self, args: &Self::Args) -> Self::Args;
}

trait GradientCalculator<F, A> {
    fn gradient(&self, f: &F, x: &A) -> A;
}

struct SimpleGradientCalculator {
    h: f64
}

impl<F> GradientCalculator<F, Vec<f64>> for SimpleGradientCalculator
where F: Fn(&Vec<f64>) -> f64 {
    fn gradient(&self, f: &F, x: &Vec<f64>) -> Vec<f64> {
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

impl<F> GradientCalculator<F, f64> for SimpleGradientCalculator
where F: Fn(&f64) -> f64 {
    fn gradient(&self, f: &F, xp: &f64) -> f64 {
        let x = *xp;
        let x1 = x - self.h;
        let x2 = x + self.h;
        (f(&x2) - f(&x1)) / (2. * self.h)
    }
}



struct DifferentiableFunc<'a, F, G, A> {
    func: &'a F,
    gradient_calculator: G,
    phantom: PhantomData<A>,
}

// impl<'a, F, G> Differentiable for DifferentiableFunc<'a, F, G>
// where F: Fn(&Vec<f64>) -> f64,
//       G: GradientCalculator<F, Vec<f64>>  {
//     type Args = Vec<f64>;
//     fn gradient(&self, args: &Self::Args) -> Self::Args {
//         self.gradient_calculator.gradient(self.func, args)
//     }
// }

impl<'a, F, G, A> Differentiable for DifferentiableFunc<'a, F, G, A>
where F: Fn(&A) -> f64,
      G: GradientCalculator<F, A>  {
    type Args = A;
    fn gradient(&self, args: &Self::Args) -> Self::Args {
        self.gradient_calculator.gradient(self.func, args)
    }
}




impl<'a, F, G, A> Deref for DifferentiableFunc<'a, F, G, A>
where F: Fn(&A) -> f64 {
    type Target = F;

    fn deref(&self) -> &Self::Target {
        self.func
    }
}

fn main() {
    let f = DifferentiableFunc{
        func: &multivariate_func,
        gradient_calculator: SimpleGradientCalculator{h: 0.0001},
        phantom: PhantomData,
    };

    let g = DifferentiableFunc{
        func: &single_variable_func,
        gradient_calculator: SimpleGradientCalculator{h: 0.0001},
        phantom: PhantomData,
    };

    let x1 = vec![1., 2.];
    println!("value: {}, gradient: {:?}", f(&x1), f.gradient(&x1));
    let x2 = 3.;
    println!("value: {}, gradient: {:?}", g(&x2), g.gradient(&x2));
}