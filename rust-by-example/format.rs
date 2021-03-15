fn main() {
    println!("{0} + {1} = {1} + {0}", 4, 6);
    println!("{a} {b} {c}",
             a="B)",
             b=":)",
             c="8)");
    println!("{0} in binary is {0:b}",12);
    // You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    println!("{number:>width$}", number=1, width=6);

    // You can pad numbers with extra zeroes. This will output "000001".
    println!("{number:>0width$}", number=1, width=6);
    println!("debug? {:?}",2355);
    let pi = 3.141592;
    println!("Pi is roughly {:.3}", pi);
    println!("Pi is roughly {:.*}", 3, pi);
    println!("Pi is roughly {num:.prec$}", num=pi, prec=3);
}
