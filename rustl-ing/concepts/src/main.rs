fn main() {
    {
        // Variables and Mutability
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

    {
        //Data Types
        // Keep in mind that Rust is a statically typed language, which means that it must know the types of all variables at compile time.

        //converting string to number
        // we have to specify the type of `number` because it have to know what type you want to convert it to.
        let number: u32 = "42".parse().expect("Not a Number"); 
    }

}
