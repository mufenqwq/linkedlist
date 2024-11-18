use std::ops::{Deref, DerefMut};

mod first;
mod second;

fn main() {
    println!("Hello, world!");
    let option_name = Some("Mufen".to_owned());
    let option = option_name.as_ref();
    let option1 = option_name.as_deref();
    match &option_name {
        Some(name) => println!("Name is {}", name),
        None => println!("No name provided"),
    }
    println!("{:?}", option_name);
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);
    let x = Box::new(1);
    let sum = *x + 1;
    println!("{}", sum);
    
    let y = MyBox::new(5);
    assert_eq!(5, *y);
    let s = String::from("Hello world!");
    display(&s);
    let mut s = MyBox::new(String::from("Hello world!"));
    display_mut(&mut s);
    let s1 = &s;
    let s2 = s.to_string();
    display(&(*s));
}

fn display(s: &str) {
    println!("{}", s);
}

fn display_mut(s: &mut String) {
    s.push_str("world");
    println!("{}", s);
}
struct MyBox<T>(T);

impl <T> MyBox<T> {
    fn new (x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl <T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl <T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
       &mut self.0
    }
}
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: String, age: u8) -> Self {
        Person { name, age }
    }
    fn display(self: &mut Person, age: u8) {
        let Person { name, age } = &self;
    }
}


