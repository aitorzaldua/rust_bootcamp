//Implement an algorithm capable of sorting a list from smallest to largest using the Bubble Sort method.
//Implement a function 'generate_random_vector' that requests a number from the user and generates a random list of numbers with the size of the user's number.
//implement a function print_vector that take in a single argument of type vector and prints a progress bar for each value (0 ⇒ 0%, 30 ⇒ 100%). 
//You can use the following unicode to display a nice progress bar: EMPTY: ░  FILLED: █  

fn main() {
    use std::io;

    //Asking to the user
    println!("Bubble Sort");
    println!("We are going to generate a random list.");
    println!("Please, insert the length of the list");

    let mut input_01 = String::new();

    io::stdin()
        .read_line(&mut input_01)
        .expect("Failed to read line");

    let input_01: i32 = input_01.trim().parse().expect("Please type a number!");

    //Generating the random list of numbers (0<number<30)



    let mut list:Vec<u32> = vec![24,2,15,12];

    let mut len = list.len();

    loop {

        let mut counter = 0;

        for i in 1..len {

            if list[i - 1] > list[i] {
                list.swap(i - 1, i);
                counter = i;
            }
        }
        if counter == 0 {
            break;
        }
        len = counter;

    }

    println!("{:?}", list);



}
