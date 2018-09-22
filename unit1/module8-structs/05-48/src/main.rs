enum Clock {
    Sundial { hours: u8 },
    Digital { hours: u8, minutes: u8 },
    Analog { hours: u8, minutes: u8, seconds: u8 },
}

fn main() {
    let clock = Clock::Analog {
        hours: 9,
        minutes: 25,
        seconds: 46,
    };
}