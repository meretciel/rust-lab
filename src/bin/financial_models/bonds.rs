
use nalgebra::{DMatrix, DVector, OVector, Dyn};

#[derive(Debug)]
struct BondPricing {
    price: f64,
    duration: f64
}


fn calculate_bond_pricing(face_value: f64, maturity: usize, coupon_rate: f64, discount_rate: f64)
    -> BondPricing {
    let mut discount_factor = 1.;
    let mut price = 0.;
    let coupon_payment = face_value * coupon_rate;

    let mut duration = 0.;

    for i in 0..maturity {
        discount_factor *= 1. + discount_rate;
        let discounted_payment =  coupon_payment / discount_factor;
        price += discounted_payment;
        duration += discounted_payment * ((i + 1) as f64)
    }

    // Add principal payment
    price += face_value / discount_factor;
    duration += face_value * (maturity as f64) / discount_factor;
    duration /= price;

    return BondPricing{
        price,
        duration
    }
}

fn calculate_spot_interest_rate(market_prices: Vec<f64>, face_value: f64, coupon_rate: f64)
    -> Vec<f64> {
    let n = market_prices.len();
    let mut res = Vec::new();
    let mut offset = 0.;
    let single_coupon_payment = face_value * coupon_rate;
    for i in 0..n {
        let left_side = market_prices[i] - offset;
        let exponent = i as i32 + 1;

        let spot_rate = (face_value * (1. + coupon_rate) / left_side).powf( 1. / exponent as f64) - 1.;
        offset += single_coupon_payment / (1. + spot_rate).powi(exponent);
        res.push(spot_rate);
    }
    return res;
}

fn calculate_forward_rate(spot_rates: &Vec<f64>) -> Vec<f64> {
    let mut res = Vec::new();

    for i in 0..spot_rates.len() {
        let forward_rate =
            if i > 0 {
                (1. + spot_rates[i]).powi(i as i32 + 1) / (1. + spot_rates[i-1]).powi(i as i32) - 1.
            } else {
                spot_rates[0]
            };
        res.push(forward_rate);
    }

    return res;
}

fn calculate_discount_factors(prices: &Vec<f64>, coupon_rates: &Vec<f64>, face_value: f64) -> OVector<f64, Dyn> {
    assert_eq!(prices.len(), coupon_rates.len());
    let n = prices.len();
    let mut matrix = DMatrix::<f64>::zeros(n, n);

    for i in 0..n {
        let coupon_rate = coupon_rates[i];
        for j in 0..(i+1) {
            matrix[(i, j)] =
                if i == j {
                  face_value * (1. + coupon_rate)
                } else {
                    face_value * coupon_rate
                };
        }
    }

    let price_vector = DVector::<f64>::from_column_slice(&prices);
    let discount_factors = matrix.lu().solve(&price_vector)
        .expect("Failed to solve equation.");

    return discount_factors;

}


fn main() {
    // let market_prices = vec![1000., 995.49, 989.08, 982.63];
    // let coupon_rate = 0.05;
    // let face_value = 1000.;
    // let spot_rates = calculate_spot_interest_rate(
    //     market_prices, face_value, coupon_rate
    // );
    // let forward_rates = calculate_forward_rate(&spot_rates);
    // println!("spot rates: {spot_rates:?}");
    // println!("forward rates: {forward_rates:?}");

    let prices = vec![
        96.60, 93.71, 91.56, 90.24, 89.74,
        90.04, 91.09, 92.82, 95.19, 98.14,
        101.60, 105.54, 109.90, 114.64, 119.73
    ];

    let coupon_rates = vec![
        0.02, 0.025, 0.03, 0.035, 0.04, 0.045, 0.05, 0.055, 0.06, 0.065, 0.07, 0.075, 0.08, 0.085, 0.09
    ];

    let result = calculate_discount_factors(&prices, &coupon_rates, 100.);
    println!("result: {result:?}")
}

#[cfg(test)]
mod test {
    use super::*;
    use rust_lab::compare_utils::{assert_eq_f64, assert_eq_vec_f64};

    #[test]
    fn test_calculate_bond_pricing() {
        let bond_a = calculate_bond_pricing(
            1000., 10, 0.07, 0.07
        );

        let bond_b = calculate_bond_pricing(
            1000., 10, 0.13, 0.07
        );
        let tolerance = 1e-2;

        assert_eq_f64(bond_a.price, 1000., tolerance);
        assert_eq_f64(bond_a.duration, 7.52, tolerance);
        assert_eq_f64(bond_b.price, 1421.41, tolerance);
        assert_eq_f64(bond_b.duration, 6.75, tolerance);
    }


    #[test]
    fn test_calculate_spot_interest_rate() {
        let market_prices = vec![1000., 995.49, 989.08, 982.63];
        let coupon_rate = 0.05;
        let face_value = 1000.;
        let spot_rates = calculate_spot_interest_rate(
            market_prices, face_value, coupon_rate
        );
        let tolerance = 1e-4;
        assert_eq_f64(spot_rates[0], 0.05, tolerance);
        assert_eq_f64(spot_rates[1], 0.0525, tolerance);
        assert_eq_f64(spot_rates[2], 0.0542, tolerance);
        assert_eq_f64(spot_rates[3], 0.0551, tolerance);

        let forward_rates = calculate_forward_rate(&spot_rates);
        assert_eq_vec_f64(
            forward_rates, vec![0.0500, 0.0550, 0.0575, 0.0580], tolerance
        );

    }
}