// we can define enums (enumerated types) pretty easily

// list of all possible values
#[derive(Debug)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

// we can use enums in enums, we can store data in enums
enum Action {
    Movement(Move),
    Rotate {d: i16},
    Nothing,
}

// we can associate function to enums
impl Action {
    fn describe(&self){
        match self {
            & Action::Movement(ref Move) => {
                println!("I am moving {:?}.", Move);
            },
            & Action::Rotate{d} => println!("I am changing direction by {} degrees.", d),
            & Action::Nothing => println!("I am resting."),
        }
    }
}

fn main() {
    let move_up = Move::Up;

    let next_action = Action::Movement(move_up);
    let rotation = Action::Rotate{d: -90};

    next_action.describe();
    rotation.describe();
}
