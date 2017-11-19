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
    // value must be of type Move - enum declared earlier
    Movement(Move),
    // value can be named
    Rotate {d: i16},
    Nothing,
    // value can be anonymous too
    Rest(u8),
}

// we can associate function to enums
impl Action {
    fn describe(&self){
        match self {
            &Action::Movement(ref movement) => {
                println!("I am moving {:?}.", movement);
            },
            &Action::Rotate{d} => println!("I am changing direction by {} degrees.", d),
            &Action::Nothing => println!("I am waiting."),
            &Action::Rest(t) => println!("I am resting for {} turns.", t),
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

    // Rust does not have null type, it uses enum Option instead
    // create variable input of Option enum type - it holds either i32 value or None
    let input: Option<i32> = Option::Some(10);
    // let input: Option<i32> = Option::None;

    // if we match enum (Option in this case), we have to cover all cases
    // Rust ensures null safety in compile time - we have to specify when we want
    // to use null (None) value by Option enum, and all matches that work with Option
    // enum have to cover all cases
    match input {
        Some(i) => println!("Variable is initialized, value: {}", i),
        None => println!("Variable is None."),
    }

    let up = Move::Up;
    is_movement_up(up);

    let none: Option<u8> = None;
    let three: Option<u8> = Some(3);
    let two: u8 = 2;

    // How to add two numbers, from which one can be null?
    // Rust does not allow to add different types
    let res = two + three;

    // so we use match in function add_option_enum(), remember that we must cover all possible
    // cases of enum
    add_option_enum(two,none);
    add_option_enum(two,three);
}

// Rust provides _ placeholder, that covers all not explicitely covered cases
// in this function, _ placeholder covers Left, Right and Down value
fn is_movement_up(movement: Move) {
    match movement {
        Move::Up => println!("Moving up!"),
        _ => println!("Not moving up, change direction"),
    }
}

fn add_option_enum(a1: u8, a2: Option<u8>){
    match a2 {
        Some(value) => println!("{} + {} = {}", a1, value, (a1 + value)),
        None => println!("Cannot add None to u8 number."),
    }
}
