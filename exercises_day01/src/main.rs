mod reverse;

fn main() {

    //Take a string and return a new string but with the character reversed
    //example: hello rust -> tsur olleh
    //algorism in reverse.rs

    let input_01 = String::from ("hello rust");

    let result_01 = reverse::reverse_string(&input_01);

    println! ("{}", result_01);

    assert_eq!("tsur olleh", result_01);


}
