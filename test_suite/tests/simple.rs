use valid8::Validate;

#[derive(Debug, Clone, Validate)]
struct Tester {
    #[validate(required)]
    username: String,
    #[validate(required, email)]
    email: String,
    #[validate(required)]
    password: String,
}

#[test]
fn simple_test() {
    let user = Tester {
        username: String::from("Hello World"),
        email: String::from("a@b.c"),
        password: String::from("1234"),
    };

    user.validate().unwrap();
}
