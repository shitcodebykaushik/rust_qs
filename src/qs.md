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
```Rust
use std::io;

pub fn getsum (num :i64) ->  i64 {
    let mut n: i64=num;
    let mut last: i64 = 0;
     for _ in 0..2 {
        last +=n%10;
        n/=10;

     }
     last

}
fn main () {
    println!("Enter the number ");
    let mut num= String::new();
    io::stdin().read_line(&mut num).err();
   
    let num: i64 = num.trim().parse().expect("Please enter a valid number");
    let sum=getsum(num);
    println!(" Your sum of last two digit is {}",sum);

 
}
```

# Q2 Two friends met after a long time and had a casual conversation.
 -Create a  program to simulate this conversation using multiple printf statements by correctly using new lines and required spacing.
 -The conversation is as follows: Hi Adam, how are you? Hi Eve, I am fine. How are you? I am fine. How's life going on
 -Input format:
 -Output format:
 -The output displays the conversation using proper spacings and new lines as shown in the problem statement.
# Q3 for printing of the sum two number from indices 
```Rust
use std::collections::HashMap;
use std::io;
fn two_sum (nums: Vec<i32>,target: i32) -> Vec<i32> {
    let mut num_indices = HashMap::new();

    for (index, &num) in nums.iter().enumerate(){
        let complement = target - num;
        if let Some (&prev_index) = num_indices.get(&complement){
            return  vec![prev_index as i32, index as i32];
        }
        num_indices.insert(num, index);

    }
    Vec::new()

}
fn main (){

    let mut input = String::new();
    io::stdin().read_line(&mut input).err();

    let nums :Vec<i32> =input
    .trim()
    .split_whitespace()
    .map(|s|  s.parse().expect("Please enter the valid integer"))
    .collect();


    
println!("Input the vector {:?}",nums );
let target = 20;
let result=two_sum(nums, target);
println!("Your result is {:?}",result);

}


```