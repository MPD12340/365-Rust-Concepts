fn main() {
    let mut numbers: Vec<i32> = Vec::new();
    let names = vec!["Alice", "Bob", "Charlie"];
    numbers.push(1);
    println!("The numbers are {:?}", numbers);

    for name in &names{
        println!("Hello, {}!", name);
    }

    let mut other_numbers = vec![1,2,3];
    other_numbers[1] = 10;
    println!("The second number is {}", other_numbers[1]);

    println!("The first name is {}", names[0]);
}
