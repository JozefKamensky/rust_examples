fn main() {

    // solution works, but function returns variable which is not connected with string
    let mut s = String::from("hello world");
    //println!("First word in \"{}\" ends on {}. character.", s, find_first_word(&s));
    
    // we can clear content of variables s to "" , so index no longer represents right information
    /*let index = find_first_word(&s);
    s.clear();
    println!("First word in \"{}\" ends on {}. character.", s, index);*/

    // this situation can be solved by using slices - slice represent part of collection
    let slice = find_first_word_slice(&s);
    println!("first word in {} is {}", s, slice);
    // clear() will cause error, because of immutable reference via slice - see
    // references_and_borrowing - you cannot create mutable reference while at least one 
    // immutable reference exists 
    //s.clear();

    let literal = "string literal";
    let literal_slice = find_first_word_both(&literal[..]);    
    println!("1st word in literal \"{}\" is: {}", literal, literal_slice);

    // string literals are slices, so we can omit slice syntax in function call
    let literal_slice = find_first_word_both(&literal);
    println!("1st word in literal \"{}\" is: {}", literal, literal_slice);

    let string_slice = find_first_word_both(&s[..]);
    println!("1st word in string \"{}\" is: {}", s, string_slice);
}

// function returning index of last + 1 character in first word
fn find_first_word(string: &String) -> usize {
    let bytes = string.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return i;
        }
    }

    string.len()
}

// function returning first word of String as slice
fn find_first_word_slice(string: &String) -> &str {
    let bytes = string.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &string[0..i];
        }
    }

    &string[..]
}

// function returning first word of both String or string literal as slice - it is more generic
fn find_first_word_both(string: &str) -> &str {
    let bytes = string.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &string[0..i];
        }
    }

    &string[..]
}