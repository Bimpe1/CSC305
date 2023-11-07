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
