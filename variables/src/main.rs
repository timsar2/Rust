fn main() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);

    let tup = (500,6.4,1);
    let (c,z,heart_eyed_cat) = tup;
    println!("The value of guess is: {}, {}, {}", c,z,heart_eyed_cat);
}