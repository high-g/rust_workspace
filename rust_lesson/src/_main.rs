// use std::ops::RangeInclusive;
// mod fizzbuzz;

fn concat(a: &String, b: &String) -> String {
    let c: String = format!("{}, {}", a, b);
    c
}

fn main() {
    let x1: Vec<i32> = vec![1, 2, 3];
    let x2: &Vec<i32> = &x1;

    println!("{:?}", x1);
    println!("{:?}", x2);

    let v1: Vec<i32> = vec![1, 2, 3];
    // println!("v1 ptr: {:?}", v1.as_ptr());
    // println!("v1[0]: {:p}", &v1[0]);
    // println!("v1 len: {:?}", v1.len());
    // println!("v1 capacity: {:?}", v1.capacity());

    // v1.push(4);
    // println!("v1 ptr: {:?}", v1.as_ptr());
    // println!("v1 len: {:?}", v1.len());
    // println!("v1 capacity: {:?}", v1.capacity());

    let v2: Vec<i32> = v1.clone();
    println!("v1 ptr: {:?}", v1.as_ptr());
    println!("v2 ptr: {:?}", v2.as_ptr());

    let s1: String = String::from("hello");
    let s2: String = String::from("hello");
    let s = concat(&s1, &s2);
    println!("{}", s);
    println!("{}", s1);
    println!("{}", s2);

    // let mut cnt: i32 = 0;
    // while cnt <= 20 {
    //     fizzbuzz::fizzbuzz(cnt);
    //     cnt += 1;
    // }

    // for i in 0..=20 {
    //     fizzbuzz::fizzbuzz(i);
    // }

    // // loop
    // let mut cnt: i32 = 0;
    // loop {
    //     println!("Hello Loop");
    //     if cnt == 10 {
    //         break;
    //     }
    //     cnt += 1;
    // }

    // // while
    // let mut cnt2: i32 = 0;
    // while cnt2 <= 10 {
    //     println!("Hello While");
    //     cnt2 += 1;
    // }

    // // for
    // for i in [1, 2, 3] {
    //     println!("Hello, {}", i);
    // }

    // let r: RangeInclusive<i32> = 1..=10;
    // for j in r {
    //     println!("Hello, {}", j);
    // }

    // let x: i32 = 0;
    // match x {
    //     0 => println!("x is zero"),
    //     1 => {
    //         println!("x is one");
    //         println!("x is one");
    //     }
    //     _ => println!("x is something else"),
    // }

    // let y: i32 = match x {
    //     0 => x + 10,
    //     1 => x + 20,
    //     _ => x + 100,
    // };

    // println!("{}", y);

    // let x: i32 = 5;

    // if x > 0 {
    //     println!("OK!");
    // }

    // if x > 0 && x < 10 {
    //     println!("x > 0 && x < 10");
    // }
    // if x < 0 || x > 10 {
    //     println!("x < 0 || x > 10");
    // }

    // let x: i32 = 10;
    // let y: i32 = if x > 30 {
    //     x
    // } else {
    //     x + 10
    // };

    // println!("{}", y);

    // {}
    // {
    //     let x = 1;
    //     println!("{}", x);
    // }

    // let y: i32 = 10;
    // println!("{}", y);

    // {
    //     let y: i32 = 5;
    //     println!("{}", y);
    // }

    // println!("{}", y);
}

// fn say_hello() {
//     println!("Hello!");
// }

// fn add(x: i32, y: i32) -> i32 {
//     x + y
// }

// fn main() {
//     say_hello();
//     println!("{}", add(1,2));

//     // let v1: Vec<i32> = vec![1, 2, 3];
//     // let v2: Vec<i32> = vec![0; 1000];

//     // let mut v3: Vec<i32> = Vec::new();
//     // v3.push(1);
//     // v3.push(2);
//     // v3.push(3);
//     // println!("{:?}", v3);

//     // let x: Option<i32> = v3.pop();
//     // println!("{:?}", x);
//     // println!("{:?}", v3[1]);

//     // let z: Option<&i32> = v3.get(1);
//     // println!("{:?}", z);

//     // 文字型
//     let c1: char = 'a';

//     // 文字列型
//     let s1: &str = "hello"; // 文字列リテラル
//     let s2: String = "hello".to_string(); // String型
//     let s3: String = String::from("hello"); // String型
//     let mut s4: String = String::from("hello"); // String型
//     s4.push_str("world");

//     println!("{}", s4);
//     println!("{}", s3 + "world2");

//     let s5: String = format!("{}{}", s1, s2);
//     println!("{}", s5);

// }
