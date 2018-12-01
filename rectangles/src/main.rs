fn main() {
    let withd1 = 30;
    let height1 = 30;

    println!(
        "The area of rectangles is {} square pixels",
        area(withd1, height1)
    );
}

fn area(withd: u32, height: u32) -> u32 {
    withd * height
}
