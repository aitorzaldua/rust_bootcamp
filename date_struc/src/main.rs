use chrono::prelude::*;
use std::time::SystemTime;

fn main() {

    struct Date {
        date_str: DateTime<Local>,
        unix_timestamp: SystemTime,

    }

    impl Date {
        fn get_formatted_date_str (&mut self) -> String {
            let day = self.date_str.day().to_string();
            let month = self.date_str.month();
            let year = self.date_str.year().to_string();

            let month_str = match month {
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
            };


            let mut date_formatted = String::new();
            date_formatted.push_str(&day);
            date_formatted += " ";
            date_formatted.push_str(&month_str);
            date_formatted += " ";
            date_formatted.push_str(&year);

            date_formatted


        }

        fn get_date_str (&mut self) -> String{

            let d = self.date_str.format("%Y-%m-%d");

            d.to_string()

        }
    }


    let mut new = Date {
        date_str: Local::now(),
        unix_timestamp: SystemTime::now(),
    };


    //Create an initializer function (constructor) named new, such that you can create a Date with the following syntax: Date::new("2021-08-25")
    let d = new.date_str.format("%Y-%m-%d");

    println!("{}", d);

    //Implement a function get_formatted_date_str that returns a date string formatted as following: "2. August 2021"

    println!("{}", new.get_formatted_date_str());


    //Implement a function get_date_str that returns the date_str of the date: "2021-08-02":
    println!("{}",new.get_date_str());

    //Implement a function get_unix that returns the unix timestamp of the date: 1627862400
    println!("{:?}", new.unix_timestamp);



}