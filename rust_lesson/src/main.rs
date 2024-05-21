use rand::Rng;

fn main() {
    let random_number: i32 = rand::thread_rng().gen_range(1..101);
    println!("{}", random_number);
}
