// fn main() {
// //   let x = 6;
// //   if x > 5 {
// //     println!("The number is greater");
// //   } else {
// //     println!("The number is smaller");
// //   }
// // if x != 0 {
// //     println!("The number is non zero");
// // }
// // if x % 4 == 0 {
// //     println!("The number is divisible by 4");
// // } else if x % 3 == 0 {
// //     println!("The number is divisible by 3");
// // } else {
// //     println!("The number is indivisible");
// // }

// // let condition = true;

// // let num = if condition {5} else {6};
// // println!("The number is {num}");

// //loops in rust 

// loop {
//     println!("Hello");
// }
// }


fn main(){

    // let mut counter = 5;
     
    //  while counter != 0 {
    //     println!("Hello man");
    //     counter = counter -1 ;
    //  }
    //   loop {
    //     counter = counter + 1;
    //     println!("I am at the {counter} position");
    //     if counter >= 10 {
    //         break;
    //     }
    //     println!("The value of counter is {counter}");
    //   }

    //loop through a collection

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < a.len() {
println!("The value is {}", a[index]);
index = index + 1;
    }
    println!("I am out");

    //we can use for loop for more clarity

    for element in a {
        println!("The element is {element}");
    }

    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");
}