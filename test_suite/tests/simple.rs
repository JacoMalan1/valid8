use valid8::Validate;

#[test]
fn simple_test() {
    #[derive(Debug, Clone, Validate)]
    struct User {
        #[validate(required)]
        username: String,
        #[validate(required, email)]
        email: String,
        #[validate(required)]
        password: String,
    }
    let user = User {
        username: String::from("Hello World"),
        email: String::from("a@b.c"),
        password: String::from("1234"),
    };

    user.validate().unwrap();
}

#[test]
fn test_min_str() {
    #[derive(Debug, Clone, Validate)]
    struct User {
        #[validate(min(5), required)]
        password: String,
    }

    let user = User {
        password: String::from("1234"),
    };

    assert!(matches!(
        user.validate(),
        Err(valid8::ValidationError::Min(_))
    ));

    let user = User {
        password: String::from("12345"),
    };
    user.validate().unwrap();
}
