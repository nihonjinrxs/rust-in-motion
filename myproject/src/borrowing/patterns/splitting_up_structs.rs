// Basic idea: Methods only borrow the fields they need

// Non-working

// pub struct Monster {
//     hp: u8,
//     sp: u8,
//     friends: Vec<Friend>,
// }

// pub struct Friend {
//     loyalty: u8,
// }

// impl Monster {
//     pub fn final_breath(&mut self) {
//         if let Some(friend) = self.friends.first() {
//             // self mutably borrowed, already using friend, can't re-borrow mutably
//             self.heal(friend.loyalty);
//             println!("Healing for {}", friend.loyalty);
//         }
//     }

//     pub fn heal(&mut self, amount: u8) {
//         self.hp += amount;
//         self.sp -= amount;
//     }
// }

// Fix: split up structs into things used together

pub struct Monster {
    stats: Stats,
    friends: Vec<Friend>,
}

pub struct Stats {
    hp: u8,
    sp: u8,
}

pub struct Friend {
    loyalty: u8,
}

impl Monster {
    pub fn final_breath(&mut self) {
        if let Some(friend) = self.friends.first() {
            // self mutably borrowed, already using friend, can't re-borrow mutably
            self.stats.heal(friend.loyalty);
            println!("Healing for {}", friend.loyalty);
        }
    }

}

impl Stats {
    pub fn heal(&mut self, amount: u8) {
        self.hp += amount;
        self.sp -= amount;
    }
}

pub fn run() {
    // Non-working
    // let mut monster = Monster {
    //     hp: 63,
    //     sp: 26,
    //     friends: vec![
    //         Friend{ loyalty: 6 },
    //     ],
    // };
    
    // Fixed
    let mut monster = Monster {
        stats: Stats { hp: 63, sp: 26 },
        friends: vec![
            Friend { loyalty: 6 },
        ],
    };

    monster.final_breath();
}
