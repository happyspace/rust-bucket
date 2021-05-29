enum Foo {
    Bar,
    Baz,
    Quix(u32),
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn iffy() {
        let number = 3;

        if number < 5 {
            println!("too true")
        } else {
            println!("too not")
        }
    }

    #[test]
    fn loopy() {
        let mut counter = 0;

        let result = loop {
            counter += 1;

            if counter == 10 {
                break counter * 2;
            }
        };

        println!("The result is {}", result);
    }

    #[test]
    fn whiley() {
        let mut number = 3;

        while number != 0 {
            println!("{}!", number);

            number -= 1;
        }

        println!("Liftoff!!!");
    }

    #[test]
    fn itery() {
        let a = [10, 20, 30, 40, 50];

        for element in a.iter() {
            println!("the value is: {}", element);
        }
    }

    #[test]
    fn while_let() {
        let mut optional = Some(0);

        while let Some(i) = optional {
            if i > 9 {
                println!("Greater than 9, quit!");
                optional = None;
            } else {
                println!("'i' is '{:?}'. Try again.", i);
                optional = Some(i + 1);
            }
        }
    }

    #[test]
    fn if_let() {
        let number = Some(7);
        let letter: Option<i32> = None;
        let emoticon: Option<i32> = None;

           // The `if let` construct reads: "if `let` destructures `number` into
    // `Some(i)`, evaluate the block (`{}`).

        if let Some(i) = number {
            println!("Match {:?}", i);
        }

        if let Some(i) = letter {
            println!("Match {:?}", i);
        } else {
            // Destructure failed. Change to the failure case.
            println!("Didn't match a number. Let's go with a letter!");
        }

        let i_like_letters: bool = false;

        if let Some(i) = emoticon {
            println!("Matched {:?}!", i);
            // Destructure failed. Evaluate an `else if` condition to see if the
            // alternate failure branch should be taken:

        } else if i_like_letters {
            println!("Didn't match a number. Let's go with a letter!");
        } else {
            // The condition evaluated false. This branch is the default:
            println!("I don't like letters. Let's go with an emoticon :)!");
        }
    }

    #[test]
    fn if_let_enum() {
        let a: Foo = Foo::Bar;
        let b = Foo::Baz;
        let c = Foo::Quix(100);

        // Variable a matches Foo::Bar
        if let Foo::Bar = a {
            println!("a is foobar");
        }

        if let Foo::Bar = b {
            println!("b is foobar");
        }

        if let Foo::Quix(value) = c {
            println!("c is {}", value);
        }

        // Binding also works with 'if let'
        if let Foo::Quix(value @ 100) = c {
            println!("c is one hundered {}", value);
        }


    }
}
