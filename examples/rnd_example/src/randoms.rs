pub mod random_generations {
    use rand::Rng;
    pub fn generate_random() -> f32 {
        let mut rng = rand::thread_rng();
        let val: f32 = rng.gen_range(0,1000) as f32;
        return val;
    }
}

