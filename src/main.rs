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
    let mut num= String::new();
    io::stdin().read_line(&mut num).err();
   
    let num: i64 = num.trim().parse().expect("Please enter a valid number");
    let sum=getsum(num);
    println!("{}",sum);

 
}