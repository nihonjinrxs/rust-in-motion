use std::io;

fn main() {
    println!("Hello, world!");

    // variables
    {
        let mut x = 5;
        let y = 6;
        let z = x + y;
        println!("z is {}", z);
        x += 1;
        println!("x + y is now {}", x + y);
    }

    // Tuples
    {
        let tup = (1, 'c', true);
        let (a, b, c) = tup;
        println!("a == tup.0? {}", a == tup.0);
        println!("b == tup.1? {}", b == tup.1);
        println!("c == tup.2? {}", c == tup.2);
    }

    // Arrays
    {
        let mut ary = [7.2, 10.4, 345.01];
        ary[0] = 0.0;
        println!("ary is now {:?}", ary);
        // Slice of array
        let ary_slc = &ary[0..1];
        println!("ary slice is {:?}", ary_slc);
    }

    // String slices
    {
        let str = "hi";
        println!("{}", str);
    }

    // Functions
    {
        next_birthday("Jimmy", 28);
        next_birthday("Jake", 30);
        next_birthday("Vivian", 0);

        println!("{} squared = {}", 3, square(3));
        println!("{} squared = {}", 7, square(7));
        println!("{} squared = {}", 12, square(12));

    }

    // Control Flow
    {
        // if
        discount(10);
        discount(3);

        // loop
        // loop {
        //     let word = get_secret_word();
        //     if word.trim() == "rust" {
        //         break;
        //     }
        // }
        // println!("You know the secret word! Please proceed!");

        // while
        // println!("One more time, this time while looping!");
        // let mut word = String::new();
        // while word.trim() != "rust" {
        //     word = get_secret_word();
        // }
        // println!("You know the secret word! Please proceed!");

        // for
        for i in 1..11 {
            println!("Now serving number {}", i);
        }
    }

    // Match
    {
        let x = 3;
        match x {
            1 => println!("One is the loneliest number"),
            2 => println!("Two's company"),
            3 => println!("Three's a crowd"),
            _ => println!("Some other number"),
        }
    }

    // Enums
    {
        let position = HockeyPosition::Defense;
        next_player(position);

        tell_time(Clock::Analog(9, 25, 45));
    }

    // Structs and Methods
    {
        let mut player = create_player(
            String::from("Bryan Rust"),
            17,
            HockeyPosition::Wing,
            7,
        );
        player.goals_ytd += 1;
        println!("{} has scored {} goals this season!", player.name, player.goals_ytd);

        let triangle1 = Triangle(3, 4, 5);
        is_equilateral(triangle1);

        println!("{:?}", add_distances(Meters(3), Meters(7)));
    }
}

// Functions
fn next_birthday(name: &str, current_age: u8) {
    let next_age = current_age + 1;
    println!("Hi {}, on your next birthday, you'll be {}!", name, next_age);
}

fn square(num: i32) -> i32 {
    num * num
}

fn discount(day_of_month: u8) {
    let amount = if day_of_month % 2 == 0 {
        50
    } else {
        10
    };

    println!("Your discount is {}%!", amount);
}

fn get_secret_word() -> String {
    println!("What's the secret word?");
    let mut word = String::new();
    io::stdin().read_line(&mut word).expect("Failed to read line");
    word
}

enum HockeyPosition {
    Center,
    Wing,
    Defense,
    Goalie,
}
fn next_player(position: HockeyPosition) {
    // code that would do something with the position
    position;
}

enum Clock {
    Sundial(u8),
    Digital(u8, u8),
    Analog(u8, u8, u8),
}
fn tell_time(clock: Clock) {
    match clock {
        Clock::Sundial(hours) => {
            println!("It is about {} o'clock", hours);
        },
        Clock::Digital(hours, minutes) => {
            println!("It is {} minutes past {}", minutes, hours);
        },
        Clock::Analog(hours, minutes, seconds) => {
            println!("It is {} minutes and {} seconds past {} o'clock", minutes, seconds, hours);
        },
    }
}

struct HockeyPlayer {
    name: String,
    number: u8,
    position: HockeyPosition,
    goals_ytd: u8,
}

fn create_player(name: String, number: u8, position: HockeyPosition, goals_ytd: u8) -> HockeyPlayer {
    HockeyPlayer {
        name: name,
        number: number,
        position: position,
        goals_ytd: goals_ytd,
    }
}

struct Triangle(u32, u32, u32);

fn is_equilateral(triangle: Triangle) -> bool {
    triangle.0 == triangle.1 && triangle.1 == triangle.2
}

#[derive(Debug)]
struct Meters(u8); // Guarantees that the u8 is of type "Meters"
fn add_distances(d1: Meters, d2: Meters) -> Meters {
    Meters(d1.0 + d2.0)
}

struct MyStruct;
