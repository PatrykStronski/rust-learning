use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let val: f32 = (rng.gen_range(0,1000) as f32) / 20.0;
    println!("{}", val);
}
