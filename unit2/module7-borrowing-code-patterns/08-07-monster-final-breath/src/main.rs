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
            self.hp += friend.loyalty;
            self.sp -= friend.loyalty;
            println!("Healing for {}", friend.loyalty);
        }
    }
}

fn main() {}
