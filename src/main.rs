use std::{collections::HashMap, fmt::Debug};

#[derive(Debug)]
pub enum Department {
    Engineering,
    Sales,
    Branding,
}

impl Department {
    pub fn to_string(&self) -> String {
        match self {
            Department::Engineering => "Engineering".to_string(),
            Department::Sales => "Sales".to_string(),
            Department::Branding => "Branding".to_string(),
        }
    }
}

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

pub struct EmployeeDirectory {
    directory: HashMap<String, Vec<Employee>>,
}

impl EmployeeDirectory {
    pub fn new() -> Self {
        Self {
            directory: HashMap::new(),
        }
    }

    pub fn employee_list(&self) -> String {
        let mut list = String::new();

        for (department, department_list) in &self.directory {
            list.push_str(&department.to_string());
            list.push_str(": ");
            for e in department_list {
                list.push_str(&e.name());
                list.push_str(", ");
            }
            list.push_str("\n")
        }

        list
    }

    pub fn add_employee(&mut self, e: Employee) {
        let department_list = self
            .directory
            .entry(e.department().clone())
            .or_insert(Vec::new());

        department_list.push(e);
    }
}

fn main() {
    println!("Hello, world!");

    let directory = build_directory();

    println!("{}", directory.employee_list());
}

fn build_directory() -> EmployeeDirectory {
    println!("Beginning to build employee directory..");
    let mut dir = EmployeeDirectory::new();

    let sally = Employee::new(
        "Sally".to_string(),
        "Trestle".to_string(),
        Department::Branding,
    );

    let bill = Employee::new(
        "Bill".to_string(),
        "Orion".to_string(),
        Department::Branding,
    );

    let steve = Employee::new(
        "Steve".to_string(),
        "Jenkins".to_string(),
        Department::Engineering,
    );

    dir.add_employee(sally);
    dir.add_employee(bill);
    dir.add_employee(steve);

    loop {
        match get_employee_info() {
            Some(e) => {
                // Add employee to directory
                println!("Adding {} to the employee directory: \n{:#?}", e.name(), e);
                dir.add_employee(e);
            }
            None => {
                println!("Directory building completing..");
                break;
            }
        }
    }

    dir
}

// Asks user for input to create new employee
fn get_employee_info() -> Option<Employee> {
    // Let user know they can start creating a new employee
    // Grab first name, last name, department
    // Create employee and return to caller
    // Optionally return none if user decided not to create an employee

    let mut _fn = String::new();
    let mut _ln = String::new();
    let mut _d = String::new();

    // Cast department to Department enum
    // Should probably convert to all one case for better matching

    return None;
}
