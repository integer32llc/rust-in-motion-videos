enum HockeyPosition {
    Center,
    Wing,
    Defense,
    Goalie,
}

fn next_player(position: HockeyPosition) {
    // code that would do something like look up
    // another player at the position specified
}

fn main() {
    let position = HockeyPosition::Defense;
    next_player(position);
}
