extern crate tjd_api_response;
// use std::fmt::Display;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use tjd_api_response::TjdApiResponse;

#[derive(Debug)]
pub struct Types{
    type_list: HashMap<&'static str, TjdType>
}

impl Types {
    // initializes descriptions of raw types
    fn new() -> Types {
        // make an empty Types instance
        let mut tjd_types = Types{ type_list: HashMap::new() };
        
        // fill in starter types
        tjd_types.type_create("Integer", "i32", Some("Standard integer."), None);
        tjd_types.type_create("Non-negative Integer", "u32", Some("Positive integer."), None);
        tjd_types.type_create("Decimal", "f64", Some("Number with decimals."), None);
        tjd_types.type_create("Text", "String", Some("Standard text."), None);
        tjd_types.type_create("True/False", "bool", Some("True/false toggle."), None);
        
        // return available types
        tjd_types
    }
    
    // register custom type for frontend
    fn type_create(&mut self,
                    display_name: &'static str,
                    type_name: &'static str,
                    _description: Option<&'static str>,
                    _archived: Option<&'static bool>)
            -> TjdApiResponse<TjdType> {
        // is name available
        match self.type_list.get(type_name){
            Some(existing_type) => {
                // already set. error out
                TjdApiResponse {
                    success: false,
                    message: Some(format!("There is already a type for '{}'' named '{}'.",
                                            type_name, existing_type.display_name)),
                    value: None
                }
            },
            None => {
                // make static since it wont change
                let new_type = TjdType::new(display_name, _description, _archived);
                
                // create new atomic type and store reference
                self.type_list.insert(type_name, new_type);
                
                // return successful response
                TjdApiResponse {
                    success: true,
                    message: Some(format!("Type '{}' set.", type_name)),
                    value: None
                }
            }
        }
    }
    
    fn build(&self) -> TjdApiResponse<u8>{
        // get file
        let mut file_result = File::open("./tjd_type_enum.rs");
        
        match file_result {
            Ok(mut file) => {
                match file.write_all(b"Hello, world!"){
                    Ok(_write_response) => {
                        // return successful response
                        TjdApiResponse {
                            success: true,
                            message: Some("Success!".to_owned()),
                            value: None
                        }
                    },
                    Err(_e) => {
                        // return successful response
                        TjdApiResponse {
                            success: false,
                            message: Some("Failure!".to_owned()),
                            value: None
                        }
                    }
                }
            },
            Err(_e) => {
                // return successful response
                TjdApiResponse {
                    success: false,
                    message: Some("Could not find file".to_owned()),
                    value: None
                }
            }
        }
    }
}

#[derive(Debug)]
struct TjdType {
    display_name: &'static str,
    description: &'static str,
    archived: &'static bool
}

impl TjdType {
    fn new(display_name: &'static str,
            _description: Option<&'static str>,
            _archived: Option<&'static bool>) -> TjdType {
        // description falls back on empty string slice
        let description = _description.unwrap_or("");
        let archived = _archived.unwrap_or(&false);
        
        TjdType{display_name, description, archived}
    }
}

#[cfg(test)]
mod tests {
    // get context one layer above current
    use super::*;
    
    // create types to build foundation of TJD. check against dupe types and names.
    #[test]
    fn type_create(){
        // create instance of core. just a place to put stuff
        let mut tjd_types = Types::new();
                
        // make new type. expect success.
        let type_int = tjd_types.type_create("Test Type i8", "i8", None, None);
        assert_eq!(type_int.success, true);
        
        // make another type using the same rust type. expext failure to dupe type.
        let type_int_two = tjd_types.type_create("Test Type i8 redux", "i8", None, None);
        assert_eq!(type_int_two.success, false);
        
        // add fourth atomic type with dupe name and different type. expect success.
        let type_int_three = tjd_types.type_create("Test Type i8", "i16", None, None);
        assert_eq!(type_int_three.success, true);
    }
    
    // check prepackaged tjd types are installed 
    #[test]
    fn create_tjds_default_types(){
        // init
        let tjd_types = Types::new();
        
        // check initial types
        match tjd_types.type_list.get("i32"){
            Some(default_type) =>
                assert_eq!(default_type.description, "Standard integer."),
            None => println!("Failed to get Integer from Atomic Types")
        }
        match tjd_types.type_list.get("u16"){
            Some(default_type) =>
                assert_eq!(default_type.description, "Positive integer."),
            None => println!("Failed to get Non-negative Integer from Atomic Types")
        }
        match tjd_types.type_list.get("f32"){
            Some(default_type) =>
                assert_eq!(default_type.description, "Number with decimals."),
            None => println!("Failed to get Decimal from Atomic Types")
        }
        match tjd_types.type_list.get("String"){
            Some(default_type) =>
                assert_eq!(default_type.description, "Standard text."),
            None => println!("Failed to get Text from Atomic Types")
        }
        match tjd_types.type_list.get("bool"){
            Some(default_type) =>
                assert_eq!(default_type.description, "True/false toggle."),
            None => println!("Failed to get True/False from Atomic Types")
        }
    }
    
    // write all types to enum
    #[test]
    fn write_tjd_types_to_enum_file(){
        // init
        let tjd_types = Types::new();
        
        // write default types to enum file
        let tjd_build_response = tjd_types.build();
        
        match tjd_build_response.message {
            Some(msg) => println!("{}", msg),
            None => {}
        }
        
        // expect success
        assert_eq!(tjd_build_response.success, true);
    }
}