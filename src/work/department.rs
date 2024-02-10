#[derive(Debug)]
pub enum Department {
    Engineering,
    Sales,
    Branding,
}

impl Department {
    pub fn new(dep: &String) -> Option<Self> {
        match dep.to_lowercase().as_str() {
            "branding" => Some(Self::Branding),
            "engineering" => Some(Self::Engineering),
            "sales" => Some(Self::Sales),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Department::Engineering => "Engineering".to_string(),
            Department::Sales => "Sales".to_string(),
            Department::Branding => "Branding".to_string(),
        }
    }
}
