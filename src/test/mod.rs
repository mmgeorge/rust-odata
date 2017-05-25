
#[cfg(test)]
mod test {
    use std::sync::Mutex;
    use std::thread;
    use std::time::Duration;

    use serde_json;
    use hyper::Client;
    use hyper::status::StatusCode;

    use edm;
    use entity::{Entity, EntitySet, Property, EntityDescr, EntitySetDescr};
    use model::{Model, ModelBuilder};
    use service::{Service, ServiceBuilder, Res, Error};
    

    
   
    #[test]
    fn monster_test () { 

        // Entity/EntitySet declarations
        defEntity!(Dog(keys => id, name) {
            id: Int64,
            name: String,
            age: Int16
        });

        defEntitySet!(Dogs, Dog);

        
        // Test database
        lazy_static! {
            static ref MYDOGS: Mutex<Vec<Dog>> = {
                let mut id = -1;
                let mut id = || { id +=1; id };
                
                let danny_dog = Dog::new(id(), String::from("danny"), 4);
                let jimmy_dog = Dog::new(id(), String::from("jimbo"), 6);
                let little_pup = Dog::new(id(), String::from("lil"), 1);

                Mutex::new(vec![danny_dog, jimmy_dog, little_pup])
            };
        }
        

        // Implement CRUD
        impl EntitySet for Dogs {

            fn create(&self, value: serde_json::Value) -> Res
            {
                let dog = Dog {
                    id: (*MYDOGS).lock().unwrap().len() as i64 + 1,
                    name: String::from(value["name"].as_str().unwrap()),
                    age: value["age"].as_i64().unwrap() as i16
                };

                let value = json!(dog);
                (*MYDOGS).lock().unwrap().push(dog);
                Res::Created(value)
            }
            
            
            fn read(&self, key: String) -> Res 
            {
                let id = key.parse::<usize>().expect("Could not parse key!");
                match (*MYDOGS).lock().unwrap().get(id) {
                    Some(dog) => Res::Succ(Some(json!(dog))),
                    None      => Res::Err(Error::NotFound(String::from("Dogs")))
                }
            }

            
            fn read_list(&self) -> Res
            {
                Res::Succ(Some(json!(*MYDOGS)))
            }
        }


        // Start the service thread
        thread::spawn(move || {
            let m: Model = ModelBuilder::new("dog_cafe.svc")
                .add(Dogs::declare())
                .build();

            let s: Service = ServiceBuilder::new("apis")
                .add(m)
                .build();

            
            s.start();
        });
        
        // Technically a race ...but we are lazy so let's assume 200 msec is
        // enough for server to startup
        thread::sleep(Duration::from_millis(200));

        
        { // Test metadata
            println!("Checking Metadata");
            let client = Client::new();
            let url = "http://localhost:8080/apis/dog_cafe.svc/$metadata";
            let res = client.get(url).send().unwrap();
            assert_eq!(res.status, StatusCode::Ok);
        }
        
        { // Test read list
            println!("Checking Read List");
            let client = Client::new();
            let url = "http://localhost:8080/apis/dog_cafe.svc/Dogs";
            let res = client.get(url).send().unwrap();
            assert_eq!(res.status, StatusCode::Ok);
        }

        { // Test read single
            println!("Checking Read Single");
            let client = Client::new();
            let url = "http://localhost:8080/apis/dog_cafe.svc/Dogs(1)";
            let res = client.get(url).send().unwrap();
            assert_eq!(res.status, StatusCode::Ok);
        }

        { // Test read not found
            println!("Checking Read NotFound");
            let client = Client::new();
            let url = "http://localhost:8080/apis/dog_cafe.svc/Dogs(3)";
            let res = client.get(url).send().unwrap();
            assert_eq!(res.status, StatusCode::NotFound);
        }

        { // Test create
            println!("Checking Create");
            let client = Client::new();
            let mut url = "http://localhost:8080/apis/dog_cafe.svc/Dogs";
            let value = json!({ "age": 14, "name": "mynewdog" });
            let mut res = client.post(url).body(&value.to_string()).send().unwrap();
            assert_eq!(res.status, StatusCode::Created);

            // We should now be able to get the item we created
            println!("Checking Read Create");
            url = "http://localhost:8080/apis/dog_cafe.svc/Dogs(3)";
            res = client.get(url).send().unwrap();
            assert_eq!(res.status, StatusCode::Ok);
        }
    }
}
