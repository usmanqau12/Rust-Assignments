mod lib;
fn main() {
    println!("Hello, we are inisde of main function ");
    // crate::lib::Q2::SQ2::Question_2(); idiomatic path 
    lib::Q2::SQ2::Question_2();// relative path
    println!("Hello, we are exiting from main funciton after using libraries ");
}
