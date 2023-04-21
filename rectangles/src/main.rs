#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }
}

fn main() {
    let width1 = 30;
    let height1 = 50;
    // let area = area(width1, height1);

    // let rect = (width1, height1);
    // let area = area(rect);

    let rect1 = Rectangle {
        width: width1,
        height: height1
    };

    // println!("rect1 is {:#?}", rect1);
    // dbg!(&rect1);

    // let area = area(&rect1);

    // println!("The area of the ractangle is {area} square pixels.");

    println!("The area of the ractangle is {} square pixels.", rect1.area());

    let sq = Rectangle::square(3);
}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
