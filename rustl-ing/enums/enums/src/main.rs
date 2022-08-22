fn main() {
    {
        enum IPAddrStandard {
            v4,
            v6,
        }

        struct IPAddress {
            kind: IPAddrStandard,
            address: String,
        }

        let home = IPAddress {
            kind: IPAddrStandard::v4,
            address: String::from("127.0.0.1"),
        };

        let loopback = IPAddress {
            kind: IPAddrStandard::v6,
            address: String::from("::1"),
        };

        fn route(ip_kind: IPAddrStandard) {}
        let four = IPAddrStandard::v4;
    }
    {
        enum IPAddr {
            V4(String),
            V6(String),
        }

        let home = IPAddr::V4(String::from("127.0.0.1"));

        let loopback = IPAddr::V6(String::from("::1"));
    }

    {
        enum IPAddr {
            V4(u8, u8, u8, u8),
            V6(String),
        }

        let home = IPAddr::V4(127, 0, 0, 1);
        let loopback = IPAddr::V6(String::from("::1"));
    }

    {
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(u8, u8, u8),
        }

        impl Message {
            fn call(&self) {}
        }

        let m = Message::Write(String::from("Hello"));
        m.call();
    }

    {
        /*
        enum Option<T> {
            None,
            Some(T),
        }
        */
        let some_int = Some(5);
        let some_char = Some('e');

        let absent_num: Option<i32> = None;

        {
            let x: i8 = 5;
            let y: Option<i8> = Some(4);
            let y: Option<i8> = None;
            //error
            // let sum = x + y;

            let sum = x + y.unwrap_or(0);
        }
    }
}
