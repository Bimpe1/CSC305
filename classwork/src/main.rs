mod how_to_hold_data_for_operations;
use how_to_hold_data_for_operations::derived::user_defined;
use how_to_hold_data_for_operations::primitives::compound;
use how_to_hold_data_for_operations::primitives::scalar;

fn main() {
    println!("Scalar ");
    scalar::run();

    println!("");
    println!("Compound ");
    compound::run();

    println!("");
    println!("User-Defined ");
    user_defined::run();

}
//a function to multiply 2 arrays
// fn multiplier (arr:&[f64]) -> f64{
//      let mut i=0;
//      let mut product=1.0;
//      while i<arr.len(){
//         product = product*arr[i];
// i+=1;
//      }
//      product
    
// }