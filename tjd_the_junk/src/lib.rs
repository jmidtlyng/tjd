extern crate tjd_api;
use std::collections::HashMap;
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::error::Error;
use tjd_api::TjdApiResponse;

#[derive(Debug)]
pub struct Types{
    pub type_list: HashMap<&'static str, TjdType>
}

impl Types {
    // initializes descriptions of raw types
    pub fn new() -> Types {
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
        match fs::read_to_string("seed.txt") {
            Ok(seed_text) => {
                let mut type_enum_string = seed_text.to_owned();
                // string to fill enum definitions
                let mut enum_defs = String::from("").to_owned();
                // write match case for thing trait's create fn
                let mut thing_impl = String::from("").to_owned();
                
                // loop types and build string
                for (type_key, tjd_type) in &self.type_list {
                    // add line to type enum string
                    enum_defs.push_str("    ");
                    // give enum strum message and detailed message
                    enum_defs.push_str("#[strum(message = \"");
                    enum_defs.push_str(tjd_type.display_name);
                    enum_defs.push_str("\", detailed_message = \"");
                    enum_defs.push_str(tjd_type.description);
                    enum_defs.push_str("\")]\n");
                    // #[strum(message = "Red", detailed_message = "This is very red")]
                    // add line to type enum string
                    enum_defs.push_str("    ");
                    
                    // add Rust type. end with newline
                    enum_defs.push_str(type_key);
                    enum_defs.push_str("(");
                    enum_defs.push_str(type_key);
                    enum_defs.push_str("),\n");
                    
                    // build match case for Thing create fn. start with indent
                    thing_impl.push_str("        ");
                    // match display name and output type
                    thing_impl.push_str("\"");
                    thing_impl.push_str(tjd_type.display_name);
                    // only None of no matches. return type from enum to inherit fns
                    thing_impl.push_str("\" => Some(TjdTypes::");
                    // new type and end with newline
                    thing_impl.push_str(type_key);
                    thing_impl.push_str("),\n");
                }
                
                type_enum_string.push_str(&enum_defs);
                
                // parse impl of Thing trait on TjdTypes enum
                match fs::read_to_string("soil.txt") {
                    Ok(soil_text) => {
                        let type_thing_impl_string = soil_text.to_owned();
                        // add type creator to output
                        type_enum_string.push_str(&type_thing_impl_string);
                        
                        // add enum type implementers set up during type loops
                        type_enum_string.push_str(&thing_impl);
                        
                        // close type implementors with None option and brackets
                        match fs::read_to_string("sunlight.txt") {
                            Ok(sunlight_text) => {
                                let type_thing_impl_close_string = sunlight_text.to_owned();
                                // push closing lines of implementor
                                type_enum_string.push_str(&type_thing_impl_close_string);
                                
                                // temp testing during dev
                                println!("{}", type_enum_string);
                                
                                // get writeable file and remove anything inside it before accessing
                                let file_result = OpenOptions::new().write(true).truncate(true)
                                                    .open("../tjd_the_drawer/src/tjd_type_enum.rs");
                                
                                // validate writing to file
                                match file_result {
                                    Ok(mut file) => {
                                        match file.write_all(type_enum_string.as_bytes()){
                                            Ok(_write_response) => {
                                                // return successful response
                                                TjdApiResponse {
                                                    success: true,
                                                    message: Some("Success!".to_owned()),
                                                    value: None
                                                }
                                            },
                                            Err(e) => {
                                                // return successful response
                                                TjdApiResponse {
                                                    success: false,
                                                    message: Some(format!("Failure! Error: {}", e)),
                                                    value: None
                                                }
                                            }
                                        }
                                    },
                                    Err(e) => {
                                        // return successful response
                                        TjdApiResponse {
                                            success: false,
                                            message: Some(format!{"Could not find file. Error: {}", e}),
                                            value: None
                                        }
                                    }
                                }
                            },
                            Error => {
                                // return successful response
                                TjdApiResponse {
                                    success: false,
                                    message: Some(String::from("Could not find sunlight.txt")),
                                    value: None
                                }
                            }
                        }
                    },
                    Error => {
                        // return successful response
                        TjdApiResponse {
                            success: false,
                            message: Some(String::from("Could not find soil.txt")),
                            value: None
                        }
                    }
                }
            },
            Error => {
                // return successful response
                TjdApiResponse {
                    success: false,
                    message: Some(String::from("Could not find seed.txt")),
                    value: None
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct TjdType {
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
        
        // make another type using the same rust type. expect failure to dupe type.
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
        match tjd_types.type_list.get("u32"){
            Some(default_type) =>
                assert_eq!(default_type.description, "Positive integer."),
            None => println!("Failed to get Non-negative Integer from Atomic Types")
        }
        match tjd_types.type_list.get("f64"){
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