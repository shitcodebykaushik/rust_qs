use std::io;
fn main(){
    println!("Enter the new name ");
    let mut name =String::new();
    io::stdin ().read_line(&mut name).err();
    println!("The new name is {}",name );
}