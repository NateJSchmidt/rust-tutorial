#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }

    fn square(size: u32) -> Self {
        Self {
            height: size,
            width: size,
        }
    }
}

fn main() {
    let height = 30;
    let width = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        area(height, width)
    );

    let rectangle = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(rectangle)
    );

    let rect = Rectangle {
        height: 30,
        width: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(&rect)
    );

    println!("rect is {rect:#?}");

    let scale = 2;
    let rect1 = Rectangle {
        height: dbg!(30 * scale),
        width: 50,
    };
    dbg!(&rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect3 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect4 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect2 hold rect3? {}", rect2.can_hold(&rect3));
    println!("Can rect2 hold rect4? {}", rect2.can_hold(&rect4));

    let sq = Rectangle::square(3);
    println!("sq is {:#?}", sq);
    println!(
        "The area of the square is {} square pixels.",
        sq.area()
    );
}

fn area(height: u32, width: u32) -> u32 {
    height * width
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}
