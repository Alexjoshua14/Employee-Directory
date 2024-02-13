use chrono::{prelude::Local, Datelike};

#[derive(Debug)]
pub struct Date {
    day: u8,
    month: u8,
    year: u16,
}

impl Date {
    pub fn new_with_date(day: u8, month: u8, year: u16) -> Self {
        Self { day, month, year }
    }

    pub fn today() -> Self {
        let today = Local::now();
        Self {
            day: today.day() as u8,
            month: today.month() as u8,
            year: today.year() as u16,
        }
    }

    pub fn get_date(&self) -> String {
        format!("{}/{}/{}", self.day, self.month, self.year)
    }
}
