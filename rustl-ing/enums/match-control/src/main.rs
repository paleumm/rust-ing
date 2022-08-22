fn main() {
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn values_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
    {
        #[derive(Debug)]
        enum UsState {
            Alabama,
            Alaska,
        }

        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter(UsState),
        }

        fn values_in_cents(coin: Coin) -> u8 {
            match coin {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter(state) => {
                    println!("State of this quarter is {:?}", state);
                    25
                }
            }
        }
    }

    {
        fn plus_one(x: Option<i32>) -> Option<i32> {
            match x {
                None => None,
                Some(i) => Some(i + 1),
            }
        }

        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);
    }

    {
        let dice_roll = 9;
        match dice_roll {
            3 => add_hat(),
            7 => remove_hat(),
            other => move_palyer(other),
        }

        fn add_hat() {}
        fn remove_hat() {}
        fn move_palyer(num_spaces: u8) {}
    }

    {
        let dice_roll = 9;
        match dice_roll {
            3 => add_hat(),
            7 => remove_hat(),
            _ => reroll(),
        }

        fn add_hat() {}
        fn remove_hat() {}
        fn reroll() {}
    }

    {
        let dice_roll = 9;
        match dice_roll {
            3 => add_hat(),
            7 => remove_hat(),
            _ => (),
        }

        fn add_hat() {}
        fn remove_hat() {}
    }
}
