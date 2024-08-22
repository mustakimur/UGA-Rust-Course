use julian::{Calendar, Month};

fn between_dates() {
    let cal = Calendar::JULIAN;
    let date = cal.at_ymd(2024, Month::August, 18);

    let today = match date {
        Ok(d) => d,
        Err(error) => {
            println!("Invalid date: {}", error);
            return;
        }
    };

    let date = cal.at_ymd(2023, Month::May, 11);

    let last_year = match date {
        Ok(d) => d,
        Err(error) => {
            println!("Invalid date: {}", error);
            return;
        }
    };

    println!("Today is: {}", today);
    println!("Last year was: {}", last_year);

    println!(
        "Days between: {}",
        today.julian_day_number() - last_year.julian_day_number()
    );
}

fn main() {
    between_dates();
}
