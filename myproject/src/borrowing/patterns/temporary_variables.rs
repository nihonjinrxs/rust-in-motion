// Basic idea: end a borrow and get a result before doing another borrow

#[derive(Debug)]
pub struct Player {
    score: i32,
}

impl Player {
    pub fn set_score(&mut self, new_score: i32) {
        self.score = new_score;
    }

    pub fn score(&self) -> i32 {
        self.score
    }

    pub fn new() -> Self {
        Player { score: 0 }
    }
}

pub fn run() {
    let mut player1 = Player::new();

    // Used to not compile because inner player1.score() call is mutable at same time as mutating
    player1.set_score(player1.score() + 1);

    // Fix looked like
    let old_score = player1.score();
    player1.set_score(old_score + 1);
    
    println!("Player is {:?}", player1);
}
