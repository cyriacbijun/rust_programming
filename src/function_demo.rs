
use std::cmp::Ordering;

pub fn print_array(){
    println!("Simply to show function usage");
    let array : [i32;5] = [12,24,36,48,60];
    let mut index = 0;
    let limit = 5;
    loop{
        match index.cmp(&limit) {
            Ordering::Less => {
                println!("{}",array[index]);
                index = index + 1;
            }
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => break,
        }
    }
}