use std::convert::From;

// by defining From, Into is also defined
#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let num = Number::from(30);
    println!("My number is {:?}", num);
    // when using Into, you need to specify type! or else the compiler complains
    let number = 10;
    let num2 : Number = number.into();
    println!("My number is also: {:?}", num2);
}