#![allow(unused_variables)]
#![allow(unused_imports)]
use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;

pub fn read_username_from_file(file_name: &String) -> Result<String, io::Error> {
    fs::read_to_string(file_name)
}

pub fn read_username_from_file_match(file_name: &String) -> Result<String, io::Error> {
    let f = File::open(file_name);

    let mut f: File = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

pub fn read_username_from_file_question(file_name: &String) -> Result<String, io::Error> {
    let mut f = File::open(file_name)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

pub fn read_username_from_file_question_chained(file_name: &String) -> Result<String, io::Error> {
    let mut s = String::new();
    File::open(file_name)?.read_to_string(&mut s)?;
    Ok(s)
}
