//Variable are inmutables by default
//Rust is a block-scoped language

pub fn run (){
    
    //mutable and inmutable:
    let name = "Brad";
    let mut age = 37;
    println!("name: {}, age: {}", name, age);
    age = 38;
    println!("name: {}, age: {}", name, age);

    //Constant:
    const ID: i32 = 001;
    println!("ID: {}", ID);

    //multiples vars:
    let (name, age) = ("Brad", 37);
    println!("{} {}", name, age);



}