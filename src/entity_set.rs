

use entity::Entity;

pub trait EntitySet {

    fn declare(&self) -> Entity;
    
    // CRUD-Q stubs
    fn create (&self) -> bool {
        false
    }
    
    fn read (&self) -> bool {
        false
    }
    
    fn update (&self) -> bool {
        false
    }
    
    fn delete (&self) -> bool {
        false
    }
    
    fn query (&self) -> bool {
        false
    }
    
    
}
