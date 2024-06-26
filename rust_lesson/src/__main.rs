use std::rc::Rc;

fn main() {
    let x: Box<i32> = Box::new(1);
    println!("x: {:p}", x);
    println!("*x +2 = {}", *x + 2);

    let a: Rc<String> = Rc::new("hello".to_string());
    println!("count1: {}", Rc::strong_count(&a));
    {
        let b: Rc<String> = Rc::clone(&a);
        println!("a: {:p}", a);
        println!("b: {:p}", b);
        println!("count2: {}", Rc::strong_count(&a));
    }
    println!("count3: {}", Rc::strong_count(&a));
}
