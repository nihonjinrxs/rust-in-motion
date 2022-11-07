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
            // close the door
        },
        (DoorState::Closed, DoorAction::Open) => {
            // open the door
        },
        // if you get here, something is very wrong, so this will panic!
        _ => unreachable!(),
    }
}

pub fn run() {
    // Panics! Invalid input!
    take_action(DoorState::Opened, DoorAction::Open);
}
