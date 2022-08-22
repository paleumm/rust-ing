fn main() {
    {
        let config = Some(3u8);
        match config {
            Some(max) => println!("The maximum is {}", max),
            _ => (),
        }
    }

    {
        let config = Some(5u8);
        if let Some(max) = config {
            println!("The maximum is {}", max);
        }
    }
}
