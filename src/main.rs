trait SomeCustomTrait {
    fn blah_blah(&self, a: &str, b: &str) -> String;
}

#[derive(Debug)]
struct DougsStruct {
    something: i32,
}

impl SomeCustomTrait for DougsStruct {
    fn blah_blah(&self, a: &str, b: &str) -> String {
        self.something.to_string() + " - " + a + " - " + b
    }
}

#[allow(debug_code)]
fn do_this<T>(some_var: &T) -> String
where
    T: SomeCustomTrait,
{
    // Some complex logic
    some_var.blah_blah("first", "second")
}

impl SomeCustomTrait for i32 {
    fn blah_blah(&self, a: &str, b: &str) -> String {
        "i32".to_string() + " - " + a + " - " + b
    }
}

fn main() {
    let test = DougsStruct { something: 1000 };
    let result = do_this(&test);

    let testi32 = 18;
    let result = do_this(&testi32);
}
