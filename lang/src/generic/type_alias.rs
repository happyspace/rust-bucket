#[cfg(test)]
mod test {

    #[test]
    fn type_alias() {
        type Thunk = Box<dyn Fn() + Send + 'static>;

        let f: Thunk = Box::new(|| println!("Hi"));

        fn takes_long_type(f: Thunk) {
            (*f)();
        }

        fn returns_long_type() -> Thunk {
            Box::new(|| println!("Hi"))
        }

        takes_long_type(f);
        takes_long_type(returns_long_type());
    }
}
