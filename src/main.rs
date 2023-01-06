#[derive(Debug)]
struct Human {
    is_man: bool,
    age: u8,
}
#[derive(Debug)]
struct State {
    population: Vec<Human>,
}

impl Human {
    fn new(is_man: bool, age: u8) -> Self {
        Self { is_man, age }
    }
}
//implement From and you can also use into function
impl From<Vec<Human>> for State {
    fn from(value: Vec<Human>) -> Self {
        Self { population: value }
    }
}

impl State {
    fn print_age(&self) {
        for human in &self.population {
            println!("{}", human.age);
        }
    }
}

fn main() {
    let windy = Human::new(true, 8);
    let andy = Human::new(true, 10);
    let annie = Human::new(false, 12);

    let maple_town = State::from(vec![windy, andy, annie]);
    maple_town.print_age();
}
