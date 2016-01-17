fn factorial(value: usize) -> usize {
    if value == 0 { 1 }
    else { value * factorial(value-1) }
}

fn calculate_binominal_coefficient(n: usize, k: usize) -> usize {
    factorial(n) / (factorial(k) * factorial(n-k))
}

pub fn calculate_bezier(t:f32, points: Vec<f32>) -> f32 {
    let n = points.len()-1;
    points.iter().enumerate().map(|(k, x)| {
        calculate_binominal_coefficient(n, k) as f32 * (1.0-t).powi((n-k) as i32) * t.powi(k as i32) * x
    }).fold(0f32, |sum, x| sum + x)
}

pub fn max(a: f32, b: f32) -> f32 {
    if a > b {a}
    else {b}
}

pub fn min(a: f32, b: f32) -> f32 {
    if a < b {a}
    else {b}
}
