fn main() {
    let s = String::from("Hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{} {}", hello, world);

    let s = String::from("hello");

    let len = s.len();

    let slice = &s[0..len];
    let slice = &s[..];

    println!("{slice}");
}
