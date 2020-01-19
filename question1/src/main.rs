mod Q1
{
    pub mod SQ1
    {
        pub fn Question_1()
        {
            println!("next line is from Q1::SQ1::Question1():");
        }
    }
}
use crate::Q1::SQ1::Question_1;// this is idiomatic path 
use crate::Q1::SQ1; // this is relative path

fn main() {
    println!("Hello, from main function");
    Q1::SQ1::Question_1();// we are calling qustion 1 function form main using idiomati cpath  
    
}
