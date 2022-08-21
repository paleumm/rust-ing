fn main() {
    // normal
    {
        let width1 = 30;
        let height1 = 50;

        println!(
            "Normal: The area of the rectangle is {} square pixels.",
            area(width1, height1)
        );

        fn area(width: u32, height: u32) -> u32 {
            width * height
        }
    }

    // with tuples
    {
        let rect1 = (30, 50);

        println!("Tuples: Area is {}", area(rect1));

        fn area(dimensions: (u32, u32)) -> u32 {
            dimensions.0 * dimensions.1
        }
    }

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    // with struct
    {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        println!("struct: Area is {}", area(&rect1));

        // borrow rather than take ownership
        fn area(rectangle: &Rectangle) -> u32 {
            rectangle.width * rectangle.height
        }
    }

    {
        let rect1 = Rectangle {
            width: 30,
            height: 30,
        };

        println!("{:#?}", rect1);
        dbg!(&rect1);
    }
}
