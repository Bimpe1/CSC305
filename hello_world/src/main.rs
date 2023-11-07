mod greetings;
use greetings::*;
extern crate hello_world_lib;

fn main() {
    let  mut greeting =" Hello there";
    greeting = "Hello there again!";
    println!("{}", greeting);

println!("Hello, world!");
println!("My first greeting is {} then my second is {}, {},{}", 
english::default_greeting(), english::default_greeting2(), 
spanish::default_greeting(),french::default_greeting());
println!("{}", hello_world_lib::greeting_from_lib());
}