use std::fmt;

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
}
