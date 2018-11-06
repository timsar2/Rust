fn main() {
    let y = {
        let x = 5;
        x + 1
    };
    another_function(y)
}

fn another_function( x: i32){
    println!("value x is {}", x);
}