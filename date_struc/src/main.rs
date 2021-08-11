
use chrono::{Datelike, Utc};


fn main() {

    let now = Utc::now();


    //let (is_common_era, year) = now.year_ce();
    println!(
        "The current UTC date is {}-{:02}-{:02} {:?}",
        now.year(),
        now.month(),
        now.day(),
        now.weekday(),
        //if is_common_era { "CE" } else { "BCE" }
    );

    struct Date {
        date_str: String
        unix_timestamp: 
     
    }



}