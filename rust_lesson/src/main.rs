mod test_module {
    pub mod sub_module1 {
        pub fn test_fn1() {
            println!("test_fn1_1");
        }

        fn test_fn2() {
            println!("test_fn1_2");
        }
    }

    mod sub_module2 {
        pub fn test_fn1() {
            println!("test_fn2_1");
        }

        fn test_fn2() {
            println!("test_fn2_2");
        }
    }
}

use test_module::sub_module1;

fn main() {
    // crate::test_module::sub_module1::test_fn1();
    // test_module::sub_module1::test_fn1();

    sub_module1::test_fn1();
}
