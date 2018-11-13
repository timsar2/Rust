fn main() {
    refrence();

    let x = 5;
    makes_copy(x);
    
    let s1 = String::from("hello");
    takes_ownership(s1);
    println!("{}", x);
    
    let s2 = String::from("re take ownership");
    let (s3, len) = calculate_lenght(s2);
    println!("{}, sie:{}", s3, len);
}

fn refrence() {
    let s1 = String::from("hello");    
    let len = refrence_calc_length(&s1);
    println!("refrence: {}", len);
}
fn refrence_calc_length(s: &String) -> usize {
    s.len()
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn calculate_lenght(s: String) -> (String, usize) {
    let lenght = s.len();
    (s, lenght)
}