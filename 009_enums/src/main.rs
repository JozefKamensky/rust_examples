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
    Rest(u8),
}

// we can associate function to enums
impl Action {
    fn describe(&self){
        match self {
            & Action::Movement(ref movement) => {
                println!("I am moving {:?}.", movement);
            },
            & Action::Rotate{d} => println!("I am changing direction by {} degrees.", d),
            & Action::Nothing => println!("I am waiting."),
            & Action::Rest(t) => println!("I am resting for {} turns.", t),
        }
    }
}

fn main() {
    let move_up = Move::Up;
    let move_left = Move::Left;
    let move_right = Move::Right;
    let move_down = Move::Down;

    let mut next_action = Action::Movement(move_up);
    let rotation = Action::Rotate{d: -90};
    let resting = Action::Rest(5);
    let nothing = Action::Nothing;

    next_action.describe();

    next_action = Action::Movement(move_left);
    next_action.describe();

    next_action = Action::Movement(move_down);
    next_action.describe();

    next_action = Action::Movement(move_right);
    next_action.describe();

    rotation.describe();
    resting.describe();
    nothing.describe();

    let input: Option<i32> = Option::Some(10);
    // let input: Option<i32> = Option::None;

    match input {
        Some(i) => println!("Variable is initialized, value: {}", i),
        None => println!("Variable is None."),
    }
}
