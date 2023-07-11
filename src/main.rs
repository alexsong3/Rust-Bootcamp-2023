fn main() {
    // println!("Hello, Rust Bootcamp by VBI Academy!");
    // exercise1_should_work()

    // exercise4()
    // exercise5()

    // println!("111 {:#?}", Direction::North.opposite() == Direction::North);
    // assert_eq!(Direction::North.opposite(), Direction::South);
    //     assert_eq!(Direction::East.opposite(), Direction::East);

    let mut state = State {
        quit: false,
        position: Point { x: 0, y: 0 },
        color: (0, 0, 0),
    };
    state.process(Message::ChangeColor(255, 0, 255));
    state.process(Message::Echo(String::from("hello world")));
    state.process(Message::Move(Point { x: 10, y: 15 }));
    state.process(Message::Quit);

    assert_eq!(state.color, (255, 0, 255));
    assert_eq!(state.position.x, 10);
    assert_eq!(state.position.y, 15);
    assert_eq!(state.quit, true);
}
// #[derive(Debug, PartialEq)]
// struct Person {
//     name: String,
//     age: u8,
//     hobby: String,
// }
// // impl Person {
//     fn exercise1() -> Person {
//         let age = 30;
//         // Hobby = Rust
//         let p = Person {
//             name: String::from("sunface"),
//             age,
//             hobby: String::from("Rust"),
//         };

//         p
//     }
// // }

// fn exercise1_should_work() {
//     let p = exercise1();
//     println!("p1: {:#?}", p);
//     let p_expectation = Person {
//         name: String::from("sunface"),
//         age: 30,
//         hobby: String::from("Rust"),
//     };
//     println!("p2: {:#?}", p_expectation);
//     assert_eq!(p, p_expectation);
//     println!("res: {:?}", p_expectation == p)
// }

// #[derive(Debug, Clone)]
// struct User {
//     first: String,
//     last: String,
//     age: u32,
// }

// fn exercise4() {
//     let u1 = User {
//         first: String::from("John"),
//         last: String::from("Doe"),
//         age: 22,
//     };

//     let u2 = User {
//         first: String::from("Mary"),
//         ..u1.clone()
//     };

//     // println!("user: {:#?}", u2);
//     println!("user: {:#?}", u1);
// }

// #[derive(Debug, Clone)]
// struct Foo {
//     str_val: String,
//     int_val: i32,
// }

// fn exercise5() {
//     let mut foos = Vec::new();
//     foos.push(Foo {
//         str_val: "ten".to_string(),
//         int_val: 10,
//     });
//     foos.push(Foo {
//         str_val: "twenty".to_string(),
//         int_val: 20,
//     });
//     println!("user: {:?}", foos);

//     let moved = &foos[0];
//     println!("mov: {:?}", moved);

//     let moved_field = &foos[0].str_val;
// }

// #[derive(Debug, PartialEq)]
// enum Direction {
//     North,
//     East,
//     South,
//     West,
// }

// impl Direction {
//     fn opposite(&self) -> Direction {
//         println!("aaa {:#?}", self);
//         println!("aaa {:#?}", &Direction::North);
//         println!("aaa {:#?}", self == &Direction::North);
//         match self {
//             //TODO
//             Direction::North => Direction::North,
//             Direction::East => Direction::East,
//             Direction::South => Direction::South,
//             Direction::West => Direction::West,
//             // _ => None,
//         }
//     }
// }

#[derive(Debug, PartialEq)]
enum Message {
    // TODO: implement the message variant types based on their usage below
    ChangeColor(i32, i32, i32),
    Echo(String),
    Move(Point),
    Quit,
}

#[derive(Debug, PartialEq)]
struct Point {
    x: u8,
    y: u8,
}

#[derive(Debug)]
struct State {
    color: (u8, u8, u8),
    position: Point,
    quit: bool,
}

impl State {
    fn change_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn echo(&self, s: String) {
        println!("{}", s);
    }

    fn move_position(&mut self, p: Point) {
        self.position = p;
    }

    fn process(&mut self, message: Message) {
        // TODO: create a match expression to process the different message variants
        // Remember: When passing a tuple as a function argument, you'll need extra parentheses: fn function((t, u, p, l, e))
        println!("aaa {:#?}", self);
        // println!("bbb {:#?}", Message);
        println!("ccc {:#?}", message);

        match self {
            // Message::ChangeColor => self.change_color(message),
            Message::Echo => self.echo(message(0)),
            Message::Move => self.move_position(),
            Message::Quit => self.quit(),
        }
    }
}
