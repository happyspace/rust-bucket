/// Random links on macro invocation
///
/// <https://github.com/rust-lang/rfcs/blob/master/text/0378-expr-macros.md>
/// discussion
/// <https://users.rust-lang.org/t/braces-square-brackets-and-parentheses-in-macro-call/9984/4>
///

#[macro_export]
macro_rules! vec {
  // moo
  ( $( $x:expr ),* ) => {

    {
      let mut temp_vec = Vec::new();
      $(
        println!("moo { }", $x );
        temp_vec.push($x);
      )*

      temp_vec
    }
  };
}
