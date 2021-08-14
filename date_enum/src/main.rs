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
            //Primavera: Equinoccio de marzo 3: día 20 y a veces 19
            //Verano: Solsticio de junio 6: días 20 o 21
            //Otoño: Equinoccio de septiembre 9: día 22 y a veces 23
            //Invierno: Solsticio de diciembre 12: día 21 a veces 22
            let actual_month = self.date_str.month().to_string().parse::<i32>().unwrap();

            let actual_day = self.date_str.day().to_string().parse::<i32>().unwrap();


            let season = match actual_month {
                1 | 2 =>  "Winter",
                3     => {fn match_day (actual_day: i32) -> String {
                    "Winter"
                }},
                4 | 5 => "Spring",
                6     => "",
                7 | 8 => "Summer",
                9     => "",
                10|11 => "Autumn",
                12    => "",
                _ => "panic!",

            };

            season.to_string()

        }
    }

    let mut august = Date {
        date_str: Local::now(),
        month: Month::August,
        season: Season::Summer,


    };


    println! ("{}" , august.get_season());
   

}
