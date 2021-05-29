use num::cast::NumCast;
use num::Float;
use num::Integer;
use num::Num;
use num::{FromPrimitive, ToPrimitive, Zero};
use rand::distributions::{Distribution, Uniform};
use std::ops::{Add, Div};
use std::{convert::*, ops::Neg};
use std::{fmt::Debug, ops::Index};
use std::{iter::Sum, ops::DivAssign};

pub struct RandGenerator {
    range: Uniform<i32>,
}

impl RandGenerator {
    pub fn new(range: Uniform<i32>) -> Self {
        RandGenerator { range }
    }
}

impl Iterator for RandGenerator {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.range.sample(&mut rand::thread_rng()))
    }
}

pub fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

pub fn mean_r<T>(list: &[T]) -> Option<f64>
where
    T: Copy + ToPrimitive + FromPrimitive + Num + Into<f64> + Sum + Add<T, Output = T>,
{
    match list.len() {
        0 => None,
        _ => {
            // let sum: T = list.iter().sum();
            // from the docs...
            // cloned is the same as .map(|&x| x), for integers
            // let v_map: Vec<_> = a.iter().map(|&x| x).collect();

            let sum: T = list.iter().cloned().sum();

            let sum: f64 = Into::<f64>::into(sum);
            let length = f64::from_usize(list.len())?;
            Some(sum / length)
        }
    }
}

pub fn mean_s<T>(list: &[T]) -> Option<f64>
where
    T: ToPrimitive + Copy + Num + Add + Into<f64>,
{
    // let length = list.len();
    let mut total = 0.0;
    if list.is_empty() {
        return None;
    }

    for elem in list {
        total += Into::<f64>::into(*elem);
        // total = *elem.into() + total;
    }

    let mean = total / list.len() as f64;

    Some(mean)
}

/// All sorts of interesting issues ...
/// <https://www.reddit.com/r/rust/comments/620m1v/never_hearing_the_trait_x_cannot_be_made_into_an/>
/// <https://rustwasm.github.io/twiggy/concepts/generic-functions-and-monomorphization.html>
///
pub fn median<T>(list: &mut [T]) -> Option<f64>
where
    T: ToPrimitive + Copy + Eq + PartialEq + Ord + PartialOrd + Into<f64> + Num,
{
    let length = list.len();
    if length == 0 {
        return None;
    }
    list.sort_unstable();
    let mut median: f64;
    if length % 2 != 0 {
        let a_t = list.index(length / 2);

        // silliness
        median = Into::<f64>::into(*a_t);
        // median = *a_t.into();
        median = NumCast::from(*a_t).unwrap();

    // median = NumCast::from(list.index(length / 2)) as f64;
    } else {
        let a_t_1 = list.index((length - 1) / 2);
        let a_t_2 = list.index(length / 2);
        let sum = *a_t_1 + *a_t_2;
        median = sum.into() / 2.0;
    }

    Some(median)
}

/// <https://stackoverflow.com/questions/28565440/how-do-i-use-integer-number-literals-when-using-generic-types>
pub fn count_digits<T>(mut x: T) -> u8
where
    T: Integer + Neg<Output = T> + DivAssign,
    u8: Into<T>,
{
    // convert zero to our type T integer.
    let zero = 0.into();
    if x == zero {
        return 1;
    }
    let mut digits = 0u8;
    if x < zero {
        digits += 1;
        x = -x;
    }

    while x > zero {
        x /= 10.into();
        digits += 1;
    }

    digits
}

///
/// https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.inspect

#[rustfmt::skip]
pub fn sum_odd_with_inspection<T>(list: &mut [T]) -> Option<T>
where
    T: Num + Debug + Copy + ToPrimitive + From<i32> + Integer + FromPrimitive + Zero,
{
    let zero: T = 0.into();

    let sum = list.iter()
        .cloned()
        .inspect(|x| println!("{:?}", x));

    Some(zero)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::distributions::Uniform;
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    use rand::Rng;
    use std::collections::HashMap;
    use std::ops::Index;

    #[test]
    fn test_push_str() {
        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push_str(s2);
        println!("s2 is {}", s2);
    }

    #[test]
    fn test_plus_str() {
        let s1 = String::from("Hello, ");
        let s2 = "world!".to_string();
        let s3 = s1 + &s2;

        println!("s1 has been moved (consumed) s2: {} s3: {}", s2, s3);
    }

    #[test]
    fn test_hash_map() {
        let teams = vec![String::from("Blue"), String::from("Yellow")];
        let initial_scores = vec![10, 50];

        let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        let team_name_blue = String::from("Blue");
        let team_name_yellow = String::from("Yellow");
        let score = scores.get(&team_name_blue);

        println!("Score: {:?}", score);

        for elem in &scores {
            println!("{}: {}", elem.0, elem.1);
            let (key, value) = elem;
            println!("{}: {}", key, value);
        }

        for (key, value) in &scores {
            println!("{}: {}", key, value);
        }

        scores.insert(String::from("Blue"), 25);

        scores.entry(team_name_blue).or_insert(77);
        scores.entry(String::from("Green")).or_insert(88);

        println!("{:?}", scores);

        let text = "hello world wonderful world";

        let mut word_map = HashMap::new();

        for word in text.split_whitespace() {
            let count = word_map.entry(word).or_insert(0);
            *count += 1;
        }

        println!("{:?}", word_map);
    }

    #[test]
    fn test_collection_ex() {
        let rand_list_gen = RandGenerator::new(Uniform::from(0..20));

        let mut odd_list: Vec<_> = rand_list_gen.take(7).collect();

        let mut ints_odd = vec![1, 4, 7, 8, 10];
        let mut ints_even = vec![1, 2, 3, 4];

        ints_odd.shuffle(&mut thread_rng());
        ints_even.shuffle(&mut thread_rng());

        let median_odd = median(&mut ints_odd);
        let median_even = median(&mut ints_even);

        assert_eq!(median_odd.unwrap(), 7 as f64);
        assert_eq!(median_even.unwrap(), 2.5);

        let median_ran_odd = median(&mut odd_list);
        println!("List {:?}: ", odd_list);
        println!("Median {}: ", median_ran_odd.unwrap());
    }

    #[test]
    fn test_mean() {
        let list = vec![1, 2, 3, 4, 5];

        let mean = mean_r(&list);

        assert_eq!(mean.unwrap(), 3.0);
    }

    #[test]
    fn test_count_digits() {
        assert_eq!(count_digits(45), 2);

        assert_eq!(count_digits(-45), 3);
    }
}
