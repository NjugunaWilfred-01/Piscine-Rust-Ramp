// fn main() {
//     println!("Hello, world!");
// }

use std::io;

pub fn main() {
    let riddle = "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";
    let answer = "The letter e";
    let mut count = 0;

    loop {
        println!("{}",riddle);
        let mut result = String::new();

        io::stdin()
            .read_line(&mut result)
            .expect("The letter e");
        
        count += 1;

        if result.trim() == answer {
            println!("Number of trials: {}", count);
            break;
        }
    }
}
