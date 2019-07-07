enum DoorState {
    Opened,
    Closed,
}

enum DoorAction {
    Open,
    Close,
}

fn take_action(current_state: DoorState, action: DoorAction) {
    match (current_state, action) {
        (DoorState::Opened, DoorAction::Close) => {
            // code to close the door goes here
        },
        (DoorState::Closed, DoorAction::Open) => {
            // code to open the door goes here
        },
        // If you get here, a programming mistake has been made
        _ => unreachable!(),
    }
}

fn main() {
    // A call that will go into the first match arm so you can see what it looks like to encounter
    // an unimplemented call
    take_action(DoorState::Closed, DoorAction::Close);
}
