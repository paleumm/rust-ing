fn main() {
    // dangling
    {
        // let r;

        {
            let x = 5;
            // r = &x;
        }

        // println!("r: {}", r);
    }
    // ok
    {
        let x = 5;
        let r = &x;
        println!("{}", r);
    }

    {
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }

        let string1 = String::from("abcd");
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        println!("{}", result);
    }

    {
        struct ImportantExcerpt<'a> {
            part: &'a str,
        }

        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("could not find a '.'");
        let i = ImportantExcerpt {
            part: first_sentence,
        };
    }
}
