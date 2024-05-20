struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // コンストラクタ
    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }
}

enum Shape {
    Circle,
    Square(u32, u32),
    Triangle { base: u32, height: u32 },
}

impl Shape {
    fn sample_method(&self) {
        println!("sample_method");
    }
}

fn main() {
    // enum Option<T> {
    //     None,
    //     Some(T),
    // }

    let c: Shape = Shape::Circle;
    let s: Shape = Shape::Square(10, 20);
    let t: Shape = Shape::Triangle {
        base: 10,
        height: 20,
    };

    c.sample_method();
    s.sample_method();
    t.sample_method();

    let mut rectangle: Rectangle = Rectangle::new(30, 50);

    println!("width: {}", rectangle.width);
    println!("height: {}", rectangle.height);

    rectangle.height = 20;
    println!("height: {}", rectangle.height);

    println!("area: {}", rectangle.area());
}
