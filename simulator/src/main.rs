extern crate rand;

// "Lifetime of the return value of the simulate_game method is related to both
//  the home and away parameters" (makes it related to the shorter-lived one)
fn simulate_game<'a>(home: &'a str, away: &'a str) -> &'a str {
    if rand::random() {
        home
    } else {
        away
    }
}

fn main() {
    let team1 = String::from("Panthers");
    {
        let team2 = String::from("Yellow Jackets");
        let winner = simulate_game(&team1, &team2);

        println!("{} vs. {}: {} won", team1, team2, winner);
    }

    println!("Can still discuss the {} here", team1);
}

/*
 * Notes:
 * - Can declare multiple lifetime parameters <'a, 'b, 'c>, names in snake_case starting with apostrophe
 * - Can declare lifetime parameters and types in same clause <'a, 'b, T, U> (generic types in UpperCamelCase/PascalCase)
 * - Both lifetime parameters and generic types can be used in the same places
 * - Generic type parameters are generic over types; lifetime parameters are generic over scopes
 */
