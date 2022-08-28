fn main() {
    {
        let num_list = vec![34, 50, 25, 100, 65];
        let result = largest_i32(&num_list);
        println!("largest i32 {}", result);

        let char_list = vec!['y', 'z', 'a', 'b'];
        let result = largest_char(&char_list);
        println!("largest char {}", result);
    }

    {
        struct Point<T> {
            x: T,
            y: T,
        }

        let integer = Point { x: 5, y: 10 };
        let float = Point { x: 1.0, y: 10.0 };
    }

    {
        struct Point<T, U> {
            x: T,
            y: U,
        }

        let mix = Point { x: 10, y: 5.0 };
    }

    {
        struct Point<T> {
            x: T,
            y: T,
        }

        impl<T> Point<T> {
            fn x(&self) -> &T {
                &self.x
            }
        }

        impl Point<f32> {
            fn distance_from_origin(&self) -> f32 {
                (self.x.powi(2) + self.y.powi(2)).sqrt()
            }
        }

        let p = Point { x: 5, y: 10 };
        println!("p.x = {}", p.x());
    }

    {
        struct Point<X1, Y1> {
            x: X1,
            y: Y1,
        }

        impl<X1, Y1> Point<X1, Y1> {
            fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
                Point {
                    x: self.x,
                    y: other.y,
                }
            }
        }

        let p1 = Point { x: 5, y: 10.3 };
        let p2 = Point { x: "Hello", y: 'c' };

        let p3 = p1.mixup(p2);

        println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    }
}

fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for number in list {
        // if number > largest {
        //     largest = number;
        // }
    }

    largest
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for ch in list {
        if ch > largest {
            largest = ch;
        }
    }
    largest
}
