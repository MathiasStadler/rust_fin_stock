mod main_test;


pub fn greet() -> String {
    String::from("Hello, World!")
}

fn main() {
    println!("Hello, world!");
}

// f: Box<dyn Fn(&X)
// https://www.google.com/search?client=ubuntu-sn&channel=fs&q=f%3A+Box%3Cdyn+Fn%28%26X%29+