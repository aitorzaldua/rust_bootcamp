mod reverse;
mod max_of_2;
mod small_to_high;



fn main() {

// ALGORISM IN reverse.rs
// Take a string and return a new string but with the character reversed
// example: hello rust -> tsur olleh
// TESTS:
// reverseString("hello")   // => "olleh"
// reverseString("123i s8") // => "8s i321"
// reverseString("")        // => ""

    let input_01 = String::from ("hello rust");

    let result_01 = reverse::reverse_string(&input_01);

    println! ("{}", result_01);

    assert_eq!("tsur olleh", result_01);


// ALGORISM in max_of_2.rs
// Take 2 numbers and find the max of them
// TESTS:
// max(-122387,124)    // => 124
// max(-125,-2)        // => -2
// max(142,2)          // => 142

    let result_02 = max_of_2::max_number(-122387,124);

    println! ("The greater number is {}", result_02);

    assert_eq!(124, result_02);


// Sort a numeric array by values (smallest to highes)
// TESTS: 
// sort([8,12,53,1,2,-6,2])    // => [-6,1,2,2,8,12]
// sort([21,5,0,5,22,504])     // => [0,5,5,21,22,504]

let input_03 = [8,12,53,1,2,-6,2];

let to_vec = input_03.to_vec();

let result_03 = small_to_high::smt(to_vec);

println!("{:?}", result_03);

//????assert_eq!([-6,1,2,2,8,12], result_03);?????

// (D) remove a given character from a string
// TESTS:** 
// removeChar("Hello World", "H")            // => "ello World"
// removeChar("Hi, how are you doing?", "i") // => "H, how are you dong?"

let input_04 = String::from("Hi, how are you doing?");

let remove_char = 'i';

let result_04 = str::replace(&input_04, remove_char, "");

println!("{}", result_04);

}



