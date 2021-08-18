use std::fmt;
use std::io;

struct Person {
    firstname: String,
    lastname: String,
    age: u32,
}


//function to_string()
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Your name is: {} {}. Your age is: {}", self.firstname, self.lastname, self.age)
    }
}

impl Person {
    fn print (&self) {
        println!("Your name is: {} {}. Your age is: {}", self.firstname, self.lastname, self.age);
    }

    fn happy_birthday (&mut self) -> u32 {
        self.age += 1;
        self.age

    }

    pub fn set_last_name (&mut self) {
        println!("Please type your name");
        let mut new_firstname = String::new();

        io::stdin()
            .read_line(&mut new_firstname)
            .expect("Failed to read line");

        new_firstname = new_firstname.trim().to_string();

        println!("Please type your surname");

        let mut new_lastname = String::new();

        io::stdin()
            .read_line(&mut new_lastname)
            .expect("Failed to read line");

        new_lastname = new_lastname.trim().to_string();

        self.firstname = new_firstname;
        self.lastname = new_lastname;


    }


}


fn main() {
    let mut new_person = Person {
        firstname: String::from("Frankie"),
        lastname: String::from("Miller"),
        age: 25,
    };

    println!("Validating the print() function:");
    new_person.print();
    println!("Validating the to_string() function:");
    println!("{}", new_person.to_string());
    println!("Validating the happy_birthday() function:");
    new_person.happy_birthday();
    println!("Happy birthday {}! You are {} years old now!", new_person.firstname, new_person.age);

    //Validating user data -> function set_last_name():
    //2.- Requesting the info from the user:
    println!("Now, we are going to check the name and surname of the person");
    println!("You are {} {}. Is that correct?", new_person.firstname, new_person.lastname);
    println!("Please, type YES if it is correct or NO if not:");

    let mut check_data = String::new();

    io::stdin()
        .read_line(&mut check_data)
        .expect("Failed to read line");

    check_data = check_data.trim().to_lowercase();


//3.- Modifying the instance

    if check_data == "no" {
        new_person.set_last_name();

        println!("You are {0} {1}. Thank you Mr. {1}.", new_person.firstname, new_person.lastname);

    }
    else if check_data == "yes" {
        println!("Thank you, Mr. {}", new_person.lastname);
    }
    else {
        println!("I have specifically asked you to write YES or NO. I don't have time for that. Goodbye.")
    }

}
