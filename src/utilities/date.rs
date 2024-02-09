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
        Self {
            day: 8,
            month: 2,
            year: 2024,
        }
    }

    pub fn get_date(&self) -> String {
        format!("{}/{}/{}", self.day, self.month, self.year)
    }
}
