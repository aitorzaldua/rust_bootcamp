//1. Create an Enum `Gender` that defines 2 values `Male` and `Female`.
//2. extend the `Person` struct from Excercise 2 with a new enum variable of type `Gender`, and add it to the constructor, so that you can initialize a Person with:
//Person::new("Frankie", "Miller", 30, Gender::Male)`
//3. implement a function `greet` that prints `Hello Mr. Miller` or `Hello Mrs. Miller` , depending on the value of the enum. Be sure to use the `match` expression to distinguish between both cases.

fn main() {

    enum Gender {
        Male,
        Female,
    }

    struct Person {
        firstname: String,
        lastname: String,
        age: u32,
        gender: Gender,
    }


    let mut person_01 = Person {
        firstname: String::from("Frankie"),
        lastname: String::from("Miller"),
        age: 30,
        gender: Gender::Male,
    };

    let mut person_02 = Person {
        firstname: String::from("Olivia"),
        lastname: String::from("Miller"),
        age: 30,
        gender: Gender::Female,
    };

    impl Person {

        fn greet(&mut self) {
            let mr_mrs = match &self.gender {
                Gender::Male => "Mr.",
                Gender::Female => "Mrs.",
            };

            println! ("Hello {} {}", mr_mrs, self.lastname);

        }

    }

    person_01.greet();
    person_02.greet();


}
