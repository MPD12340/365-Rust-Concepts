// fn main() {
//     println!("Hello, world!");
//     another_function(5, 'h');
// }

// fn another_function(x: u32, unit_label: char){
//     println!("I am doing it {x} times and the alphabet is {unit_label}");
// }

// Statements are instructions that perform some action and do not return a value.
// Expressions evaluate to a resultant value. Let’s look at some examples.

fn main() {
    let y = plus_one(5);
    println!("The value is {}", y);

    let m;
    let z = {
        m = 5;
        m + 1 
    };

    println!("The value of z is {:#?}", z);
}

fn plus_one(x: i32) -> i32 {
    x + 1 
}

// fn main() {
//     let y = {
//         let x = 3;
//         x + 1
//     };

//     println!("The value of y is: {y}");
// }
