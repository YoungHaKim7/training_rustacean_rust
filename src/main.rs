trait SomeCustomTrait {
    fn blah_blah(&self, a: &str, b: &str) -> String;
}


#[allow(debug_code)]
fn do_this<T>(some_var: &T) -> String
where T: SomeCustomTrait {
    // Some complex logic
    some_var.blah_blah("first", "second")
}

fn do_this2(some_var: &dyn SomeCustomTrait) -> String {
    // Some complex logic..
    println!("{:?}", some_var);
    some_var.blah_blah("first", "second")
}

fn main() {
    
}
