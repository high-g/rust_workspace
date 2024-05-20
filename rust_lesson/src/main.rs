fn main() {
    {
        let r: &i32;
        {
            let x: i32 = 1;
            r = &x;
            println!("{}", r);
        }
    }
}
