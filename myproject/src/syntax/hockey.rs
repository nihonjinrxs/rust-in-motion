pub enum HockeyPosition {
    Center,
    Wing,
    Defense,
    Goalie,
}

pub struct HockeyPlayer {
    pub name: String,
    pub number: u8,
    pub position: HockeyPosition,
    pub goals_ytd: u8,
}

impl HockeyPlayer {
    pub fn new(name: String, number: u8, position: HockeyPosition) -> HockeyPlayer {
        HockeyPlayer {
            name: name,
            number: number,
            position: position,
            goals_ytd: 0,
        }
    }

    pub fn shoot_puck(&mut self, seconds_remaining: u16) {
        if seconds_remaining < 300 {
            match self.position {
                HockeyPosition::Center => {
                    self.goals_ytd += 1;
                    println!("Goal!");
                },
                _ => println!("Miss!"),
            }
        } else if self.goals_ytd > 1 {
            self.goals_ytd += 1;
            println!("Goal!");
        } else {
            println!("Miss!");
        }
    }    
}
