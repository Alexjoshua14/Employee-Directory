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
