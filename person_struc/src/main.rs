//Utilities for formatting and printing Strings.
use std::fmt;

//Struct (class) of a generic person
struct Person {
    firstname: String,
    lastname: String,
    age: u32,
}

//"Methodsfuntions for the struct

//1.- Implementation (method) to use to_string with a struct
//Check the write! line 
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} is {} years old", self.firstname, self.lastname, self.age)
    }
}

//2.- method for a class
impl Person {
    fn happy_birthday (&mut self) -> u32 {
        self.age += 1;
        self.age

    }
}


fn main() {


    let mut new = Person {
        firstname: String::from("Frankie"),
        lastname: String::from("Miller"),
        age: 25,
    };

    //Normal instance print
    println!("{} {} is {} years old", new.firstname, new.lastname, new.age);

    new.happy_birthday();

    //The print of that line depends on the Display imp
    println!("{}", new.to_string());


}
