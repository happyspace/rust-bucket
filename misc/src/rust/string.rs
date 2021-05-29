use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    ops::AddAssign,
};

/// https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-elision

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

pub fn check_palindrome(input: &str) -> bool {
    if input.len() == 0 {
        return true;
    }
    let mut last = input.len() - 1;
    let mut first = 0;

    // to_owned() not needed here ... input.as_bytes().to_owned();
    let my_vec = input.as_bytes();
    while first < last {
        if my_vec[first] != my_vec[last] {
            return false;
        }

        first += 1;
        last -= 1;
    }
    return true;
}

/// https://stackoverflow.com/questions/47441279/creating-an-stdenvargs-iterator-for-testing
/// https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/boxing_errors.html
///
pub fn wc<T>(files: T) -> Result<WordCounts, Box<dyn Error>>
where
    T: Iterator<Item = String>,
{
    let mut wcs = WordCounts::new();
    let mut file_count: usize = 0;

    for (count, file_name) in files.enumerate() {
        file_count = count;
        let mut wc = WordCounts::new();
        let f = File::open(&file_name)?;
        // From<std::io::Error>` is not satisfied
        let f = BufReader::new(f);

        for line in f.lines() {
            let ll = line?;
            wc.lines = wcs.lines + 1;
            wc.words = ll.split_whitespace().count();
            wc.chars = wc.chars + ll.len() + 1;
        }

        log::info!("\t{}\t{}\t{}\t{}", wc.lines, wc.words, wc.chars, &file_name,);
        wcs += wc;
    }

    if file_count > 1 {
        log::info!("\t{}\t{}\t{}", wcs.lines, wcs.words, wcs.chars,);
    }

    return Ok(wcs);
}

#[derive(Default)]
pub struct WordCounts {
    lines: usize,
    words: usize,
    chars: usize,
}

impl WordCounts {
    fn new() -> Self {
        Default::default()
    }
}

impl AddAssign for WordCounts {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            lines: self.lines + other.lines,
            words: self.words + other.words,
            chars: self.chars + other.chars,
        }
    }
}

/// TODO create a work around for "Run Test" "Debug" code
#[cfg(test)]
mod test {
    use log::LevelFilter;
    use simple_logger::SimpleLogger;

    use crate::rust::string::{check_palindrome as is_palindrome, first_word, wc as word_count};

    #[test]
    fn wc() {
        // let _ = TermLogger::init(LogLevelFilter::Info, Config::default());
        SimpleLogger::new()
            .with_level(LevelFilter::Info)
            .init()
            .unwrap();
        log::info!("{:?}", std::env::current_dir());

        let file_names = vec!["..\\poem.txt"];
        let _counts = word_count(file_names.iter().map(|s| s.to_string()));
        match _counts {
            Ok(_) => {
                log::info!("{}", "word!");
            }
            Err(e) => {
                log::info!("world of troubles: {}", e);
            }
        }
    }

    #[test]
    fn check_palindrome() {
        // true
        let mut check = "1";
        assert!(is_palindrome(check));
        // true
        check = "abccba";
        assert!(is_palindrome(check));
        // true
        check = "abcba";
        assert!(is_palindrome(check));
        // false
        check = "acba";
        assert!(!is_palindrome(check));
    }

    // https://www.linuxjournal.com/content/text-processing-rust
    // 'basicOP.rs' as a test
    #[test]
    fn basic_op() {
        // &str
        let l_j: &str = "Linux Journal ^^^";
        // Or
        let magazine = "magazine";

        // Use format! to create a String
        let my_str = format!("Hello {} {}!", l_j, magazine);
        println!("my_str L:{} C:{}", my_str.len(), my_str.capacity());

        // char by char
        for c in my_str.chars() {
            println!("{} ", c);
        }

        println!();

        let n: &str = "10";

        match n.parse::<i32>() {
            Ok(n) => println!("number is: {}", n),
            Err(e) => println!("nan is {}", e),
        }

        let n2: &str = "10.2";
        match n2.parse::<i32>() {
            Ok(nan) => println!("nope no number here: {}", nan),
            Err(e) => println!("pesky parser: {}", e),
        }

        // fn  works with both str and String
        let my_str: &str = "first words are important!";
        let mut my_string: String = String::from("second words are important too!");

        println!("my first word: {}", first_word(&my_str));
        println!("second word: {}", first_word(&my_string));

        println!(
            "my string L: {} C: {}",
            my_string.len(),
            my_string.capacity()
        );
        my_string.push_str(" Ok?");
        println!(
            "my string L: {} C: {}",
            my_string.len(),
            my_string.capacity()
        );

        // convert String => str
        let _s_str: &str = &my_string[..];
        // str => String
        let s_string = _s_str.to_owned();

        println!("famous last words: {}", s_string);
    }
}
