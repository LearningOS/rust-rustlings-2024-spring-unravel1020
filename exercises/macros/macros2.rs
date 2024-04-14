// macros2.rs
//
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a
// hint.


macro_rules! my_macro {  
    () => {  
        println!("Check out my macro!");  
    };  
}  
  
fn main() {  
    my_macro!(); // Now the macro is defined before it's used, so this will work.  
}