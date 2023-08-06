#![allow(unused)]

use std::f32::consts::PI;
use std::fmt::Display;
use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::ops::{Add, Index};
use std::collections::HashMap;

mod restaurant;
use crate::restaurant::order_food;
mod algorithms;
use crate::algorithms::algorithms::sort_vector;

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

fn vector_stuff() {
    let mut vector = Vec::from([1, 2, 3]);
    println!("Vector = {:?}", vector);
    change_vec(&mut vector);
    println!("Vector = {:?}", vector);

    println!("5.313 + 3.224 = {}", get_sum_gen(5.313, 3.224));
}

fn clone_ref_stuff() {
    let mut str1 = String::from("Worlds");
    let str1_copy = str1.clone();
    let mut str2 = &mut str1;
    *str2 = "worlds".to_string();
    println!("Hello {}", str1);

}

fn print_str(str: &String) {
    println!("String = {}", str);
}

fn change_string(str: &mut String) {
    str.push_str(" some more strings");
}

fn more_strings() {
    let mut str = String::from("Some String");
    print_str(&str);
    println!("{}", str);
    change_string(&mut str);
    println!("{}", str);
}

fn hash_maps() {
    let mut map = HashMap::new();
    map.insert(0x134, "Password");
    map.insert(0x535, "user2");

    for (k, v) in map.iter() {
        println!("Key: {}, Value: {}", k, v);
    }

    println!("Length: {}", map.len());

    if map.contains_key(&0x134) {
        let pswd = map.get(&0x134).unwrap();
        println!("Pswd: {}", pswd);
    }

    let val = map.get(&0x14).unwrap_or(&"none");
    println!("Value = {}", val);
}

fn structs() {
    struct Customer {
        name: String,
        address: String,
        balance: f32,
    }
    let mut bob: Customer = Customer { name: "Bob".to_string(), address: "Kotor 34".to_string(), balance: 500.3 };
    println!("{}", bob.name);
}

fn shapes() {
    struct Rectangle<T, U> {
        length: T,
        height: U,
    }
    let rec: Rectangle<i32, f32> = Rectangle { length: 4, height: 3.53 };
    // like an interface 
    trait Shape {
        fn new(length: f32, width: f32) -> Self;
        fn area(&self) -> f32;
    }
}

fn more_shapes() {
    trait Shape {
        fn new(length: f32, width: f32) -> Self;
        fn area(&self) -> f32;
    }
    struct Rectangle {length: f32, width: f32};
    struct Circle {length: f32, width: f32};

    impl Shape for Rectangle {
        fn new(length: f32, width: f32) -> Rectangle {
            return Rectangle{length, width};
        }
        fn area(&self) -> f32 {
            return self.length * self.width;
        }
    }

    impl Shape for Circle {
        fn new(length: f32, width: f32) -> Self {
            Circle { length, width }    
        }
        fn area(&self) -> f32 {
            PI * (self.length / 2.0).powf(2.0)
        }
    }

    let rec: Rectangle = Shape::new(1.42, 5.2);
    println!("Rectangle area = {}", rec.area());

    let circle: Circle = Shape::new(4.13, 0.4);
    println!("Circle area = {}", circle.area());

}


fn file_handling() {
    let path = "lines.txt";
    let output = File::create(path);
    let mut output = match output {
        Ok(file) => file,
        Err(error) => panic!("We have a problem...")
    };
    write!(output, "Just some text\nRandom words").expect("Failed to write to file");

    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    
    for line in buffered.lines() {
        // println!("{:?}", line);
        println!("{}", line.unwrap());
    } 
}

fn iterators() {
    let mut arr_it = [1, 2, 3, 4];
    for val in arr_it.iter() {
        println!("{}", val);
    }
    let mut iter1 = arr_it.iter();
    println!("1st: {}", iter1.next().unwrap());
    println!("2st: {}", iter1.next().unwrap());
}

fn closures() {
    // let var_name = |parameter| -> return_type
    let can_vote = |age: i32| {
        age >= 18
    };
    // println!("Can vote: {}", can_vote(28));

    let mut samp = 5;
    let print_var = || println!("sample = {}", samp);
    print_var();
}
// crate::algorithms::algorithms::sort_floats();

fn smart_pointers() {

   let b_int1 = Box::new(10); 

}


fn main() {
    smart_pointers();
}
