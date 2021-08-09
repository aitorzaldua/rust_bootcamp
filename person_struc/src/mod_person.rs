use std::fmt;
use std::io;


pub struct Person {
    pub firstname: String,
    pub lastname: String,
    pub age: u32,
}


impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} is {} years old", self.firstname, self.lastname, self.age)
    }
}


impl Person {
    pub fn happy_birthday (&mut self) -> u32 {
        self.age += 1;
        self.age

    }

    pub fn set_last_name (&mut self) {
        println!("Please insert your name");
        let mut new_firstname = String::new();

        io::stdin()
            .read_line(&mut new_firstname)
            .expect("Failed to read line");

        new_firstname = new_firstname.trim().to_string();

        println!("Please inser your surname");

        let mut new_lastname = String::new();

        io::stdin()
            .read_line(&mut new_lastname)
            .expect("Failed to read line");

        new_lastname = new_lastname.trim().to_string();

        println! ("You are {} {}", new_firstname, new_lastname);


    }
}


