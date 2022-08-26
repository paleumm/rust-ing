fn main() {
    {
        let _s = String::new();

        let data = "initial contents";
        let s = data.to_string();
        println!("{}", s);

        // or
        let s = "initial contents".to_string();
        println!("{}", s);

        // or
        let s = String::from("initial contents");
        println!("{}", s);
    }

    {
        let mut s = String::from("foo");
        s.push_str("bar");

        println!("{}", s);

        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push_str(s2);
        println!("s2 is {}", s2);

        let mut s = String::from("lo");
        s.push('l');
        println!("lol {}", s);
    }

    {
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s1 = s1 + &s2;

        println!("{}", &s1);
    }

    // complicated
    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = s1 + "-" + &s2 + "-" + &s3;
        println!("{}", s);
    }

    // use format!
    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = format!("{}-{}-{}", s1, s2, s3);
        println!("{}", s);
    }

    {
        let s1 = String::from("Hello");
        // error
        // let h = s1[0];
    }

    {
        let hello = "Здравствуйте";

        let s = &hello[0..4];

        // panick
        // let s = &hello[0..1];
    }

    {
        for c in "Зд".chars() {
            println!("{}", c);
        }

        for b in "Зд".bytes() {
            println!("{}", b);
        }
    }
}
