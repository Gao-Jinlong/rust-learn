pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}。",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

#[cfg(test)] // 标注单元测试，只有在测试时才编译
             // cfg 是 configuration 的缩写，它告诉 Rust 其之后的项只应被包含进特定配置选项中，这里是 test，即 Rust 提供的用于编译和运行测试的配置选项。
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[ignore] // 忽略测试
              // cargo test -- --ignored 只运行被忽略的测试
              // cargo test -- --include-ignored 运行所有测试
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 3,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    // fn smaller_cannot_hold_larger() {
    //     let larger = Rectangle {
    //         width: 8,
    //         height: 7,
    //     };
    //     let smaller = Rectangle {
    //         width: 5,
    //         height: 3,
    //     };

    //     assert!(smaller.can_hold(&larger));
    // }
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn it_adds_two_ne() {
        assert_ne!(5, add_two(2));
    }

    #[test]
    #[should_panic(expected = "less than or equal to 100")] // 添加 should_panic 测试应该 panic 的代码, 并且 panic 信息包含 "less than or equal to 100"
    fn greater_than_100() {
        Guess::new(200);
    }

    // 将 Result<T, E> 用于测试
    #[test]
    fn is_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
