//use person_struc::now;
use person_struc::mod_person;

use std::io;



fn main() {

//1.- Creating the instance:

    let mut new_person = mod_person::Person {
        firstname: String::from("Frankie"),
        lastname: String::from("Miller"),
        age: 25,
    };

    println!("You are {} {}", new_person.firstname, new_person.lastname);

//2.- Requesting the info from the user:

    println!("Please, type YES if it is correct or NO if not:");
    let mut check_data = String::new();

    io::stdin()
        .read_line(&mut check_data)
        .expect("Failed to read line");

    check_data = check_data.trim().to_lowercase();

    println!("Your answer is  {:?}", check_data);

//3.- Modifying the instance

    if check_data == "no" {
        new_person.set_last_name();
    }
    else if check_data == "yes" {
        println!("YES!, That´s great, thank you");
    }
    else {
        println!("Please, type yes or no")
    }

}
