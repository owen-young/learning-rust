use std::mem;

// This function borrows a slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    // Fixed-size array (type signature is superfluous)
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    /* with the below code (and direct references commented out)
     * the code will compile, but it will crash when analyze_slice
     * is called. looks like rust can't predict everything....
     * let xs = [];
     * comment out second array thing
     */

    // All elements can be initialized to the same value
    let ys: [i32; 500] = [0; 500];

    // Indexing starts at 0
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // `len` returns the count of elements in the array
    println!("number of elements in array: {}", xs.len());

    // Arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // Slices can point to a section of an array
    // They are of the form [starting_index..ending_index]
    // starting_index is the first position in the slice
    // ending_index is one more than the last position in the slice
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);

    // Out of bound indexing causes compile error
    // println!("{}", xs[5]);

    // printing an array using iter()
    for element in xs.iter() {
        println!("the value is: {}", element);        
    }

    // it is recommended to use Vec::with_capacity whenever possible to specify how big the vector is expected to get.

    let mut lst = vec![0, 1];
    lst.push(2);
    println!("vec after pushing {:?}", lst);
    // let's change vec!
    for element in lst.iter_mut() {
        println!("element: {}", element);
        *element+= 10;
    }
    println!("vec after adding 10 to each {:?}", lst);

    // cannot use iter_mut for this, compiler denies it cannot be borrowed as mutable. 
    // it passes a reference to itself (borrows), type signature of iter_mut is &mut self, 
    // but this is just &self. doing *element vs element has no effect, because element
    // is automatically dereferenced. however, this does not work in the above example
    // with *element+=10.
    let lst = vec![2, 3];
    for element in lst.iter() {
        println!("element is: {}", element);
    }
}
