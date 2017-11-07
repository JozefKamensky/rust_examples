fn main() {
    // rust basics - control flow
    
    // if-else statement
    /*{    
        let x = 5;
        if x == 5 {
            println!("I know this number, it is five!");    
        } else {
            println!("I don`t know this number, but I will learn it soon!");
        }
    }*/

    // if-elseif statement
    /*{    
        let x = 5;
        if x == 5 {
            println!("I know this number, it is five!");    
        } else if x == 7 {
            println!("I know this number, it is seven!");    
        } else {
            println!("I don`t know this number, but I will learn it soon!");
        }
    }*/

    // if condition must be bool
    /*{
        let x = 1;
        if x {
            println!("Condition was true!");
        }
    }*/

    // using if to initialize variable
    /*{
        let condition = false;
        let x = if condition {
                    5
                } else {
                    6
                };
        println!("Value of x is: {}", x);
    }*/

    // using if to initialize variable - all "arms" must have same return type!
    /*{
        let condition = false;
        let x = if condition {
                    5
                } else {
                    "six"
                };
        println!("Value of x is: {}", x);
    }*/

    //loops
    //for loop - iterate over collection
    /*{
        let array = [1,2,3,4,5];
        let mut sum = 0;
        for number in array.iter() {
            println!("Number to add: {}", number);
            sum = sum + number;
        }
        println!("Sum of numbers in array is: {}", sum);
    }*/

    //for loop - certain number of iterations
    /*{
        for number in (5..10).rev() {
            println!("Number: {}", number);
        }
        println!("End.");
    }*/

    //while loop
    {
        let mut limit = 10;
        while limit > 0 {
            println!("I am working.");
            limit = limit - 1;
        }
        println!("End.");
    }
}



