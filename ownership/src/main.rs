fn main() {
    
    struct_define();
    
    slice_string();
    refrence();
    ownership();    
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool
}

fn struct_define() {
    let mut user1 = User {
        username: String::from("Mrg"),
        email: String::from("info.golab@test.com"),
        sign_in_count: 1,
        active: true
    };
    user1.email = String::from("info.golab@gmail.com");
    let mut user2 = build_user("Mrg2", "info2@gmail.com");
    println!("struct_define: {}", user1.email);
}

fn slice_string() {
    let  s1 = String::from("hello world");    
    let word = first_word(&s1); // word will get the value 5
    let second = second_word(&s1); // word will get the value 5
    
    //s1.clear();
    println!("slice: source:{}, first:{}, second:{}",s1, word, second );    
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i+1..];
        }
    }

    &s[..]
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