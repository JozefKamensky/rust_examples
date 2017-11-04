fn main() {
    // rust basics    
    
    // immutable variables
    {
        // variable x is by default declared as immutable
        let x = 6;
        println!("Value of x is: {}", x);
        
        // compiler found an error - re-assignment of immutable variable
        x = 7;
        println!("Value of x is now: {}", x);
    }

    // mutable variables
    /*{
        // variable x is declared as mutable by keyword mut
        let mut x = 6;
        println!("Value of x is: {}", x);
        
        // error free code - re-assignment of mutable variable is ok
        x = 7;
        println!("Value of x is now: {}", x);
    }*/
    
    // constants
    {
        // constant have to be type annotated
        // const PI = 3.14;       
        // println!("Value of constant PI is: {}", PI);

        // constant cannot be re-assigned
        // const PI: f32 = 3.14;       
        // println!("Value of constant PI is: {}", PI);
        // error
        // PI = 3.1415;
        // println!("More accurate value of constant PI is: {}", PI);

        // difference between constant and immutable variable
        // const cannot be marked as mutable
        // const mut: f32 PI = 3.14;
        // println!("Value of constant PI is: {}", PI);
    }

    //shadowing
    /*{
        // immutable variable
        let x = 10;
        println!("Value of x is: {}", x);

        // error - re-assignment of immutable variable
        // x = x + 2;

        // we use shadowing to change value of immutable variable
        let x = x + 2;
        println!("Value of x is: {}", x);

        // by shadowing we can change type of immutable variable too
        let x = "twelve";
        println!("Value of x is: {}", x);
    }*/
}





