#[derive(Debug)]
struct Animal {
    age: u8,
    name: String,
    weight: u8,
    is_healthy: bool,
}

struct AnimalDescription {
    name: String,
    is_healthy: bool,
}

//destructuring
impl AnimalDescription {
    fn simple_initalization(input: Animal) -> Self {
        let Animal {
            name, is_healthy, ..
        } = input;

        Self { name, is_healthy }
    }
}

fn main() {
    let my_pet = Animal {
        age: 8,
        name: "jjilong".to_string(),
        weight: 12,
        is_healthy: true,
    };

    //destructuring
    let Animal {
        age: a,
        name: b,
        weight: c,
        is_healthy: d,
    } = &my_pet;

    println!(
        "His name is {}. {} years old. and get weight {}, is healthy? {}",
        b, a, c, d
    );

    let another_pet = AnimalDescription::simple_initalization(my_pet);
    println!(
        "His name is {}, is he healthy? {}",
        another_pet.name, another_pet.is_healthy
    );
}
