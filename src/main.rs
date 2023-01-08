use std::fmt::Debug;

//instance must be have a Debug type.
trait display_state: Debug {
    fn to_string(&self) {
        println!("status : {:?}", self);
    }
}

struct Jerry {}
struct Tom {}

impl<T: Debug> display_state for T {}

fn main() {
    let mouse = Jerry {};
    let cat = Tom {};

    #[cfg(panic)]
    mouse.to_string();
    #[cfg(panic)]
    cat.to_string();

    let something_string = "hello".to_string();

    //instead of ToString::to_string trait
    display_state::to_string(&something_string); // candidate #2
}
