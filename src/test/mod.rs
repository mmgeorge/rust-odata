
#[cfg(test)]
mod test {
    use serde_json;

    use edm;
    use entity::{Entity, EntitySet, Property, EntityDescr, EntitySetDescr};
    use model::{Model, ModelBuilder};
    use service::{Service, ServiceBuilder, Res};


    defEntity!(Dog(keys => id, name) {
        id: Int64,
        name: String,
        age: Int16
    });

    defEntitySet!(Dogs, Dog);

    

    #[test]
    fn runme () {

        // Fake database
        lazy_static! {
            static ref MYDOGS: Vec<Dog> = {
                let mut id = -1;
                let mut id = || { id +=1; id };
                
                let danny_dog = Dog::new(id(), String::from("danny"), 4);
                let jimmy_dog = Dog::new(id(), String::from("jimbo"), 6);
                let little_pup = Dog::new(id(), String::from("lil"), 1);

                vec![danny_dog, jimmy_dog, little_pup]
            };
        }
        
        
        impl EntitySet for Dogs {
            fn read_list(&self) -> Res
            {
                Res::Succ(Some(json!(*MYDOGS)))
            }
        }

        
        let m: Model = ModelBuilder::new("dog_cafe.svc")
            .add(Dogs::declare())
            .build();

        let s: Service = ServiceBuilder::new("gateway")
            .add(m)
            .build();

        // Starts server -- this isn't really a unit test.
        // --Should now be able to make a GET to <root>/gateway/dog_cafe.svc/Dogs
        // --Everything else should print error
        s.start();
        //        println!("{}", serde_json::to_string_pretty(m.get_metadata()).unwrap());
        
        unimplemented!();
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
