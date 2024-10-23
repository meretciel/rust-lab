
pub type Vector = Vec<f64>;
// pub type MultiVarFunction = Fn(&Vector) -> f64;
// pub trait MultiVarFunction : Fn(&Vector) -> f64 {}
// pub trait MultiVariableFunction = Fn(&Vector) -> f64;
// pub trait SingleVariableFunction = Fn(f64) -> f64;

pub fn add(v1: &Vector, v2: &Vector) -> Vector {
    let mut res = vec![0.0; v1.len()];
    for i in 0..v1.len() {
        res[i] = v1[i] + v2[i];
    }
    return res;
}

pub fn sub(v1: &Vector, v2: &Vector) -> Vector {
    let mut res = vec![0.0; v1.len()];
    for i in 0..v1.len() {
        res[i] = v1[i] - v2[i];
    }
    return res;
}

pub fn mul(k: f64, v: &Vector) -> Vector {
    let mut res = v.clone();
    for i in 0..res.len() {
        res[i] *= k;
    }
    return res;
}

pub fn dot(v1: &Vector, v2: &Vector) -> f64 {
    v1.iter().zip(v2).map(|(x1, x2)| { x1 * x2}).sum()
}

pub fn norm(v: &Vector) -> f64 {
    dot(&v, &v).sqrt()
}