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

fn shoot_puck(hockey_player: HockeyPlayer, seconds_remaining: u16) {
    if seconds_remaining < 300 {
        match hockey_player.position {
            HockeyPosition::Center => println!("Goal!"),
            _ => println!("Miss!"),
        }
    } else {
        println!("Goal!");
    }
}

fn main() {
    let mut player = HockeyPlayer {
        name: String::from("Bryan Rust"),
        number: 17,
        position: HockeyPosition::Wing,
        goals_ytd: 7,
    };

    shoot_puck(player, 1000);
}