mod utilities;
mod work;

use work::Department;
use work::Employee;
use work::EmployeeDirectory;

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

    let xan = Employee::new("Xan".to_string(), "Essex".to_string(), Department::Sales);

    println!("{} started on {}", xan.name(), xan.start_date());

    dir.add_employee(sally);
    dir.add_employee(bill);
    dir.add_employee(steve);
    dir.add_employee(xan);

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
