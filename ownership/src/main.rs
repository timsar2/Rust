fn main() {
    refrence();
    ownership();    
}

fn refrence() {
    let mut s1 = String::from("hello");    
    let len = refrence_calc_length(&mut s1);
    println!("refrence: {}, {}", s1, len);
    {
        let r1 = &mut s1;
        r1.push_str(" R1");
        println!("first barrow {}", r1)
    }
    let r2 = &mut s1;
    r2.push_str(" then R2");
    println!("second borrow {}", r2);
}
fn refrence_calc_length(s: &mut String) -> usize {
    s.push_str(", world");
    s.len()
}

fn ownership() {
    let x = 5;
    makes_copy(x);
    
    let s1 = String::from("hello");
    takes_ownership(s1);
    println!("{}", x);
    
    let s2 = String::from("re take ownership");
    let (s3, len) = calculate_lenght(s2);
    println!("{}, sie:{}", s3, len);
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