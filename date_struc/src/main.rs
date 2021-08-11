extern crate chrono;

use chrono::prelude::*;

fn main() {
    let date = Utc::today().format("%Y-%m-%d");

    println! ("Today is {}", date);
}
