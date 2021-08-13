use chrono::prelude::*;
use std::time::SystemTime;

fn main() {

    struct Date {
        date_str: DateTime<Local>,
        unix_timestamp: SystemTime,

    }

    impl Date {
        fn get_formatted_date_str (&mut self) -> String {
            let day = self.date_str.day();
            let month = self.date_str.month();
            let year = self.date_str.year();

            match month {
                1 => "January".to_string(),
                2 => "Febraury".to_string(),
                3 => "March".to_string(),
                4 => "April".to_string(),
                5 => "May".to_string(),
                6 => "June".to_string(),
                7 => "July".to_string(),
                8 => "August".to_string(),
                9 => "September".to_string(),
                10 => "October".to_string(),
                11 => "November".to_string(),
                12 => "December".to_string(),
                _=> "panic! ".to_string(),
            }


        }
    }

    let mut new_date = Date {
        date_str: Local::now(),
        unix_timestamp: SystemTime::now(),
    };

    let date_formated = new_date.date_str.format("%Y-%m-%d");
    println!("{}", date_formated);
    println!("{:?}", new_date.unix_timestamp);

    println!("{}", new_date.get_formatted_date_str());




}