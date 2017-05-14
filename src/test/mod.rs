

#[cfg(test)]
mod test {
    use entity::{Entity, EntityBuilder};
    use entity_set::EntitySet;
    use edm::Edm;

    struct TestProducts { }
    
    impl EntitySet for TestProducts {

        fn declare (&self) -> Entity {
            let product = EntityBuilder::new("Product")
                .add("name", Edm::Type::String)
                .add("id", Edm::Type::String)
                .build();

            product
        }
        
        fn create (&self) -> bool {
            println!("Create is triggered");
            true
        }
    }
    
    
    #[test]
    fn assert_create () {
        unimplemented!()
    }

    #[test]
    fn assert_read () {
        unimplemented!()
    }

    #[test]
    fn assert_update () {
        unimplemented!()
    }

    #[test]
    fn assert_delete () {
        unimplemented!()
    }

    #[test]
    fn assert_query () {
        unimplemented!()
    }
}
