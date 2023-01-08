use std::fmt::{Debug, Display};

trait DisplayState {
    //provide required type
    fn load_state(&self)
    where
        Self: Debug,
    {
        println!("status : {:?}", self);
    }

    fn load_state_with_display(&self)
    where
        Self: Debug + Display,
    {
        println!("status with display : {}", self);
    }
}

#[derive(Debug)]
struct Jerry {}
#[derive(Debug)]
struct Tom {}

impl Display for Tom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

// <T> is free!
impl<T> DisplayState for T {}

fn main() {
    let mouse = Jerry {};
    let cat = Tom {};

    mouse.load_state(); // expected Jerry
    #[cfg(panic)]
    mouse.load_state_with_display();

    let something_string = "hello".to_string();
    something_string.load_state(); // expected "hello"
    something_string.load_state_with_display(); // expected hello

    cat.load_state(); // expected Tom
    cat.load_state_with_display(); //aborted. struct haven't attribute
}
