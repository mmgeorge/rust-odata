
#[cfg(test)]
mod test {
    use entity::Entity;
    use property::Property;
    use model::Model;
    
    // use entity_set::EntitySet;
    // use edm::Edm;

    defEntity!(Dog {
        key: Int64
    });

    defEntitySet!(Dogs, Dog);

    #[test]
    fn runme () {
        let mut m = Model::new();
        m.add(Dogs::describe());


        //Dogs::describe();
        panic!();
    }
    
    // struct TestProducts { }
    
    // impl EntitySet for TestProducts {

    //     fn declare (&self) -> Entity {
    //         let product = EntityBuilder::new("Product")
    //             .add("name", Edm::Type::String)
    //             .add("id",   Edm::Type::String)
    //             .build();

    //         product
    //     }
        
    //     fn create (&self) -> bool {
    //         println!("Create is triggered");
    //         true
    //     }
    // }
    
    
    // #[test]
    // fn assert_create () {
    //     unimplemented!()
    // }

    // #[test]
    // fn assert_read () {
    //     unimplemented!()
    // }

    // #[test]
    // fn assert_update () {
    //     unimplemented!()
    // }

    // #[test]
    // fn assert_delete () {
    //     unimplemented!()
    // }

    // #[test]
    // fn assert_query () {
    //     unimplemented!()
    // }
}
