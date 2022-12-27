#[derive(Debug)]
struct Food {
    cost: u32,
    food_type: FoodType,
}

#[derive(Debug)]
enum FoodType {
    Beef,
    Vegi,
    Sauce,
}

impl Food {
    fn new(cost: u32, food_type: FoodType) -> Self {
        //constructor method
        Self { cost, food_type }
    }

    fn print(&self) {
        println!("It's {} won. Food type is {:#?}", self.cost, self.food_type);
    }

    fn change_type(&mut self, change_type: FoodType) {
        self.food_type = change_type;
    }
}

fn main() {
    let mut my_food = Food::new(10, FoodType::Vegi);
    my_food.print(); // == Food::print(&my_food) syntactic sugar
    my_food.change_type(FoodType::Beef);
    my_food.print();
}
