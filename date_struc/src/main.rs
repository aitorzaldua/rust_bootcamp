use chrono::prelude::*;
use std::time::SystemTime;

fn main() {

    struct Date {
        date_str: DateTime<Local>,
        unix_timestamp: SystemTime,

    }

    impl Date {
        fn get_formatted_date_str (&mut self) -> String{
            let day = self.date_str.day();
            let month = self.date_str.month();
            let year = self.date_str.year();


            let month_str = String::new();

            println!("{:?}", month);

            match month {
                1 => month_str.push("January"),
                2 => "Febraury"
                3 => "March"
                4 => "April"
                5 => "May"
                6 => "June"
                7 => "July"
                8 => "August"
                9 => "September"
                10 => "October"
                11 => "November"
                12 => "December"
                _=>println!("nothing"),
            }


        }
    }

    let mut new_date = Date {
        date_str: Local::now(),
        unix_timestamp: SystemTime::now(),
    };

    let date_formated = new_date.date_str.format("%Y-%m-%d");
    println!("{:?}", date_formated);
    println!("{:?}", new_date.unix_timestamp);

    new_date.get_formatted_date_str();


}