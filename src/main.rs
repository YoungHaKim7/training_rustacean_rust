// Treating Smart Pointers Like Regular References with the "Deref" Trait

// Note: there's one big difference between the MyBox<T> type we're about to build and the real Box<T>:our version will not store its data on the heap. We are focusing this example on "Deref", so where the data is actually stored is less important than the pointer-like behavior.


// Following the Pointer to th Value with the Dereference Operator


fn main () {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
