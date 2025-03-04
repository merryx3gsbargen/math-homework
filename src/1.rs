fn main() {
    let mut rng = rand::thread_rng();
    let x: f64 = rng.gen_range(1.0..=10.0);
    let y: f64 = rng.gen_range(1.0..=10.0);
    println!("{:?}", x + y);
}
