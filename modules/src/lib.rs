pub mod math;
pub mod shapes;
pub mod utils;

use shapes::rectangle::areas;

pub fn test_areas() {
    let result = areas(4.6, 5.6);
    println!("The area of the rectangle is {}", result);
}
