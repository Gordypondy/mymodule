pub fn sum(values: &[f64]) -> f64 {
    values.iter().sum()
}

pub fn mean(values: &[f64]) -> f64 {
    sum(values) / values.len() as f64
}

pub fn maximum(values: &[f64]) -> f64 {
    values
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max)
}

pub fn minimum(values: &[f64]) -> f64 {
    values
        .iter()
        .copied()
        .fold(f64::INFINITY, f64::min)
}