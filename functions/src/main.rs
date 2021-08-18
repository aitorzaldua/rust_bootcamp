use std::cmp;  //Exercise B: max of 2
use std::collections::HashMap; //Exercise F: Chunk array


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
    let input_d2 = String::from("Hi, how are you doing?");

    let result_d1 = str::replace(&input_d1, "H", "");
    let result_d2 = str::replace(&input_d2, "i", "");

    println!("EXERCISE D: Remove a given character from a string.");

    println!("Let´s remove H from {} -> {}", input_d1, result_d1);
    println!("Let´s remove i from {} -> {}", input_d2, result_d2);

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


let input_f1 = vec![1,2,3,4,5,1,2,3,4,5,1,2,3];
let input_f2 = vec![1,2,3,4,5,1,2,3,4,5,1,2,3];

fn slice_array (input: &Vec<i32>, chunk_number: usize) -> Vec<&[i32]>{

    let mut vec_result = vec![];
    let mut count_1 = input.len();
    let mut count_2 = 0;

    loop {

        if count_1 >= chunk_number {

            let slice_1 = &input[count_2..chunk_number + count_2];
            vec_result.push(slice_1);

            count_1 = count_1 - chunk_number;
            count_2 = count_2 + chunk_number;

        }
        else {
            let slice_1 = &input[count_2..];
            vec_result.push(slice_1);

            break
        }

    }

   vec_result

}

let result_f1 = slice_array(&input_f1, 5);
let result_f2 = slice_array(&input_f2, 3);

println!("EXERCISE F: Chunk an array into an array of arrays.");

println!("Chunk array with {:?} and divisor 5 =>{:?}", input_f1, result_f1);
println!("Chunk array with {:?} and divisor 3 =>{:?}", input_f2, result_f2);

println! ("********************************");

// EXERCISE G: Count occurencies all distinct values of an array and return as dictionary.
// TESTS:
// createDict(["a","b","a","a","c","c","a"])
//         => {"a":4, "b":1, "c":2}


    let input_g1 = vec!["a","b","a","a","c","c","a"];

    let mut result_g1: HashMap<String, usize> = HashMap::new();

    for &element in &input_g1 {
        let count = input_g1.iter().filter(|&&n| n == element).count();
        result_g1.insert(element.to_string(), count);
    }

    println!("EXERCISE G: Count occurencies all distinct values of an array");

    println!("Counting occurencies in array: {:?} => {:?}", input_g1, result_g1);

    println! ("********************************");


// EXERCISE H: Multiply each element of the array with x
// multiplyByX([1,2,3,4],10); => [10,20,30,40]
// multiplyByX([1,12,33,9],1.5); => [1.5,18,49.5,13.5]

    let input_h1 = vec![1,2,3,4];
    let input_h2 = vec![1,12,33,9];

    let ih1_to_float: Vec<f32> = input_h1.iter().map(|x| *x as f32).collect();
    let ih2_to_float: Vec<f32> = input_h2.iter().map(|x| *x as f32).collect();



    fn multiply_by_x (vec: Vec<f32>, multi: f32) -> Vec<f32>{

        vec.iter().map(|x| x * multi).collect()

    }

    let result_h1 = multiply_by_x(ih1_to_float, 10 as f32);
    let result_h2 = multiply_by_x(ih2_to_float, 1.5);

    println!("EXERCISE H: Multiply each element of the array with x.");

    println!("{:?} * 10 = {:?}", input_h1, result_h1);
    println!("{:?} * 1.5 = {:?}", input_h2, result_h2);

    println! ("********************************");


// exercise I: flatten an array of an array.
// flatArr([1,2,3],[4,5,6]) // => [1,2,3,4,5,6]

    let input_i =  vec!([1,2,3],[4,5,6]);
    let mut result_i = vec![];

    for i in input_i {

        for x in i {

            result_i.push(x);
        }
    }

    println!("EXERCISE I: flatten an array of an array.");

    println!("Let´s flatten this array: ([1,2,3],[4,5,6]) => {:?}", result_i);

    println! ("************* THE END *******************");


}

