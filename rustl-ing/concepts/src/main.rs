use std::io;
fn main() {
    // Variables and Mutability
    {
        {
            // Mutability
            let x = 6; // This is immutable
            let mut y = 6; // this is mutable
            println!("x={x}, y={y}");
            // x = 7; // this will lead to error
            y = 7; // this is OK
    
            println!("x={x}, y={y}");
        }
        // constants
        {
            const MINUTES_IN_HOUR: u32 = 60;
            println!("constant : {MINUTES_IN_HOUR}");
        }
        // shadowing - use same name with different value for immutable variable
        {
            let shadow = 5;
            let shadow = shadow + 1;
    
            {
                let shadow = shadow * 2;
                println!("Inner scope shadow : {shadow}");
            }
    
            println!("shadow : {shadow}");
    
            // The first spaces is string type and the second one is number type.
            let spaces = "   ";
            let spaces = spaces.len();
    
            // this cannot be done by mutable variables
            // let mut spaces = "   ";
            // spaces = spaces.len();
        }
    }

    //Data Types
    {
        // Keep in mind that Rust is a statically typed language, which means that it must know the types of all variables at compile time.

        //converting string to number
        // we have to specify the type of `number` because it have to know what type you want to convert it to.
        let number: u32 = "42".parse().expect("Not a Number"); 

        //scalar type

        // integer types
        /*
            length      signed  unsigned
            8-bit       i8      u8
            16-bit      i16     u16
            32-bit      i32     u32
            64-bit      i64     u64
            128-bit     i128    u128
            arch        isize   usize
        */

        // arch is depend on the architecture of the machine, 64 or 32.

        // Dec -> 98_222, Hex -> 0xff, Octal -> 0o77, Binary -> 0b1111_0000, Byte (u8)-> b'A'

        // Floating point
        // f32 and f64

        let x = 2.0; //f64 -> default
        let y: f32 = 3.0; //f32

        // Numeric Operations are the same with c, c++

        // Boolean Type
        let t = true;
        let f: bool = false;

        // Character Type
        let c = 'Z';
        let ch: char = 'A';
        let heart_eyed_cat = '😻';

        // Compound Type

        // Tuple
        let tup: (i32, f64, u8) = (500, 6.4, 1);
        let tup2 = (100, 2.34, 6);

        let (x, y, z) = tup2;
        println!("The value of y in tuple is {y}");

        // access by index
        let tup_0 = tup.0;
        let tup_1 = tup.1;
        let tup_2 = tup.2;

        // Array
        let a = [1, 2, 3, 4];
        let b: [u8; 4] = [1, 1, 0, 1]; // with size

        let c = [3; 10]; // array with 10 '3'

        let c_0 = c[0]; // like other lang

        // invalid index access
        let mut index = String::new();
        io::stdin().read_line(&mut index).expect("");
        
        let index: usize = index.trim().parse().expect("");

        let element = a[index];

        println!("Array index {index} is {element}");

    }

    // functions
    {
        some_func();
        some_func_with_params(10);

        // let x = (let y = 6); // NOOOOOOOOOOOO
        let y = {
            let x = 3;
            x + 1 // no ; means return
        }; // Yessss

        let k = five();
    }

    // Control Flow
    {
        let num = 4;
        // condition must be `bool` type only, not like c,c++ that can be int.
        if num < 5{
            println!("Haha");
        } else {
            println!("Wow");
        }

        let number = if num < 5 { num + 1 } else { num-1 };


        // Loops

        // infinite loop
        let mut c = 0;
        let result = loop {
            println!("again!");
            c += 1;

            if c >= 10 {
                break c * 2;
            }
        };
        println!("result = {result}");

        // loop label

        let mut count = 0;
        'counting_up: loop {
            println!("count = {count}");
            let mut remaining = 10;

            loop {
                println!("remaining = {remaining}");
                if remaining == 9 {
                    break;
                }
                if count == 2 {
                    break 'counting_up;
                }
                remaining -= 1;
            }
            count += 1;
        }
        println!(" End count = {count}");

        // while loop

        let mut num = 3;

        while num != 0 {
            println!("{num}");

            num -= 1;
        }
        println!("Wowwwww");

        // for loop
        let a = [10, 20, 30, 40, 50];
        
        for element in a{
            println!("element : {element}");
        }

        for number in (1..10).rev() {
            println!("{number}");
        }

    }

}

fn some_func(){
    println!("Do somethings");
}

fn some_func_with_params(x: i32){
    println!("Do somethings {x}");
}

// func with return type
fn five() -> i32{
    5
}