use std::cmp;  //max of 2

fn main() {

// EXERCISE A: Reverse a String
// Take a string and return a new string but with the character reversed
// TESTS:
// reverseString("hello")   // => "olleh"
// reverseString("123i s8") // => "8s i321"
// reverseString("")        // => ""


    let input_a1 = String::from ("hello");
    let input_a2 = String::from ("123i s8");
    let input_a3 = String::from ("");


    fn reverse_string (input: &String) -> String{

        let reverse: String = input.chars().rev().collect();

        reverse
    }

    let result_a1 = reverse_string(&input_a1);
    let result_a2 = reverse_string(&input_a2);
    let result_a3 = reverse_string(&input_a3);

    println! ("EXERCISE A: Take a string and return a new string but with the character reversed.");

    println! ("'Hello' backward is '{}'", result_a1);
    println! ("'123i s8' backward is '{}'", result_a2);
    println! ("'' backward is '{}'", result_a3);

    println! ("********************************");

// EXERCISE B: Max of 2
// Take 2 numbers and find the larger of both.
// TESTS:
// max(-122387,124)    // => 124
// max(-125,-2)        // => -2
// max(142,2)          // => 142

    fn max_number (x: i32, y: i32) -> i32 {

        let result = cmp::max(x, y);

        result
    }

    let result_b1 = max_number(-122387,124);
    let result_b2 = max_number(-125,-2);
    let result_b3 = max_number(142,2);

    println!("EXERCISE B: Take 2 numbers and find the larger of both.");

    println! ("between -122387 and 124 the greater number is {}", result_b1);
    println! ("between -125 and -2 the greater number is {}", result_b2);
    println! ("between 142 and 2 the greater number is {}", result_b3);

    println! ("********************************");


//EXERCISE C: Sort numbers
//Sort a numeric array by values (smallest to highes)
// TESTS:
// sort([8,12,53,1,2,-6,2])    // => [-6,1,2,2,8,12]
// sort([21,5,0,5,22,504])     // => [0,5,5,21,22,504]

    let input_c1 = [8,12,53,1,2,-6,2];
    let input_c2 = [21,5,0,5,22,504];

    let c1_to_vec = input_c1.to_vec();
    let c2_to_vec = input_c2.to_vec();

    fn smt (mut to_vec: Vec<i32>) -> Vec<i32> {

        to_vec.sort();
        to_vec

    }

    let result_c1 = smt(c1_to_vec);
    let result_c2 = smt(c2_to_vec);

    println!("EXERCISE C: Sort a numeric array by values, smallest to highes.");

    println!("The ordered array for {:?} is: {:?}", input_c1, result_c1);
    println!("The ordered array for {:?} is: {:?}", input_c2, result_c2);

    println! ("********************************");

// EXERCISE D: Remove a given character from a string.
// TESTS:
// removeChar("Hello World", "H")            // => "ello World"
// removeChar("Hi, how are you doing?", "i") // => "H, how are you dong?"

    let input_d1 = String::from("Hello World");
    let remove_char_d1 = 'H';
    let input_d2 = String::from("Hi, how are you doing?");
    let remove_char_d2 = 'i';

    let result_d1 = str::replace(&input_d1, remove_char_d1, "");
    let result_d2 = str::replace(&input_d2, remove_char_d2, "");

    println!("EXERCISE D: Remove a given character from a string.");

    println!("Let´s remove {} from {} -> {}", remove_char_d1, input_d1, result_d1);
    println!("Let´s remove {} from {} -> {}", remove_char_d2, input_d2, result_d2);

    println! ("********************************");


// EXERCISE E: sum all numbers of an array
// TESTS:
// sum([1,2,3,4,5])      // => 15
// sum([829,-12,758,2])  // => 1577

    let input_e1 = [1,2,3,4,5];
    let input_e2 = [829,-12,758,2];

    let result_e1: i32 = input_e1.iter().sum();
    let result_e2: i32 = input_e2.iter().sum();

    println!("EXERCISE E: Sum all numbers of an array.");

    println!("The total sum of this array of numbers: {:?} is {}", input_e1, result_e1);
    println!("The total sum of this array of numbers: {:?} is {}", input_e2, result_e2);

    println! ("********************************");

// EXERCISE F: Chunk an array into an array of arrays
// => what data structur would you use for that in Rust
// => can you even create an Array of Arrays in Rust?
// TESTS:
// chunkArray([1,2,3,4,5,1,2,3,4,5,1,2,3], 5)
//        => [[1,2,3,4,5],[1,2,3,4,5],[1,2,3]]
// chunkArray([1,2,3,4,5,1,2,3,4,5,1,2,3], 3)
//        => [[1,2,3],[4,5,1],[2,3,4],[5,1,2],[3]]


let input_f1 = [1,2,3,4,5,1,2,3,4,5,1,2,3];
let chunk_number_f1 = 5;
let input_f2 = [1,2,3,4,5,1,2,3,4,5,1,2,3];
let chunk_number_f2 = 3;

let input_f1_to_vec = input_f1.to_vec();
let input_f2_to_vec = input_f2.to_vec();


fn slice_array (input: Vec<i32>, chunk_number: usize){

    let slice_1 = &input[..chunk_number];
    let input = &input[chunk_number..];
    println!("{:?} - {:?}", slice_1, input);

}

slice_array(input_f1_to_vec, chunk_number_f1);


}


