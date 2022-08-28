pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn print_and_return_10(a: i32) -> i32 {
    println!("Value is {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_pass() {
        let val = print_and_return_10(4);
        assert_eq!(10, val);
    }

    #[test]
    fn this_fail() {
        let val = print_and_return_10(4);
        assert_eq!(5, val);
    }

    #[test]
    fn itworks() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("2 + 2 is not equal to 4"))
        }
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
