use std::f64::consts::{PI, E};

pub fn L(y: f64, sigma: f64) -> f64 {
    (P(y, 1., sigma)/P(y, 0., sigma)).ln()
}
fn P(y: f64, x: f64, sigma: f64) -> f64 {
    E.powf(-(y-x)*(y-x)/(2.*sigma*sigma))/(sigma*sigma*PI*2.).sqrt()
}
pub fn soft_decode(y: f64, sigma: f64) -> (bool, f64) {
    let soft = L(y, sigma);
    let hard = if soft > 0. {
        true
    } else {
        false
    };
    (hard, soft)
}
pub fn reorder<T>(original: Vec<T>, new_order: &Vec<usize>) -> Vec<T>
where T: Copy {
    let mut new = vec![original[0]; original.len()];
    for (i, j) in new_order.iter().enumerate() {
        new[*j] = original[i];
    }
    new
}
pub fn reorder_reverse<T>(original: Vec<T>, new_order: &Vec<usize>) -> Vec<T>
where T: Copy {
    let mut new = vec![original[0]; original.len()];
    for (i, j) in new_order.iter().enumerate() {
        new[i] = original[*j];
    }
    new
}