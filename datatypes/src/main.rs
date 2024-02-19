// fn main(){
//     let x = 2.0;
//     let y:f32 = 3.4;
//     println!("The values are {x} and {y} respectively !");

//     //Lets do arithmetic calculations

// let _sum = 5 + 6;
// let _difference = 9.5 - 5.4;
// let _product = 6 * 30;
// let _quotient = 56.7 / 32.2;
// let truncated = -15 / 4;
// let _remainder = 54 % 6;
// let t = true;
// let f:bool = false;
// println!("{truncated}");

// //lets learn about char type in rust 

// let c= 'z';
// let c:char = 'a';
// println!("{c}");

// //let's talk about the compoumd types in rust 

// let tup = ( 3, 4.5, 2);
// let (x, y, z) = tup;
// println!("The value is {}", y);
// let first_val = tup.0;
// let second_val = tup.2;
// let third_val = tup.1;
// println!("{first_val} and {second_val} and {third_val}");

// //Arrays in rust 
//  let arr: [u32; 5] = [2,3,4,5,6];
//  let arr2 = [3;5];
//  let first = arr2[0];
//  let second = arr2[1];
//  println!("{first} and {second} ");
//  let months = ["January", "February", "March", "April", "May", "June", "July",
//               "August", "September", "October", "November", "December"];
// }

//lets get an index of an array from the user 

use std::io;

fn main(){
    let a = [1,2,3,4,5];
    println!("Please enter an array index");
    let mut index = String::new();
    io::stdin()
    .read_line(&mut index)
    .expect("failed to read line");

    let index: usize = index.trim().parse().expect("Index was not a number");
    let element = a[index];
    println!("The number is {element} present as index {index}");

}