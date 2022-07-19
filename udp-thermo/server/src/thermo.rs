use rand::Rng;

pub struct Thermometer;

impl Thermometer {
    pub fn generate() -> f64 {
        let mut rng = rand::thread_rng();
        rng.gen_range(20.0..50.0)
    }
}
