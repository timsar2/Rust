fn main() {
    let y = {
        let x = 5;
        x + 1
    };
    
    let g = another_function(y);

    println!("g is {}", g)
}

fn another_function( x: i32) -> i32{
    x + 1
}