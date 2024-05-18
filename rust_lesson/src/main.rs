fn main() {
    let x: i32 = 5;

    if x > 0 {
        println!("OK!");
    }

    if x > 0 && x < 10 {
        println!("x > 0 && x < 10");
    }
    if x < 0 || x > 10 {
        println!("x < 0 || x > 10");
    }


    let x: i32 = 10;
    let y: i32 = if x > 30 {
        x
    } else {
        x + 10
    };

    println!("{}", y);

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
