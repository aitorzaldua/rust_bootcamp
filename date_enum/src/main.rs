use chrono::prelude::*;


fn main() {

    enum Month {
        January,
        Febreaury,
        March,
        May,
        June,
        July,
        August,
        September,
        October,
        November,
        December,
    }

    enum Season {
        Winter,
        Summer,
        Spring,
        Autumn,
    }

    struct Date {

        date_str: DateTime<Local>,
        month: Month,
        season: Season,

    }

    impl Date {
        fn get_season(&mut self) -> String {

            let actual_month = self.date_str.month().to_string().parse::<i32>().unwrap();

            let actual_day = self.date_str.day().to_string().parse::<i32>().unwrap();

            let season = match actual_month {
                1 | 2 =>  "Winter",
                3    => {
                    if actual_day < 20 {
                        "Winter"
                    }
                    else {
                        "Spring"
                    }
                },
                4 | 5 => "Spring",
                6     => {
                    if actual_day < 21 {
                        "Spring"
                    }
                    else {
                        "Summer"
                    }
                },
                7 | 8 => "Summer",
                9     => {
                    if actual_day < 22 {
                        "Summer"
                    }
                    else {
                        "Autumn"
                    }
                },
                10|11 => "Autumn",
                12    => {
                    if actual_day < 21 {
                        "Autumn"
                    }
                    else {
                        "Winter"
                    }
                },
                _ => "Error! Invalid Entry",

            };

            season.to_string()

        }
    }

    let mut august = Date {
        date_str: Local::now(),
        month: Month::August,
        season: Season::Summer

    };

    println! ("{}" , august.get_season());

}
