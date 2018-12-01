struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let rect2 = Rectangle { width: 40, height: 60 };

    println!(
        "The area of rectangles is {} square pixels",
        area_struct_tup(&rect2)
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

fn area_struct_tup(rectangle: &Rectangle) -> u32{
    rectangle.width * rectangle.height
}

fn area_with_tup(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
