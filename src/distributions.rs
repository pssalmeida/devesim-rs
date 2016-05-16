
use rand::Rng;

pub fn weibull<R: Rng>(rng: &mut R, scale: f64, k: f64) -> f64 {
    let x: f64 = rng.gen();
    scale * (-f64::ln(1.0 - x)).powf(1.0 / k)
}

