extern crate chrono;
use chrono::prelude::*;
//use std::time::{ SystemTime, UNIX_EPOCH };

fn main() {
    let date = Utc::today().format("%Y-%m-%d");

    println! ("Today is {}", date);

    struct Date {
        date_str: DateTime<Utc>,
        //unix_timestamp: u128,

    }

    let new_date = Date {
        date_str: Utc::now(),
        //unix_timestamp: u128,
    };

    println!("{}", new_date.date_str);
}
