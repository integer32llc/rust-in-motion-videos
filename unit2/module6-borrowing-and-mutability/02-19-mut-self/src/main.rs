#[derive(Debug)]
pub struct CarPool {
    passengers: Vec<String>,
}

impl CarPool {
    /// Add the named passenger to the carpool
    pub fn pick_up(&mut self, name: String) {
        self.passengers.push(name);
    }
}

fn main() {
    let mut monday_car_pool = CarPool {
        passengers: vec![],
    };

    monday_car_pool.pick_up(String::from("Jake"));
    println!("Carpool state: {:?}", monday_car_pool);

    monday_car_pool.pick_up(String::from("Carol"));
    println!("Carpool state: {:?}", monday_car_pool);
}
