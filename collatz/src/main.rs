// TESTS
//collatz(5)  => [5,16,8,4,2,1]
//collatz(12) => [12,6,3,10,5,16,8,4,2,1]

use std::io;

fn main() {

    println!("To test the Collatz Conjeture, please, type a number:");

    let mut input_01 = String::new();

    io::stdin()
        .read_line(&mut input_01)
        .expect("Failed to read line");

    let mut input_01: i32 = input_01.trim().parse().expect("Please type a number!");

    let mut collatz: Vec<i32> = Vec::new();

    println!("The Collatz Conjeture for {} is:", input_01);

    loop{
        if collatz.contains(&input_01) {
            println!("{:?}", collatz);
            break
        }
        else{
            collatz.push(input_01);
            if input_01 % 2 == 0{
                input_01 = input_01 / 2;
            }
            else {
                input_01 = (input_01 * 3) + 1;
            }
        }
    }


}
