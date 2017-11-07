fn main() {
    // ownership

    // out of scope error - using variable after it was droped
    /*{
        // variable s is initialized in scope ending on line 9
        let s = String::from("Hello world!");
        println!("variable s is in scope: {}", s);
    } // variable s is out of scope now - we can use it no more

    //this code will print error
    println!("variable s is out of  scope: {}", s);*/


    // copying of variables
    {
        /*// copying of primitive data types is easy
        let x = 5;
        let y = x;
        println!("X: {}\nY: {}", x, y);*/

        /*// problems come with variables in heap, like String
        let s1 = String::from("hello");
        let s2 = s1;
        // value was not copied, but moved
        // so variable s1 is not longer valid to use
        println!("s1: {}\ns2: {}", s1, s2);*/

        // to create deep copy of String, we need to call clone() function
        let s1 = String::from("hello");
        let s2 = s1.clone();
        // value was copied, so variable s1 is still valid
        println!("s1: {}\ns2: {}", s1, s2);

        //see Copy trait
    }

    // ownership and functions

    // passing variable to function is equal to assigning value to variable -
    // variable is is either moved or copied
    {
        let hello = String::from("hello");
        // String has no Copy trait, so after calling function with hello as parameter
        // s is moved to function and its scope, after function end hello is droped 
        take_ownership(hello);
        println!("Variable s is out of scope: {}", hello);
    }
}

fn take_ownership(s: String){
    println!("Taking ownership of variable, its value was: {}", s);
}
