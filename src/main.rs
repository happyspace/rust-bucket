use rand::prelude::*;
use std::fs::File;
use std::io::{self as ioZzz, Error, ErrorKind};
use std::net::IpAddr;

use std::env;

use lang::std::plus_one;
use misc;
use misc::mix;
use misc::PrimaryColor;

use lang::std::file::read_username_from_file_match;
use lang::std::file::read_username_from_file_question;
// use lang:;
// use lang::Point;

use lang::std::enums::Coin;

use lang::error::find;
use lang::error::find_extension_local_map;
use lang::error::find_extension_map;
use lang::error::find_extension_smpl;
use lang::error::Guess;

use lang::generic::Complex;
use lang::generic::Point;

use lang::vec;

use log::{info, warn};

fn main() {
    // get arguments...
    let mut argv = env::args();

    let args: Vec<_> = env::args().collect();

    println!("arguments {}", args[0]);

    let num = 10;

    println!("Hello, world! {} plus one is {}", num, misc::add_one(num));

    let _x: u8 = random();

    println!("{}", _x);

    // let _v = vec_x![1, 2, 3];
    let _v: Vec<u32> = vec![1, 2, 3];

    let _p = Point { x: 1, y: 2 };
    println!("{}", _p);
    println!("{:?}", _p);
    println!("{:#?}", _p);

    let _c = Complex { re: 1.1, im: 1.4 };
    println!("{}", _c);

    let _home: IpAddr = "127.0.0.1".parse().unwrap();

    let guess = "32";
    let guess: i32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    let guess: Guess = Guess::new(guess);
    println!("{}", guess);

    let one_more = plus_one(Some(32));

    match one_more {
        Some(om) => println!("One more {}", om),
        None => {}
    }

    // suffixed literals 0u8 = 0 unsigned 8-bit integer.
    let some_u8_value = Some(0u8);

    if let Some(3) = some_u8_value {
        println!("three");
    }

    let file_name = "hello.txt";

    match find(file_name, '.') {
        None => println!("No file extension found."),
        Some(i) => println!("file extension: {}", &file_name[i + 1..]),
    }

    match find_extension_smpl(file_name) {
        None => println!("No file extension found."),
        Some(str) => println!("file extension smpl: {}", str),
    }

    match find_extension_map(file_name) {
        None => println!("No file extension found."),
        Some(str) => println!("file extension map: {}", str),
    }

    match find_extension_local_map(file_name) {
        None => println!("No file extension found."),
        Some(str) => println!("file extension local map: {}", str),
    }

    let f = File::open(file_name).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(file_name).unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    let rs: Result<String, std::io::Error> = read_username_from_file_match(&file_name.to_owned());

    let name = rs.unwrap_or("moo".to_owned());
    println!("And the name is: {}", name);

    let mut setting_value = None;
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite existing value.");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    let s = Some(String::from("Hello!"));
    if let Some(_) = s {
        println!("found a string");
    }
    println!("{:?}", s);

    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    let quarter = Coin::Quarter;
    let cent: Coin = Coin::Penny;

    println!("Is cent? {}", quarter.is_cent());
    println!("Is cent? {}", cent.is_cent());

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("Liftoff!!!");
}
