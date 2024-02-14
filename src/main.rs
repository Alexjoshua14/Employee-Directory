mod utilities;
mod work;

use std::io;
use std::io::Write;

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
                if ask_to_continue("Add another employee to the directory?") {
                    continue;
                } else {
                    break;
                }
            }
            None => {
                println!("Failed to create employee..");
                if ask_to_continue("Add another employee to the directory?") {
                    continue;
                } else {
                    break;
                }
            }
        }
    }

    dir
}

// Asks user for input to create new employee
fn get_employee_info() -> Option<Employee> {
    println!("Creating new employee..");

    print!("Employee's first name: ");
    io::stdout().flush().unwrap();
    let first_name = get_input("No first name provided..");

    print!("Employee's last name:  ");
    io::stdout().flush().unwrap();
    let last_name = get_input("No last name provided..");

    print!("Employee's department: ");
    io::stdout().flush().unwrap();
    let department = get_input("No department provided..");

    // TODO: Should ensure that the department is valid
    let department = Department::new(&department)?;

    // TODO: Should ensure the employee is not already in the directory

    Some(Employee::new(first_name, last_name, department))
}

// Gets user input and trims whitespace
fn get_input(exception: &str) -> String {
    let mut buf = String::new();

    io::stdin().read_line(&mut buf).expect(&exception);

    buf.trim().to_string()
}

fn ask_to_continue(prompt: &str) -> bool {
    print!("{} [y/n]: ", prompt);
    io::stdout().flush().unwrap();
    match get_input("Please enter y or n").as_str() {
        "y" => true,
        "yes" => true,
        "n" => false,
        _ => false,
    }
}
