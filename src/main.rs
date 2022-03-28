// Smart Pointers
// A pointer is a general concept for a variable that contains an address in memory.
// In Rust, the different smart pointers defined in the standard library provide functionality beyond that provided by references.
// We've already encountered a few smart pointers in this book, such as "String" & "Vec<T>"in Chapter 8

// We'll cover the most common smart pointers in the standard library:
// "Box<T>" for allocating values on the heap
// "Rc<T>", a reference counting type that enables multiple ownership
// "Ref<T>" & "RefMut<T>", accessed through "RefCell<T>", a type that enforces the borrowing rules at runtime instead of compile time

// Using "Box<T>" to Point to Data on the Heap
fn main() {
    let b = Box::new(5);
    println!("b = {b}");
}
