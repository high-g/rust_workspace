fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<i32> = vec![0; 1000];
    
    let mut v3: Vec<i32> = Vec::new();
    v3.push(1);
    v3.push(2);
    v3.push(3);
    println!("{:?}", v3);

    let x: Option<i32> = v3.pop();
    println!("{:?}", x);
    println!("{:?}", v3[1]);


    let z: Option<&i32> = v3.get(1);
    println!("{:?}", z);
}
