
#[cfg(test)]
mod test {
    use serde_json;

    use property::Property;
    use edm::Edm;
    use entity::{Entity, EntityDescr};
    use entity_set::{EntitySet, EntitySetDescr};
    use model::{Model, ModelBuilder};
    use service::{Service, ServiceBuilder};

    
    defEntity!(Dog(keys => id, name) {
        id: Int64,
        name: String,
        age: Int16
    });

    defEntitySet!(Dogs, Dog);

    impl EntitySet for Dogs {
        
    }


    #[test]
    fn runme () {
        // Create oData model
        let m: Model = ModelBuilder::new("")
            .add(Dogs::declare())
            .build();

        let s: Service = ServiceBuilder::new("gateway")
            .add(m)
            .build();
        
        s.start();
        
//        println!("{}", serde_json::to_string_pretty(m.get_metadata()).unwrap());
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
