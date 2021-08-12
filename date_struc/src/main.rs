use chrono::prelude::*;
use std::time::SystemTime;

fn main() {

    struct Date {
        date_str: DateTime<Local>,
        unix_timestamp: SystemTime,

    }

    impl Date {
        fn get_formatted_date_str (&mut self) {


        }
    }

    let new_date = Date {
        date_str: Local::now(),
        unix_timestamp: SystemTime::now(),
    };

    let date_formated = new_date.date_str.format("%Y-%m-%d");
    println!("{}", date_formated);
    println!("{:?}", new_date.unix_timestamp);

    


}