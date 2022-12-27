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
        Self { cost, food_type }
    }
}

fn main() {
    let my_food = Food::new(10, FoodType::Vegi);
}
