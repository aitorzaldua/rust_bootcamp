fn main() {
    let mut list:Vec<u32> = vec![24,2,15,12];
    
    let mut len = list.len();

    println!("{}", len);

    loop {

        let mut counter = 0;

        for i in 1..len {

            if list[i - 1] > list[i] {
                list.swap(i - 1, i);
                counter = i;
            }
        }
        if counter == 0 {
            break;
        }
        len = counter;

    }

    println!("{:?}", list);



}
