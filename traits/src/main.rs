/*
 * basket is a struct that can hold any kind of data
 * stack can also hold any kind of data
 * 
 * define the methods of each as a trait (as they are similar)
 *
 */ 

mod basket;
mod stack;
mod container;
use basket::Basket;
use stack::Stack;
use container::Container;

fn add_string<T: Container<String>>(c: &mut T, s: String) {
    c.put(s);
}

fn main() {
    let mut b1 = Basket::new(String::from("hi there"));
    let b2 = Basket::new(true);

    let mut s1 = Stack::new(
        vec![String::from("hi")]
        );
    let s2 = Stack::new(vec![1,2,3]);

    add_string(&mut b1, String::from("hi"));
    add_string(&mut s1, String::from("hiIHI"));

    println!("Hello, world!");
}
