fn main() {
    let hello = String::from("hello");

    // error - cannot move borrowed value
    // hello is borrowed by println function and cannot be moved into function
    //println!("Length of string {} is: {}", hello, calc_length(hello));

    // error - hello is moved into function and cannot be used after function ends
    //let l = calc_length(hello);
    //println!("Length of string {} is: {}", hello, l);

    // hello is still usable, because we pass it by reference
    //println!("Length of string {} is: {}", hello, calc_length_ref(&hello));

    // changing borrowed value
    //append_world(&hello);
    //println!("Modified value: {}", hello);

    // changing borrowed value - using mutable reference
    //let mut _hello = String::from("hello");
    //append_world_correct(&mut _hello);
    //println!("Modified value: {}", _hello);

    // mutable references have one serious restriction - only one mutable reference can exist
    // for one piece of data, this approach helps to prevent data races
    //let mut s = String::from("nothing");
    //let s1 = &mut s;
    //let s2 = &mut s;

    // mutable reference cannot co-exist with immutable references
    // reason is simple - users of immutable references do not expect data to change
    //let mut s = String::from("nothing");
    //let s1 = &s;
    //let s2 = &s;
    //let s3 = &mut s;

    // dangling pointer - one try to create it
    let dangle = return_dangling_pointer();
}

fn calc_length(s: String) -> usize{
    s.len()
}

fn calc_length_ref(s: &String) -> usize{
    s.len()
}

/*fn append_world(s: &String){
    s.push_str(" world");
}*/

fn append_world_correct(s: &mut String){
    s.push_str(" world");
}

// trying to return reference to no longer existent variable
fn return_dangling_pointer() -> &String {
    let s = String::from("dangling_pointer");
    &s
}