//Implement an algorithm capable of sorting a list from smallest to largest using the Bubble Sort method.
//Implement a function 'generate_random_vector' that requests a number from the user and generates a random list of numbers with the size of the user's number.
//implement a function print_vector that take in a single argument of type vector and prints a progress bar for each value (0 ⇒ 0%, 30 ⇒ 100%). 
//You can use the following unicode to display a nice progress bar: EMPTY: ░  FILLED: █  


use std::io;
use rand::Rng;
use std::{thread, time};



fn main() {


    //Asking to the user
    println!("***********   BUBBLE SORT   *************");
    println!("We are going to generate a random list of numbers between 1 and 30");
    println!("Please, insert the length of the list, between 3 and 30:");

    let mut input_01 = String::new();

    io::stdin()
        .read_line(&mut input_01)
        .expect("Failed to read line");

    let user_input: i32 = input_01.trim().parse().expect("Please type a number!");


    //Generating the random list of numbers (0<number<30)

    let mut list: Vec<i32> = Vec::new();

    for _i in 1..user_input + 1{
        let number = rand::thread_rng().gen_range(1, 31);
        list.push(number);

    }

    println!("The initial list is: {:?}", list);
    let initial_list = list.clone();

// for [24,2,15,12] should print the following to the console:
//████████████████████████░░░░░░
//██░░░░░░░░░░░░░░░░░░░░░░░░░░░░
//███████████████░░░░░░░░░░░░░░░
//████████████░░░░░░░░░░░░░░░░░░


    for data in &list {
        let mut chain = String::new();

        loop{
            for _x in 1..data+1 {
                chain.push_str("█");
            }
            let add_to_chain = 30 - data;
            for _i in 1..add_to_chain+1 {
                chain.push_str("░");
            }
            println!("{}", chain);
            chain.clear();
            break

        }
    }

    let pause_1 = time::Duration::from_millis(4000);
    thread::sleep(pause_1);

    //Bubble Sort with progress bar

    let mut len = list.len();

    loop {

        let mut counter = 0;

        for i in 1..len {

            print!("{esc}c", esc = 27 as char);
            let pause_2 = time::Duration::from_millis(300);

            thread::sleep(pause_2);

            if list[i - 1] > list[i] {
                list.swap(i - 1, i);
                counter = i;
                //here:
                println! ("Sorting in progress, please wait...{:?}", list);
                for data in &list {
                    let mut chain = String::new();

                    loop{
                        for _x in 1..data+1 {
                            chain.push_str("█");
                        }
                        let add_to_chain = 30 - data;
                        for _i in 1..add_to_chain+1 {
                            chain.push_str("░");
                        }
                        println!("{} ", chain);
                        chain.clear();
                        break

                    }
                }
            }
        }
        if counter == 0 {
            break;
        }
        len = counter;

    }

    //final print statement:

    print!("{esc}c", esc = 27 as char);
    println!("The initial, random, list was {:?}", initial_list);
    println!("The sorted list, using Bubble Sort, is: {:?}", list);
    for data in &list {
        let mut chain = String::new();

        loop{
            for _x in 1..data+1 {
                chain.push_str("█");
            }
            let add_to_chain = 30 - data;
            for _i in 1..add_to_chain+1 {
                chain.push_str("░");
            }
            println!("{}", chain);
            chain.clear();
            break

        }
    }


}
