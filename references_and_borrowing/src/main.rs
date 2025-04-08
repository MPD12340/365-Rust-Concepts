fn main() {
    let mut name = String::from("Alice");
    print_Length(&mut name);
    println!("Hello I am {}", name);
}


fn print_Length(s : &mut String){
    println!("Length of the string is {}", s.len());
    s.push_str(" is a good name");
}