pub struct Val {
    val: f64,
}

pub struct GenVal<T> {
    gen_val: T,
}

impl Val {
    pub fn value(&self) -> &f64 {
        &self.val
    }
}

// impl of GenVal for a generic type 'T'
impl<T> GenVal<T> {
    pub fn value(&self) -> &T {
        &self.gen_val
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn value_test() {
        let x = Val { val: 3.0 };
        let y = GenVal { gen_val: 3i32 };

        println!("{}, {}", x.value(), y.value());
    }
}
