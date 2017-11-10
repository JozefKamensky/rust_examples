// exaples of simple structures and concepts associated with them

// annotation #[derive(Debug)] which allow us to print structure to output
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
    diagonal: u32,
}

impl Rectangle{
    // methods associated with Rectangle struct - they all use instance of
    // structure to work with, via self keyword

    // immutable reference - just reading
    fn area(&self) -> u32 {
        self.height * self.width
    }

    // mutable reference - reassig values
    fn scale(&mut self, factor: f64) {
        self.width = (self.width as f64 * factor) as u32;
        self.height = (self.height as f64 * factor) as u32;
        self.diagonal = (self.diagonal as f64 * factor) as u32;
    }

    // associated functions - they dont work with instance of structure
    // we can make one to create square easily
    fn square(edge: u32) -> Rectangle {
        Rectangle { width: edge, height: edge, diagonal: ((2*edge*edge) as f64).sqrt() as u32 }
    }
}

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
    color: String,
}

// tuple struct - structure without named fields
struct RgbColor(u8, u8, u8);

fn main() {
    // create instance of structure
    let p1 = Point {
        x: 0,
        y: 0,
        color: String::from("Red"),
    };

    // we can use ..p1 (struct update syntax) to copy all fields not explicitly set from instance p1
    let p2 = Point {
        x: 2,
        color: String::from("Green"),
        ..p1
    };

     // be careful with collections - struct update syntax will move values 
     // it will cause error when we want to print variable p1, because it was moved,
     // not copied
    /*let p3 = Point {
        x: 3,
        ..p1
    };*/

    // we can clone collections manually
    let p4 = Point {
        x: 3,
        color: p1.color.clone(),
        ..p1
    };

    // print strucutres to console output - useful for debug
    println!("p1: {:?}\np2: {:?}\np4: {:?}", p1, p2, p4);

    // create instance of Rectangle via function
    let mut rect1 = build_rectangle(40,30);
    println!("{:#?}", rect1);

    let green = RgbColor(0,255,0);
    println!("---green color---\nR: {}\nG: {}\nB: {}", green.0, green.1, green.2);

    // call method of structure
    println!("Area of rectangle {:?} is: {}", rect1, rect1.area());
    rect1.scale(0.5);
    println!("Rectangle scaled by 0.5: {:?}", rect1);

    let square = Rectangle::square(5);
    println!("{:?}", square);
}

fn build_rectangle(width: u32, height: u32) -> Rectangle {
    let d2: f64 = (width * width + height * height) as f64;
    let r = Rectangle {
        width,
        height,
        diagonal: d2.sqrt() as u32,
    };

    r
}
