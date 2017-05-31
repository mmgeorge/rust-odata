
use edm;

/// Internal structure for holding property values for an Entity
pub struct Property {
    name: String,
    ptype: edm::Type,
} 


impl Property {
    pub fn new (name: &str, ptype: edm::Type) -> Property
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
        edm::ty(&self.ptype)
    }

    
    pub fn format (&self) -> &str
    {
        edm::format(&self.ptype)
    }
}
