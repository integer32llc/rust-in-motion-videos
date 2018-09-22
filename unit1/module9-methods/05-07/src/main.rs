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
    fn new(name: String, number: u8, position: HockeyPosition) -> HockeyPlayer {
        HockeyPlayer {
            name: name,
            number: number,
            position: position,
            goals_ytd: 0,
        }
    }
}

fn main() {
    let mut player = HockeyPlayer::new(
        String::from("Bryan Rust"),
        17,
        HockeyPosition::Wing,
    );
}
