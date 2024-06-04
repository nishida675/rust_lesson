use rand::Rng;

const A: i32 = 2;

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }
}

enum Shape {
    Circle,
    Square(u32),
    Triangle { base: u32, height: u32 },
}

impl Shape {
    fn sample_method(&self) {
        println!("call sample_method");
    }
}

mod test_module;

use test_module::sub_module1;
use rust_lesson::sample_trait::{Shape2, Rectangle2, Circle, double_area};

fn main() {
    //println!("Hello, world!");
    /*print!("Hello, ");
    print!("Rust!"); */

    print!("Hello, {}", "students");

    let a: i32 = 1;
    println!("{}", a);
    println!("{}", A);

    //整数型
    // let a = 1;
    //let b = 2.0;

    //let f: f64 = 1 as f64 + 2.0;

    //論理型
    //let g = false;

    //タプル
    let t1 = (1, true, 2.0);
    //let t2 = (2.0, 1, true); //t1 != t2

    println!("{:?}", t1);

    let i = t1.1;
    println!("{}", i);

    //配列
    let l1 = [1, 2, 3];
    //let l2 = [0;1000];
    println!("{:?}", l1);

    //ベクタ
    //let v1 = vec![1,2,3];
    //let v2 = vec![0; 1000];

    let mut v3 = Vec::new();
    v3.push(1);
    v3.push(2);
    v3.push(3);
    println!("{:?}", v3);

    //文字型
    //let c1 = 'A';

    //文字列型
    //let s1 = "RUST";

    //let s2 = String::from("Hello");
    //let s3 = "Java".to_string();

    let mut s4 = String::from("Hello");
    s4.push_str(", Rust");
    println!("{}", s4);

    //match

    let x = 0;

    match x {
        0 => println!("Zero!"),
        1 => {
            println!("One!");
            println!("One!");
        }
        _ => println!("Other!"),
    };

    //loop

    // let mut cnt = 0;
    // loop {
    //     println!("Hello");
    //     if cnt == 10 {
    //         break;
    //     }
    //     cnt += 1;
    // }

    //構造体
    //let height: u32 = 5;
    // let mut rectangle: Rectangle = Rectangle {
    //     width: 10,
    //     height,
    // };
    let mut rectangle = Rectangle::new(10, 5);

    println!("width : {}", rectangle.width);
    println!("height : {}", rectangle.height);

    rectangle.height = 10;
    println!("height : {}", rectangle.height);

    println!("area: {}", rectangle.area());

    let c = Shape::Circle;
    let s = Shape::Square(1);
    let t = Shape::Triangle {
        base: 10,
        height: 5,
    };

    c.sample_method();
    s.sample_method();
    t.sample_method();

    // enum Option<T> {
    // None,
    // Some(T),
    //}
    // let a = Some(1);
    // let b = Some("str");
    // let c: Option<i32> = None;

    let v = vec![1, 2, 3];
    let val = v.get(0);

    match val {
        // Some(1) => println!("value is 1"),
        // Some(2 | 3) => println!("value is: 2 or 3"),
        Some(x) if *x == 1 => println!("value is 1"),
        Some(x) => println!("value exists: {}", x),
        None => println!("value is None"),
    }

    if let Some(x) = val {
        println!("val={}", x);
    }

    //乱数
    let random_number = rand::thread_rng().gen_range(1..101);
    println!("{}", random_number);

    //モジュール
    crate::test_module::sub_module1::test_fn1();
    test_module::sub_module1::test_fn1();
    test_module::sub_module2::test_fn1();

    sub_module1::test_fn1();

    rust_lesson::say_hello();

    let rect = Rectangle2 {
        width: 4.0,
        height: 5.0,
    };
    let circle = Circle {
        radius: 2.0,
    };

    println!("Rectangle area is {}", rect.calc_area());
    Rectangle2::do_something();

    println!("{}", circle.defauilt_something());

    println!("{}", double_area(&rect));

    //#[属性]

    println!("{:?}", (1, 2, 3));

    #[derive(Debug, PartialEq)]
    struct S {
        val1: i32,
        val2: i32,
    }

    println!("{:?}", S {val1: 1, val2: 2});

    let s1 = S {
        val1: 1,
        val2: 3,
    };

    let s2 = S {
        val1: 1,
        val2: 3,
    };

    println!("{}", s1 == s2);

}
