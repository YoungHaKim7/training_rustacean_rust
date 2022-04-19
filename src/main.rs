use std::borrow::Cow;

#[derive(Debug)]
struct User<'a> {
    name: Cow<'a, str>,
}

impl User<'_> {
    fn is_borrowed(&self) {
        match &self.name {
            Cow::Borrowed(name) => println!("It's Borrowed: {name}"),
            Cow::Owned(name) => println!("It's owned: {name}"),
        }
    }
}

fn main() {
    let user_1 = User {
        name: "user_1".into(),
    };

    let user_2 = User {
        name: "User 2".to_string().into(),
    };

    user_1.is_borrowed();
    user_2.is_borrowed();
}
