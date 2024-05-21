use std::vec;

fn main() {
    // enum Option<T> {
    //     None,
    //     Some(T),
    // }

    let a: Option<i32> = Some(5);
    let b: Option<&str> = Some("Hello");
    let c: Option<i32> = None;

    let v: Vec<i32> = vec![1, 2, 3, 4, 5];
    let val: Option<&i32> = v.get(0);

    match val {
        // Some(1) => println!("Value is: 1"),
        // Some(2 | 3) => println!("Value is: 2 or 3"),
        Some(x) if *x == 1 => println!("Value is: 1"),
        Some(x) => println!("Value is: {}", x),
        None => println!("Value not found"),
    }
}
