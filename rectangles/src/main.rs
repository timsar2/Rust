fn main() {
    
    let rect_imp_1 = Rectangle { width: 10, height: 20 };
    
    println!(
       "The area of rectangles from method is {} square pixels",
        rect_imp_1.area()
    );

    let rect2 = Rectangle { width: 40, height: 60 };

    println!(
        "The area of rectangles is {} square pixels",
        area_struct_tup(&rect2)
    );

    println!(
        "The area of rectangles is {:#?} square pixels",
        rect2
    );

    let rect1 = (30, 50);

    println!(
        "The area of rectangles is {} square pixels",
        area_with_tup(rect1)
    );
    
    let width1 = 30;
    let height1 = 30;

    println!(
        "The area of rectangles is {} square pixels",
        area(width1, height1)
    );
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn area_struct_tup(rectangle: &Rectangle) -> u32{
    rectangle.width * rectangle.height
}

fn area_with_tup(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
