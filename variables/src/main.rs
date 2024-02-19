fn main(){
    let mut num1 = 5;
    println!("number is {num1}");
    num1 = 7;
    println!("The number is {num1}");
    const THREE_HOURS_IN_SECONDS:u32 = 60 * 60 * 3;
    println!("{THREE_HOURS_IN_SECONDS}");
    

   // -------------------Shadowing in Rust------------------------ //

    let x = 5;
    let x = x + 1;
    {
        let x = x + 2;
        println!("The value of x inside the wrap is {x}");
    }
    println!("The value of x is {x}");
}
