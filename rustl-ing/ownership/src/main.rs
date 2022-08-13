fn main() {
    
    // String datatype

    // let ss = "String Literal"; // cannot be mutated, stack mem

    let mut s = String::from("Hello");

    s.push_str(", world!");

    println!("{}", s);
    
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
}
