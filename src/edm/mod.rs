//! This module includes type information for a handful of Edm types.
//! Notably the date types are not yet implemented.

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

    
    #[derive(Clone)]
    pub enum Value {
        Boolean(bool),
        Byte(u8),
        Decimal(f32),
        Double(f64),
        Int16(i16),
        Int32(i32),
        Int64(i64),
        String(String),
        //Binary([u8]),
        //DateTime:
        //DateTimeOffset:
        //Guid: Guid
        //SByte:,
        //Single:
        //Time: Timespan
    }

    
    /// Convert an Edm::Type to the underlying Rust type using the compound
    /// Edm::Value enum. 
    pub fn toValue (ty : &Type) -> Value {
        match ty {
            Boolean => Value::Boolean(false),
            Byte => Value::Byte(0),
            Decimal => Value::Decimal(0.),
            Double => Value::Double(0.),
            Int16 => Value::Int16(0),
            Int32 => Value::Int32(0),
            Int64 => Value::Int64(0),
            String => Value::String(String::from("")),
        }
    }
}
