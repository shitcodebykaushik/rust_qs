use std::io::{self, stdin};
fn main(){
 println!("Enter your name ");
 let mut name = String::new();
 io::stdin().read_line(&mut name).err();
 print!("{}",name); 
  
}