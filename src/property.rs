
use edm::Edm;

/// Structure for holding property values for an Entity
pub struct Property {
    name: String,
    ptype: Edm::Type,
} 


impl Property {
    pub fn new (name: &str, ptype: Edm::Type) -> Property
    {
        Property {
            name: String::from(name),
            ptype: ptype,
        }
    }

    pub fn name (&self) -> &str
    {
        &self.name
    }

    pub fn types (&self) -> Vec<&str>
    {
        Edm::ty(&self.ptype)
    }

    
    pub fn format (&self) -> &str
    {
        Edm::format(&self.ptype)
    }
}
