#[allow(dead_code)]
#[derive(Debug)]
struct DougsStruct<T, U> {
    dougs_t: T,
    dougs_u: U,
}

impl<T, U> DougsStruct<T, U>
where
    T: std::fmt::Debug,
    U: std::fmt::Debug,
{
    fn log_something(&self) {
        println!("{:?}  {:?}", self.dougs_t, self.dougs_u);
    }
}

fn main() {
    let test = DougsStruct {
        dougs_t: 5.6,
        dougs_u: vec![1, 2, 3],
    };

    test.log_something();
}
