//! This module includes type information for a handful of Edm types.
//! Notably the date types are not yet implemented.
//! Type definition guidelines can be found [here][OASIS]
//! OASIS: http://docs.oasis-open.org/odata/odata-json-csdl/v4.0/csprd01/odata-json-csdl-v4.0-csprd01.html#_Toc441572996


#[macro_export]
macro_rules! rust_type {
    (Boolean) => { bool };
    (Byte) => { u8 };
    (Decimal) => { f32 };
    (Double) => { f64 };
    (Int16) => { i16 };
    (Int32) => { i16 };
    (Int64) => { i64 };
    (String) => { String };
}


pub mod Edm {

    pub enum Type {
        Boolean,
        Byte,
        Decimal,
        Double,
        Int16,
        Int32,
        Int64,
        String,
        //Binary([u8]),
        //DateTime:
        //DateTimeOffset:
        //Guid: Guid
        //SByte:,
        //Single:
        //Time: Timespan
    }

    pub fn from(s : &str) -> Type
    {
        println!("FROM = {}", s);
        match s {
            "Boolean" => Type::Boolean,
            "Byte" => Type::Byte,
            "Decimal" => Type::Decimal,
            "Double" => Type::Double,
            "Int16" => Type::Int16,
            "Int32" => Type::Int32,
            "Int64" => Type::Int64,
            "String" => Type::String,
            _ => panic!("Unable to parse invalid Edm::Type!")
        }
    }

    
    pub fn ty (ty : &Type) -> Vec<&str> {
        match ty { 
            &Type::Boolean => vec!["boolean"],
            &Type::Byte => vec!["integer"],
            &Type::Decimal => vec!["number", "string"],
            &Type::Double => vec!["number", "string"],
            &Type::Int16 => vec!["integer"],
            &Type::Int32 => vec!["integer"],
            &Type::Int64 => vec!["integer"],
            &Type::String => vec!["string"],
            _ => panic!()
        }
    }


    pub fn format (ty : &Type) -> &str {
        match ty {
            &Type::Boolean => "",
            &Type::Byte => "uint8",
            &Type::Decimal => "decimal",
            &Type::Double => "double",
            &Type::Int16 => "int16",
            &Type::Int32 => "int32",
            &Type::Int64 => "int64",
            &Type::String => "",
            _ => panic!()
        }
    }
}
