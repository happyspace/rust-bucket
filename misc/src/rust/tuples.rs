use std::fmt;

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;

    (boolean, integer)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Matrix {
    fn transpose(self: Self) -> Matrix {
        let Matrix(a, b, c, d) = self;

        Matrix(a, c, b, d)
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Matrix(a, b, c, d) = self;
        write!(f, "({} {})\n({} {})", a, b, c, d)
    }
}

struct List(Vec<i32>);

/// https://doc.rust-lang.org/stable/rust-by-example/hello/print/print_display/testcase_list.html
/// TODO: finish  
impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, " Moo")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_matrix_fmt() {
        let m = Matrix(0_f32, 0.0, 0.0, 0.0);
        println!("{}", Matrix(0_f32, 0.0, 0.0, 0.0));
        assert_eq!("(0 0)\n(0 0)", m.to_string());
    }

    #[test]
    fn test_reverse() {
        let pair = (1_i32, true);
        let rev = reverse(pair);
        assert_eq!(rev, (true, 1_i32));
    }

    #[test]
    fn test_transpose() {
        let m = Matrix(1_f32, 2.0, 3.0, 4.0);
        let m_prime = m.transpose();
        assert_eq!("(1 3)\n(2 4)", m_prime.to_string());
    }
}
