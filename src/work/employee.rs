use super::Department;
use crate::utilities::Date;

#[derive(Debug)]
pub struct Employee {
    department: Department,
    first_name: String,
    last_name: String,
    start_date: Date,
}

impl Employee {
    pub fn new(first_name: String, last_name: String, department: Department) -> Self {
        Self {
            first_name,
            last_name,
            department,
            start_date: Date::today(),
        }
    }

    pub fn name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    pub fn department(&self) -> String {
        self.department.to_string()
    }

    pub fn start_date(&self) -> String {
        self.start_date.get_date()
    }
}
