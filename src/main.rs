trait display_state {
    fn to_string(&self) {
        println!("status window");
    }
}

struct Jerry {}
struct Tom {}
//implement all type. if exist built-in function you must annotate what you use.
impl<T> display_state for T {}
fn main() {
    let mouse = Jerry {};
    let cat = Tom {};
    mouse.to_string();
    cat.to_string();
    let something_string = "hello".to_string();

    //instead of ToString::to_string trait
    display_state::to_string(&something_string); // candidate #2
}
