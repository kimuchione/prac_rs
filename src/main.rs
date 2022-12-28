#[derive(Debug)]
struct Food {
    cost: u32,
    food_type: FoodType,
}

#[derive(Debug)]
enum FoodType {
    Beef(String),
    Vegi(String),
    Sauce(String),
}

impl Food {
    fn new(cost: u32, food_type: FoodType) -> Self {
        //constructor method
        Self { cost, food_type }
    }

    fn print(&self) {
        println!(
            "It's {} won. Food type is {}",
            self.cost,
            self.food_type.call_name()
        );
    }

    fn change_type(&mut self, change_type: FoodType) {
        self.food_type = change_type;
    }
}

//implements can place behind or front
impl FoodType {
    fn call_name(&self) -> &String {
        match self {
            FoodType::Beef(anything) => anything,
            FoodType::Vegi(anything) => anything,
            FoodType::Sauce(anything) => anything,
        }
    }
}

fn main() {
    let mut my_food = Food::new(10, FoodType::Vegi(String::from("Cucumber")));
    my_food.print(); // == Food::print(&my_food) syntactic sugar

    my_food.change_type(FoodType::Beef(String::from("Pork")));
    my_food.print();

    my_food.change_type(FoodType::Sauce("Salsa".to_string()));
    my_food.print();
}
