// Same definition as in module 7
enum HockeyPosition {
    Center,
    Wing,
    Defense,
    Goalie,
}

struct HockeyPlayer {
    name: String,
    number: u8,
    position: HockeyPosition,
    goals_ytd: u8,
}

impl HockeyPlayer {
    fn shoot_puck(&mut self, seconds_remaining: u16) {
        if seconds_remaining < 300 {
            match self.position {
                HockeyPosition::Center => {
                    self.goals_ytd += 1;
                    println!("Goal!");
                },
                _ => println!("Miss!"),
            }
        } else {
            self.goals_ytd += 1;
            println!("Goal!");
        }
    }
}

fn main() {
    let mut player = HockeyPlayer {
        name: String::from("Bryan Rust"),
        number: 17,
        position: HockeyPosition::Wing,
        goals_ytd: 7,
    };

    player.shoot_puck(1000);
    player.shoot_puck(900);

    println!("{} has scored {} goals this season",
        player.name,
        player.goals_ytd,
    );
}
