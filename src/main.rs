fn _some_arrays() {
    let mut vector = vec![1, 2, 3];

    for i in &mut vector {
        *i += 1;
        println!("{}", i);
    }
}

fn _some_strings() {
    let s = "some new string".to_string();
    let vec_s = s.chars().collect::<Vec<char>>();
    println!("First char: {}", &vec_s[0]);
    
    let mut i = 0;
    for c in s.chars() {
        println!("Char {} = {}", i, c);
        i += 1;
    }
    i = 0;
    for b in s.bytes() {
        println!("Byte {} = {}", i, b);
        i += 1;
    }
}

fn _hash_maps() {
    use std::collections::HashMap;

    let mut map = HashMap::new();

    map.insert("Petar", 150);
    map.insert("Tin", 90);

    let res = map.get("Petar").copied();
    let _val = match res {
        Some(v) => v,
        None => 0,
    };

    // iterating
    for (k, v) in &map {
        println!("Key: {} => Value: {}", k, v);
    }

    let dv = map.get("hello").copied().unwrap_or(0);
    println!("hello value = {}", dv);

    map.insert("Petar", map.get("Petar").copied().unwrap() + 10);


    let _entry = map.entry("Petar").and_modify(|v| { *v += 50 });
    println!("Petar => {}", &map.get("Petar").copied().unwrap());

    map.entry("Nick").or_insert(0);
    println!("{:?}", &map);
}


fn _if_let() {

    let config_max = Some(60);

    if let Some(max) = config_max {
        println!("Max value is {}", max);
    }
}

fn _string_slicing() {
    let words = String::from("jey key val sat");
    let first_word = words.split(" ").collect::<Vec<&str>>()[0];
    println!("First word: {}", &first_word);

    let slice = &words[0..6];
    println!("Slice: {}", &slice);
}

#[derive(Debug)]
struct A {
    h: u32,
    w: u32,
}

trait Math {
    fn area(&self) -> u32;
}

impl A {
    fn intersect(&self, other: &A) -> bool {
        self.h > other.h && self.w > other.w
    }
}

impl Math for A {
    fn area(&self) -> u32 {
        self.h * self.w
    }
}

fn pattern_matching() {

    #[derive(Debug)]
    enum Message {
        Hello {id: i32}
    }

    let message = Message::Hello { id: 9 };

    match message {
        Message::Hello { id: 10..=99 } => println!("index has 2 digits"),
        Message::Hello { id: idx @ 0..=9 } => println!("idx = {}", idx),
        Message::Hello { id } => println!("id = {}", id),
        // Message::Hello { id } => println!("ID = {}", id),
        // _ => println!("Wrong value"),
    }
}

#[derive(Debug)]
struct Sentance<'a>{
    first_sentance: &'a str,
}

fn _get_slice() -> &'static str {
    "slice"
} 

fn _split_sentance() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentance = novel.split('.').next().unwrap().to_owned();
    let sentance = Sentance{first_sentance: &first_sentance};
    println!("{:?}", sentance.first_sentance);
}

mod polymorphism;

fn main() {
    polymorphism::run();
}