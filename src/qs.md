# Way to give input into the rust .
In Rust, a buffer typically refers to a contiguous region of memory used for temporary storage or for holding data being processed. Buffers are commonly used in various scenarios such as file I/O, network communication, and manipulation of data.
`read_line` is bufreader which typically is a type of reader that has an internal buffer ,allowing it to perform extraa ways of reaading. It will read line by line 
```Rust
use std::io;
fn main(){
    println!("Enter the new name ");
    let mut name =String::new();
    io::stdin ().read_line(&mut name).err();
    println!("The new name is {}",name );
}
```

# Q1 .Promt Olivia to input an integer n,calculate and display the sum of last two digit.

# Q2 Two friends met after a long time and had a casual conversation.
 -Create a  program to simulate this conversation using multiple printf statements by correctly using new lines and required spacing.
 -The conversation is as follows: Hi Adam, how are you? Hi Eve, I am fine. How are you? I am fine. How's life going on
 -Input format:
 -Output format:
 -The output displays the conversation using proper spacings and new lines as shown in the problem statement.
