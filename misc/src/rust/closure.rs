use log::*;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

#[derive(Clone)]
pub struct Cacher<T, U>
where
    T: Fn(U) -> U,
    U: Copy,
{
    calculation: T,
    value: HashMap<U, U>,
}

impl<T, U> Cacher<T, U>
where
    T: Fn(U) -> U,
    U: Copy + std::cmp::Eq + std::hash::Hash,
{
    pub fn new(calculation: T) -> Cacher<T, U> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    pub fn value(&mut self, arg: U) -> U {
        match self.value.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}

pub fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num: u32| {
        info!("calculating slowly... ");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        info!("Today, do {} pushups!", expensive_result.value(intensity));
        info!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            info!("Take a break today! Remember to stay hydrated!");
        } else {
            info!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use env_logger;
    use rand::distributions::{Distribution, Uniform};
    use std::io;
    use std::io::Write;
    use std::str::FromStr;

    #[test]
    fn exercise_generate_workout() {
        let simulated_user_specified_value = 10;
        let simulated_random_number = 7;

        generate_workout(simulated_user_specified_value, simulated_random_number);
    }

    /// command line interface to exercise generator.
    /// set RUST_LOG=info
    /// ```ps
    /// $Env:RUST_LOG=info
    /// $Env:RUST_LOG="rust::closure::tests=info"
    /// cargo test -- --nocapture --ignored test_interface_workout
    ///
    /// ```
    /// Set is_test(true) to work around cargo test and termcolor
    /// <https://github.com/env-logger-rs/env_logger/issues/107>
    ///
    ///
    fn init() {
        let _ = env_logger::builder()
            .is_test(true)
            .filter_level(LevelFilter::Info)
            .format_level(false)
            .format(|buf, record| writeln!(buf, "{}", record.args()))
            .try_init();
    }

    #[ignore]
    #[test]
    fn test_interface_workout() {
        init();

        let mut intensity = String::new();

        info!("Workout Generator.");

        loop {
            info!("Enter a number between 1 and 50!");

            io::stdin()
                .read_line(&mut intensity)
                .expect("Failed to read line.");

            if intensity.trim() == "q" {
                break;
            }
            info!("Your intensity: {}", intensity);
            let i = u32::from_str(intensity.trim()).unwrap_or(25);

            let mut rng = rand::thread_rng();
            let die = Uniform::from(1..4);

            generate_workout(i, die.sample(&mut rng));
            intensity.clear();
        }
    }

    #[test]
    fn test_move() {
        let x = vec![1, 2, 3];

        let equal_to_x = move |z: Vec<i32>| z == x;

        // println!("no x for us: {:?}", x);
        let y = vec![1, 2, 3];

        assert!(equal_to_x(y));
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }

    #[test]
    fn test_iter_map() {
        let v1: Vec<i32> = vec![1, 2, 3];

        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

        assert_eq!(v2, vec![2, 3, 4]);
    }
}
