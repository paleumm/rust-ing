fn main() {
    
    // String datatype
    {
        // let ss = "String Literal"; // cannot be mutated, stack mem
        let mut s = String::from("Hello");

        s.push_str(", world!");

        println!("{}", s);
    }
    
    // move - move s1 to s2, and s1 cannot be access anymore
    {
        let s1 = String::from("hello");
        let _s2 = s1;

        // println!("{}, world!", s1); -> compile error
        // s1 is now move to s2
    }

    // clone
    {
        let s1 = String::from("hello");
        let s2 = s1.clone();
    
        println!("s1 = {}, s2 = {}", s1, s2);
    }

    {
        let s = String::from("hello"); // s comes into scope

        takes_ownership(s);             // s's value moves into the function...
                                                // ... and so is no longer valid here
        let x = 5;

        makes_copy(x);                  // x comes into scope
                                                // x would move into the function,
                                                // but i32 is Copy, so it's okay to still
                                                // use x afterward
    }   // Here, x goes out of scope, then s. But because s's value was moved, nothing
        // special happens.

    {
        let _s1 = gives_ownership();         // gives_ownership moves its return
            // value into s1

        let s2 = String::from("hello");     // s2 comes into scope

        let _s3 = takes_and_gives_back(s2);     // s2 is moved into
                                                                // takes_and_gives_back, which also
                                                                // moves its return value into s3
    } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
    // happens. s1 goes out of scope and is dropped.

    {
        let s1 = String::from("hello");

        let (s1, len) = calculate_length(s1);

        println!("The length of '{}' is {}.", s1, len);

    }
}

fn takes_ownership(string: String) {
    println!("{}", string);
}// Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(integer: i32) {
    println!("{}", integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {                // gives_ownership will move its
                                                // return value into the function
                                                // that calls it

let some_string = String::from("yours"); // some_string comes into scope

    some_string                             // some_string is returned and
                                            // moves out to the calling
                                            // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {   // a_string comes into
                                                        // scope

    a_string  // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}