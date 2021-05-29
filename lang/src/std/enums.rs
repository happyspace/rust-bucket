pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Coin {
    pub fn value_in_cents(&self) -> u8 {
        use Coin::*;

        match *self {
            Penny => 1,
            Nickel => 5,
            Dime => 10,
            Quarter => 25,
        }
    }

    pub fn is_cent(&self) -> bool {
        matches!(*self, Coin::Penny)
    }
}

pub fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
