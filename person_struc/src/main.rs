use person_struc::now;
use person_struc::mod_person;


fn main() {


    let mut new = mod_person::Person {
        firstname: String::from("Frankie"),
        lastname: String::from("Miller"),
        age: 25,
    };

    //Normal instance print
    println!("{} {} is {} years old", new.firstname, new.lastname, new.age);

    new.happy_birthday();

    //The print of that line depends on the Display imp
    println!("{} after his birthday", new.to_string());

    println!("Unix time is {}", now());

    //now();


}
