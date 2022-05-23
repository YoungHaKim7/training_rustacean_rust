use std::error::Error;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;

#[derive(Debug)]
enum CompanyError {
    CouldntConnect,
    NotEnoughData,
    UserTimeOut,
}

impl Display for CompanyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Got a CompanyError")
    }
}

#[derive(Debug)]
struct BaseError;

fn give_error(is_company_error: bool) -> Box<dyn Error> {
    if is_company_error {
        Box::new(CompanyError::CouldntConnect)
    } else {
        Box::new(BaseError)
    }
}

impl Display for BaseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Got a BaseError")
    }
}

impl Error for CompanyError {}

impl Error for BaseError {}

// fn try_to_make_number(int_input: &str, float_input: &str) -> Result<(), ParseIntError> {
//     let my_number = int_input.parse::<i32>()?;
//     let other_number = float_input.parse::<f32>()?;
//     Ok(())
// }

fn main() {
    let error_1 = give_error(true);
    let error_2 = give_error(false);

    println!("{error_1}, {error_2:?}");
}
