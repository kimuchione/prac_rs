#[derive(Debug)]
struct Human<'a> {
    is_man: bool,
    name: &'a str,
}

trait Act {
    //trait type is seems like prototype method in js.
    fn say_hello(&self) {
        println!("Hello!");
    }
    fn say_bye(&self) {
        println!("Bye!");
    }
}

impl<'a> Act for Human<'a> {
    //declare lifetime and implement specific struct have no built-in function
    fn say_hello(&self) {
        println!("Hello {}", self.name);
    }
    fn say_bye(&self) {
        println!("Bye {}", self.name);
    }
}

fn main() {
    let man = Human {
        is_man: true,
        name: "peter",
    };
    man.say_hello();
    man.say_bye();

    assert_eq!(man.say_hello(), println!("Hello peter"));
    assert_eq!(man.say_bye(), println!("Bye peter"));
}
