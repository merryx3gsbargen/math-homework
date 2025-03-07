
pub fn get_random_rust_code() {
let mut rng = rand::thread_rng();
let num1 = rng.gen_range(1..=10);
let num2 = rng.gen_range(1..=10);
let sum = num1 + num2;
return format!("The sum of {} and {} is {}", num1, num2, sum);
}