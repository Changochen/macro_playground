extern crate the_macro;
use the_macro::{add_extra_field, mockable_derive};

struct X {
    i: i32
}

struct Y {
    i: i32
}

#[mockable_derive]
impl Y {
    pub fn new() -> Self {
        Y {
            i: 0
        }
    }

    pub fn get_i(&mut self) -> i32 {
        self.i
    }

    pub fn compare_other(&self, other: &Y) -> bool {
        self.i == other.i
    }
}

fn main() {
    let mut y = Y::new_mock();
    let _i = y.get_i_mock();
    y.compare_other_mock(&y);
    println!("Hello, world!");
}
