use super::Employee;
use std::collections::HashMap;

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
