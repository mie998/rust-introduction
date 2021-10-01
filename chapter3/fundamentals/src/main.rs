struct Person {
    name: String,
    age: u32, 
}

enum Color {
    Red,
    Blue,
    Yellow
}

fn main() {
    let s1 = String::from("Hello, world!");
    let s2 = "hello, world!";
    let s3: &str = &s1;

    let mut t = (1, '2');
    t.0 = 3;
    t.1 = 'x';

    let mut a = [0, 1, 2];
    let b = [0; 3];
    a[1] = b[1];
    println!("{:?}", &a[0..3]);

    let p = Person {
        name: String::from("unko"),
        age: 36,
    };

    let red = Color::Red;

    let res1: Result<i32, String> = Ok(200);
    let res2: Result<i32, String> = Err(String::from("error happend!!"));
    match res1 {
        Ok(code) => println!("code: {}", code),
        Err(msg) => println!("error: {}", msg)
    }
    println!("unwrap! {}", res2.unwrap_or(0));
}
