pub fn say_hello() {
    println!("Helllo!");
}

pub mod sample_trait {
    pub trait  Shape2 {
        fn calc_area(&self) -> f64;
        fn calc_perimeter(&self) -> f64;
        fn do_something();
        fn defauilt_something(&self) -> &str{
            "Tgis is default method!"
        }
    }

    pub struct Rectangle2 {
        pub width: f64,
        pub height: f64,
    }

    impl Shape2 for Rectangle2 {
        fn calc_area(&self) -> f64 {
            self.width * self.height
        }

        fn calc_perimeter(&self) -> f64 {
            self.width * 2.0 + self.height * 2.0
        }

        fn do_something() {
            println!("This is Ractangle function");
        }
    }

    pub struct Circle {
        pub radius: f64,
    }

    impl Shape2 for Circle {
        fn calc_area(&self) -> f64 {
            self.radius * self.radius * std::f64::consts::PI
        }

        fn calc_perimeter(&self) -> f64 {
            self.radius * 2.0 * std::f64::consts::PI
        }

        fn do_something() {
            println!("This is Circle function");
        }
    }

    pub fn double_area(shape: &impl Shape2) -> f64 {
        shape.calc_area() * 2.0
    }
}