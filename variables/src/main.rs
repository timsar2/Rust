fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    
    let g = another_function(number);

    println!("g is {}", g)
}

fn another_function( x: i32) -> i32{
    x + 1
}