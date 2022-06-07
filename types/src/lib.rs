// use std::fmt::Display;
use std::collections::HashMap;
use std::sync::Mutex;

pub struct TJD {
    // Atomic types
    // Used in basic fields to build options for custom Types.
    // Atomic Types have a static str name and a basic struct of 
    // their Rust data type.
    
    // so far the only difference between Atomic and Regular Types is
    // Regular have options
    // fixed code name as first value of each:
    types: HashMap<&'static str, TjdType>,
    // fields: HashMap<&'static str, Field<T>>,
    // record data is grouped by type like table
    // records: HashMap<&'static str, RecordType<T>>
}

impl TJD {
    // initializes with the following atomic types:
    // 16 bit unsigned integer
    // 32 bit signed integer
    // 32 bit float
    // 256 byte string
    // String
    // bool
    fn new() -> TJD {
        // make an empty TJD instance
        let mut new_tjd = TJD{ types: HashMap::new() };
        
        // fill in starter types
        new_tjd.create_type(Mutex::new("Integer".to_string()), "i32");
        new_tjd.create_type(Mutex::new("Non-negative Integer".to_string()), "u32");
        new_tjd.create_type(Mutex::new("Decimal".to_string()), "f64");
        new_tjd.create_type(Mutex::new("Text".to_string()), "String");
        new_tjd.create_type(Mutex::new("True/False".to_string()), "bool");
        // fields: HashMap::new(),
        // records: HashMap::new()
        
        // return new tjd
        new_tjd
    }
    
    fn create_type(&mut self, display_name: Mutex<String>, type_name: &'static str) -> 
            TjdApiResponse<TjdType> {
        // is name available
        match self.types.get(type_name){
            Some(existing_type) => {
                // already set. error out
                TjdApiResponse {
                    success: false,
                    message: Some(format!("There is already a type for '{}'' named '{}'.",
                                            type_name, existing_type.display_name.lock().unwrap())),
                    value: None
                }
            },
            None => {
                // create new atomic type and store reference
                self.types.insert(type_name, TjdType{ display_name, type_name,
                                                        description: None, archived: false });
                
                // return successful response
                TjdApiResponse {
                    success: true,
                    message: Some(format!("Type '{}' set.", type_name)),
                    value: None
                }
            }
        }
    }
    /*            
    fn create_field(&mut self, display_name: &'static str, field_type_name: &'static str) ->
            TjdApiResponse<TjdType> {
        // check field name is available
        match self.fields.get(display_name){
            Some(field) => {
                // field name taken. notify client
                TjdApiResponse {
                    success: false,
                    message: Some(format!("Field name '{}' already taken", display_name)),
                    value: None
                }
            },
            None => {
                // check if atomic or regular type
                match self.types.get(field_type_name){
                    Some(atomic_type) => {
                        // make field out of matched atomic type
                        self.fields.insert(field_type_name, atomic_type);
                        // respond in positive
                        TjdApiResponse {
                            success: true,
                            message: Some(format!("Created field called {} of atomic type {}",
                                                    display_name, field_type_name)),
                            value: None
                        }
                    },
                    None => {
                        // invalid atomic type. for now err out.
                        // to do: check for regular type
                        TjdApiResponse {
                            success: false,
                            message: Some(format!("No matching atomic type named {}", display_name)),
                            value: None
                        }
                    }
                }
            }
        }
    } */
}

// default set archived to false creating any of these structs
// pub trait defaultArchivedFalse

// client reference to native and custom Rust types.
// no types or display names
struct TjdType {
    display_name: Mutex<String>,
    type_name: &'static str,
    description: Option<&'static str>,
    archived: bool
}
/*
    // tjd_type string references type in TJD
    // no dupe display name or type + options combinations
    struct Field <T>{
        display_name: &'static mut str,
        tjd_type: &'static str,
        options: Option<Record<T>>,
        archived: bool
    }
    
    // record types act both as model definition and table
    struct RecordType <T>{
        display_name: &'static str,
        fields: HashMap<&'static str, &'static str>,
        records: HashMap<&'static u64, Record<T>>,
        archived: bool
    }
    
    // each type needs to implement display
    struct Record <T: Display> {
        data: HashMap<&'static str, T>,
        archived: bool
    }
*/

// used to track exchanges with TJD api
struct TjdApiResponse<T> {
    success: bool,
    message: Option<String>,
    value: Option<T>
}

#[cfg(test)]
mod tests {
    // get context one layer above current
    use super::*;
    
    // new() creates an atomic type and registers it in Atomic_Types.
    // Atomic Type is type without options
    #[test]
    fn create_type(){
        // create instance of core. just a place to put stuff
        let mut tjd = TJD::new();
                
        // make new type. expect success.
        let type_int = tjd.create_type(Mutex::new("Test Type i8".to_string()), "i8");
        assert_eq!(type_int.success, true);
        
        // make another type using the same rust type. expext failure to dupe type.
        let type_int_two = tjd.create_type(Mutex::new("Test Type i8 redux".to_string()), "i8");
        assert_eq!(type_int_two.success, false);
        
        // add fourth atomic type with dupe name and different type. expect success.
        let type_int_three = tjd.create_type(Mutex::new("Test Type i8".to_string()), "i16");
        assert_eq!(type_int_three.success, true);
    }
    
    #[test]
    fn create_tjds_default_types(){
        // init
        let tjd = TJD::new();
        
        // check initial field types
        match tjd.types.get("i32"){
            Some(default_type) => assert_eq!(default_type.type_name, "i32"),
            None => println!("Failed to get Integer from Atomic Types")
        }
        match tjd.types.get("u16"){
            Some(default_type) => assert_eq!(default_type.type_name, "u16"),
            None => println!("Failed to get Non-negative Integer from Atomic Types")
        }
        match tjd.types.get("f32"){
            Some(default_type) => assert_eq!(default_type.type_name, "f32"),
            None => println!("Failed to get Decimal from Atomic Types")
        }
        match tjd.types.get("String"){
            Some(default_type) => assert_eq!(default_type.type_name, "String"),
            None => println!("Failed to get Text from Atomic Types")
        }
        match tjd.types.get("bool"){
            Some(default_type) => assert_eq!(default_type.type_name, "bool"),
            None => println!("Failed to get True/False from Atomic Types")
        }
    }
    /*
    #[test]
    fn create_field_from_atomic_type(){
        // init
        let mut tjd = TJD::new();
        
        // make test field for each atomic type
        for (type_name, atomic_type) in tjd.types {
            // make field name out of type name
            let mut new_field_name_starter: String = "Field ".to_owned();
            
            // alter field name to read "Field <Atomic Type> Example"
            let new_field_name = new_field_name_starter + type_name + " Example";
            
            // make new field
            let new_field_response = tjd.create_field(&new_field_name, type_name);
            
            // check new field type
            match tjd.fields.get(new_field_name){
                Some(field_data) => assert_eq!(field_data.field_type, atomic_type),
                None => println!("Failed to set up new field with atomic type {}", type_name)
            }
        }
    }
    */
}
