#![allow(unused)]

use std::fmt::Display;
use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::ops::Add;

fn say_hello() {
    println!("What is your name?");
    let mut name = String::new();
    let greeting = "Nice to meet you";
    io::stdin().read_line(&mut name).expect("Didn't receive input");

    println!("Hello {}!", name.trim_end());
}

fn types() {
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.141592;
    let age: &str = "42";
    let mut age: u32 = age.trim().parse().expect("Age wasn't assigned a number");
    age = age + 1;
}

fn random_numbers() {
    let random_num = rand::thread_rng().gen_range(1..100);
    let random_num2: u32 = rand::thread_rng().gen();
    println!("Random number = {}", random_num);
    println!("Random number 2 = {}", random_num2);
}

fn can_vote() {
    let age = 22;
    let can_vote = if age >= 18 {
        true
    } else {
        false
    };
    println!("Can vote? {}", can_vote); 
}

fn match_statement() {
    let age = 22;
    match age {
        1..=18 => println!("Young"),
        21 | 50 => println!("Cool age"),
        65..=i32::MAX => println!("Old"),
        _ => println!("else..."),
    };
}

fn match_ordering() {
    let age = 22;
    let voting_age = 18;
    match age.cmp(&voting_age) {
        Ordering::Less => println!("Can't vote..."),
        Ordering::Equal => println!("Now you can vote..."),
        Ordering::Greater => println!("Can vote..."),
    };
}

fn arrays() {
    let arr_1 = [1, 2, 3, 4];
    println!("1st = {}", arr_1[0]);
    println!("Lenght = {}", arr_1.len());

    let array_2 = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut loop_idx = 0;
    // loop {
    //     println!("Element {} = {}", loop_idx, array_2[loop_idx]);
    //     loop_idx += 1;
    //     if loop_idx > 9 {
    //         break;
    //     }
    // }
    for val in array_2.iter() {
        println!("{}", val);
    }
}

fn tuples_example() {
    let my_tuple: (u8, String, f64) = (17, String::from("Petar"), 3.14); 
    println!("Name = {}", my_tuple.1);
    let (v1, v2, v3) = my_tuple;
    println!("Number = {}", v3);
}

fn strings() {
    // String
    // &str
    let mut str1 = String::new();
    str1.push('A');
    str1.push_str(" word");
    for word in str1.split_whitespace() {
        println!("{}", word);
    }
    let str2 = str1.replace("A", "Another");
    println!("{}", str2);

    let str3 = String::from("x r b a h");
    let mut v1: Vec<char> = str3.chars().collect();

    for char in v1 {
        println!("{}", char);
    }
    let str4: &str = "Random string";
    let mut str5: String = str4.to_string();
    println!("{}", str5);

    let byte_array: &[u8] = str5.as_bytes();
    let str6: &str = &str5[0..6];
    println!("String {} length = {}", str6, str6.len());
    str5.clear();

    let str7: String = String::from("Some text");
    let str8: String = "more text".to_string();
    let str9 = str7 + &str8;

    for char in str8.chars() {
        println!("{}", char);
    }
}


fn type_casting() {
    let int1_u8: u8 = 5;
    let int2_u8: u8 = 4;
    let float_num1: f32 = 3.13;
    let int3 = int1_u8 + int2_u8;
    let num4: f32 = float_num1 + (int2_u8 as f32);
    println!("{}", num4);
    let string_num = num4.to_string();
}

fn enum_example() {
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday
    }

    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Saturday | Day::Sunday => true,
                _ => false
            }
        }
    }

    let mut today: Day = Day::Wednesday;
    println!("Is today a weekend? => {}", today.is_weekend());

}

fn vectors() {
    let mut vec1 = vec![1, 2, 3];
    let mut vec2: Vec<i32> = Vec::new();
    vec2.push(5);
    vec2.push(10);
    println!("{:?}", vec2);

    for i in &mut vec2 {
        *i *= 2;
    }

    for i in &vec2 {
        println!("{}", i);
    }

    let mut vec1_copy = &mut vec1;

    println!("Poped value: {:?}", vec1_copy.pop().unwrap());
    println!("{:?}", vec1);
}

fn get_sum(x: i32, y: i32) -> i32 {
    x + y
    // or return x + y;
}

fn get_values() -> (i32, i32) {
    let x1 = rand::thread_rng().gen_range(0..10);
    let x2 = rand::thread_rng().gen_range(0..10);
    // return (x1, x2);
    (x1, x2)
}

fn sum_vec(vec: Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in vec {
        sum += i;
    }
    sum
}

fn sum_list(list: &[i32]) -> i32 {
    let mut sum: i32 = 0;
    for &val in list.iter() {
        sum += &val;
    }
    sum
}

fn change_vec(vector: &mut Vec<i32>) {
    vector[0] = 14;
}

fn get_sum_gen<T:Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}

/*
# About memory in Rust (ownership)

- Stack: Stores values in a last in first out format,
 data on the stack must have a definede fixed size

 - Heap: When putting data on the heap you request a
 certain amount of space. The OS finds space available
 and returns an address for that space called a pointer

# Some rules:
1) Each value has a variable that is called its owner
2) There is only one owner at a time
3) When the owner goes out of scope the value disappears


*/



fn main() {

    let mut vector = Vec::from([1, 2, 3]);
    println!("Vector = {:?}", vector);
    change_vec(&mut vector);
    println!("Vector = {:?}", vector);

    println!("5.313 + 3.224 = {}", get_sum_gen(5.313, 3.224));
}
