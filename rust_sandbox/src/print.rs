
pub fn run () {
    //Print to console
    println!("Hello from the print.rs file");

    //Basic fromatting
    println!("this is a word: {} and this is a number: {}", "Hello", 4);

    //Positional Arguments:__rust_force_expr!
    println!("{0} am studying {1} because {0} think {1} is {2}", "I", "Rust", "awsome");

    //Named Arguments:
    println!("{name} likes to play {activity}", name = "Brad", activity = "baseball");

    //placeholder trait:
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10,10,10);

    //Placeholder for debug trait and more like arrays, vectors,...:
    println!("{:?}", (12, true, "hello"));

    //Basic maths:
    println!("10 + 10 = {}", 10 + 10);


}

