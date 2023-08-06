#![allow(unused)]

use std::env;
use dotenv::from_filename;

// fn main() {
//     from_filename(".env").ok();
//     let address = env::var("ADDRESS").unwrap();
//     println!("Address: {}", address);
// }

#[derive(Debug)]
struct Rect {
    w: u32,
    h: u32,
}

impl Rect {
    fn square(size: u32) -> Self {
        Self {
            w: size,
            h: size,
        }
    }

    fn area(&self) -> u32 {
        self.w * self.h
    }

    fn width(&self) -> bool {
        self.w > 0
    } 
}

fn rect_test() {
    let r = Rect {
        w: 30,
        h: 50,
    };
    println!("Area = {}", r.area());
    println!("{:#?}", r);
    
    dbg!(&r);
    println!("{:?}", r);
    println!("Width ? => {}", r.width());

    let square = Rect::square(5);
    println!("{:?}", square);
}

fn is_palindrom(word: &String) -> bool {
    let not_rev = word.chars().collect::<Vec<char>>(); 
    let rev = word.chars().rev().collect::<Vec<char>>();
    rev == not_rev
}

// fn main() {
//     let word = String::from("xyly");
//     let pal = is_palindrom(&word);
//     println!("Is palindrom: {}", pal);
// }
