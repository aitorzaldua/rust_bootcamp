use person_struc::mod_person;

use std::io;



fn main() {

//1.- Creating the instance:

    let mut new_person = mod_person::Person {
        firstname: String::from("Frankie"),
        lastname: String::from("Miller"),
        age: 25,
    };

    println!("Are you {} {}?", new_person.firstname, new_person.lastname);

//2.- Requesting the info from the user:

    println!("Please, type YES if it is correct or NO if not:");
    let mut check_data = String::new();

    io::stdin()
        .read_line(&mut check_data)
        .expect("Failed to read line");

    check_data = check_data.trim().to_lowercase();


//3.- Modifying the instance

    if check_data == "no" {
        new_person.set_last_name();

        //println!("You are {} {}", new_person.firstname, new_person.lastname);

        println!("{}", new_person.to_string());
    }
    else if check_data == "yes" {
        println!("Thank you, Mr. {}", new_person.lastname);
    }
    else {
        println!("Please, type yes or no")
    }

//4.- Updating the age:

    new_person.happy_birthday();
    println!("Happy birthday {}! You are {} years old now!", new_person.firstname, new_person.age);

}
