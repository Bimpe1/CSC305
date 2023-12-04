mod how_to_hold_data_for_operations;
use how_to_hold_data_for_operations::derived::user_defined;
use how_to_hold_data_for_operations::primitives::compound;
use how_to_hold_data_for_operations::primitives::scalar;
use how_to_hold_data_for_operations::derived::functions;
use how_to_hold_data_for_operations::derived::pointers;

fn main() {
    println!("Scalar ");
    scalar::run();

    println!("");
    println!("Compound ");
    compound::run();

    println!("");
    println!("User-Defined ");
    user_defined::run2();


    println!("");
    println!("Functions ");
    functions::run();
    functions::run2();
    functions::run3();
    functions::run4();

    println!("");
    println!("Pointers ");
    pointers::run();
    pointers::run2();

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