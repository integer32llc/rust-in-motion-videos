pub struct Monster {
    hp: u8,
    sp: u8,
    friends: Vec<Friend>,
}

pub struct Friend {
    loyalty: u8,
}

impl Monster {
    pub fn final_breath(&mut self) {
        if let Some(friend) = self.friends.first() {
            self.heal(friend.loyalty);
            println!("Healing for {}", friend.loyalty);
        }
    }

    pub fn heal(&mut self, amount: u8) {
        self.hp += amount;
        self.sp -= amount;
    }
}

fn main() {}
