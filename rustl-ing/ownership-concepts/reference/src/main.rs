fn main() {
    // reference allow you to refer to some value without taking ownership of it
    {
        let s1 = String::from("hello");

        let len = calculate_length(&s1);

        println!("The length of '{}' is {}.", s1, len);
    }

    // try to change the value that refer to
    {
        let s = String::from("hello");

        change(&s);
    }

    // mut ref
    {
        let mut s = String::from("hello");

        change2(&mut s);
    }

    // cannot have two mutable reference to same value
    {
        let mut _s = String::from("hello");

        // let r1 = &mut s;
        // let r2 = &mut s;

        // println!("{}, {}", r1, r2);
    }
}

// & is reference
fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} /*
Here, s goes out of scope. But because it does not have ownership of what
it refers to, it is not dropped.
*/

fn change(_some_string: &String) {
    // some_string.push_str(", world"); // not work
}

fn change2(some_string: &mut String) {
    some_string.push_str(", world");
}