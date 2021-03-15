#![allow(dead_code)]
use crate::List::*;

enum List {
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    // A Box is essentially a pointer to the next Node
    Cons(u32, Box<List>),
    // Nil: A node that signifies the end of the linked list
    Nil,
}

// function prints the type of a variable
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

// Methods can be attached to an enum
impl List {
    // Create an empty list
    fn new() -> List {
        // `Nil` has type `List`
        Nil
    }

    // Consume a list, and return the same list with a new element at its front
    fn prepend(self, elem: u32) -> List {
        // `Cons` also has type List
        Cons(elem, Box::new(self))
    }

    // Return the length of the list
    fn len(&self) -> u32 {
        // `self` has to be matched, because the behavior of this method
        // depends on the variant of `self`
        // `self` has type `&List`, and `*self` has type `List`, matching on a
        // concrete type `T` is preferred over a match on a reference `&T`
        match *self {
            // Can't take ownership of the tail, because `self` is borrowed;
            // instead take a reference to the tail
            Cons(_, ref tail) => 1 + tail.len(),
            // Base Case: An empty list has zero length
            Nil => 0
        }
    }

    // Return representation of the list as a (heap allocated) string
    fn stringify(&self) -> String {
        match *self {
            // Take reference to tail instead of just the tail, since `self`
            // is borrowed
            Cons(head, ref tail) => {
                // `format!` is similar to `print!`, but returns a heap
                // allocated string instead of printing to the console
                // tail is a &alloc::boxed::Box<linkedlist::List>, references to boxes can call methods too!
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }

    // Consume the previous list in order to make a new one, since our tails
    // are not mutable. This allows us to take ownership of everything.
    fn append(self, elem: u32) -> List {
        match self {
            // This function returns a new list, but list must be structured
            // correctly. We are taking ownership of tail, so we can directly
            // access the Box. 
            // tail is a alloc::boxed::Box<linkedlist::List>, Boxes can call methods!
            Cons(head, tail) => Cons(head, Box::new(tail.append(elem))),
            Nil => Cons(elem, Box::new(Nil))
        }
    }
}

fn main() {
    // Create an empty linked list
    let mut list = List::new();

    // Prepend some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);
    list = list.append(13);
    list = list.append(5);

    // Show the final state of the list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());

    // Create a list in a box
    let mut box_list = Box::new(List::new());
    println!("boxed linked list has length: {}", box_list.len());
    // append some elements, however, they each need to go in their own box.
    // Boxes behave like things on the heap in C, you can still call methods on
    // them and stuff, the syntax is just the same as stack variables, which is..different
    box_list = Box::new(box_list.append(5));
    box_list = Box::new(box_list.append(7));
    println!("{}", box_list.stringify());
    println!("boxed linked list has length: {}", box_list.len());
}
