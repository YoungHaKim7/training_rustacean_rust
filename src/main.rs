// Enforcing Borrowing Rules at Runtime with RefCell<T>
// RefCell<T> and the Interior Mutability Pattern
// Interior Mutability is a design pattern in Rust that allows you to mutate date even when there are immutable references to that data
// Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
// Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows checked at runtime.
// Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.

// Interior Mutability: A Mutable Borrow to an Immutable Value.


fn main () {
    let x =5;
    let y = &mut x;
}
