use std::fmt;

struct Circle {
    radius: i32
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn main() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());
    println!("{}", circle);

    let parsed: i32 = "5".parse().unwrap();
    // the "unwrap" function takes the value out of "Ok"
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);

    let wrapped = "532".parse::<u32>();
    println!("wrapped: {:?}", wrapped);

    let bad = "35a32".parse::<i32>();
    println!("bad: {:?}", bad);
    match bad {
        Ok(num) => println!("Our num is: {}", num),
        Err(ref msg) => println!("Our error is: {}", msg)
    }
    
    // the below two lines , when uncommented, crash the program
    /* let sum2 = bad.unwrap() + turbo_parsed;
    println!("bad sum: {:?}", sum2); */
}
