// use std::fmt::Display;
use std::collections::HashMap;
use std::sync::Mutex;


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
}

pub struct TJD {
    // Atomic types
    // Used in basic fields to build options for custom Types.
    // Atomic Types have a static str name and a basic struct of 
    // their Rust data type.
    // so far the only difference between Atomic and Regular Types is
    // Regular have options
    // fixed code name as first value of each:
    types: HashMap<&'static str, TjdType>,
    fields: HashMap<&'static str, Field>,
    // record data is grouped by type like table
    // records: HashMap<&'static str, RecordType<T>>
}

impl TJD {
    fn new(types: Types) -> TJD {
        // TJD instance with new types and empty field list
        let mut tjd = TJD{ types: types.type_list, fields: HashMap::new() };
        // make vector of keys by cloning type keys
        let test_keys: Vec<&'static str> = tjd.types.keys().cloned().collect();
        
        // make test field for each atomic type
        for type_name in test_keys {
            // alter field name to read "Field <TjdType> Example"
            let new_field_name = "Field ".to_owned() + type_name + " Example";
            
            // make new field without options
            tjd.create_field(type_name, new_field_name, type_name);
        }
        
        tjd
    }
    
    // register custom field for frontend
    fn create_field(&mut self,
                    field_name: &'static str,
                    display_name: String,
                    type_name: &'static str)
            -> TjdApiResponse<TjdType> {
        match self.types.get(&type_name){
            Some(_tjd_type) => {
                // use try insert to not overwrite fields
                match self.fields.get(&field_name){
                    Some(_field) => {
                        // field name taken. notify client
                        TjdApiResponse {
                            success: false,
                            message: Some(format!("Field name '{}' already taken", field_name)),
                            value: None
                        }
                    },
                    None => {
                        // make new field ref
                        let new_field = Field::new(display_name, type_name, None);
                        // use entry or insert approach to grab ref for response
                        self.fields.insert(field_name, new_field);
                        
                        // respond in positive
                        TjdApiResponse {
                            success: true,
                            message: Some(format!("Created field called {} with type {}",
                                                    field_name, type_name)),
                            value: None
                        }
                    }
                }
            },
            None => {
                // field name taken. notify client
                TjdApiResponse {
                    success: false,
                    message: Some(format!("No type matching '{}'", type_name)),
                    value: None
                }
            }
        }
    }
}

// client reference to native and custom Rust types.
// no types or display names
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

#[derive(Debug)]
struct Field {
    display_name: Mutex<String>,
    tjd_type: &'static str,
    archived: bool
}

impl Field {
    fn new(display_name: String, tjd_type: &'static str, _archived: Option<bool>) -> Field{
        let archived = _archived.unwrap_or(false);
        
        Field {display_name: Mutex::new(display_name), tjd_type, archived}
    }
}

#[derive(Debug)]
struct Table {
    display_name: Mutex<String>,
    fields: HashMap<&'static str, TableField>,
    archived: bool
}

impl Table {
    fn new(display_name: String,  _archived: Option<bool>) -> Table{
        let archived = _archived.unwrap_or(false);
        
        Table { display_name: Mutex::new(display_name),
                fields: HashMap::new(),
                archived}
    }
}

#[derive(Debug)]
struct TableField {
    display_name: Mutex<String>,
    field: &'static str,
    archived: bool
}

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
    
    // make starter/default fields out of all types
    #[test]
    fn default_fields(){
        // init
        let tjd_types = Types::new();
        let tjd = TJD::new(tjd_types);
        
        // make test field for each atomic type
        for type_name in tjd.types.keys() {
            // try getting each atomic field
            match tjd.fields.get(type_name){
                Some(default_field) => {
                    // lock display name to use in comp
                    let display_name = default_field.display_name.lock().unwrap();
                    
                    // check display name by formatting type name to match expected
                    assert_eq!(*display_name, "Field ".to_owned() + type_name + " Example");
                },
                None => println!("Failed to get default field: {}", type_name)
            }
        }
    }
    
    // make a basic table out of several fields
    fn create_simple_table(){
        // init
        let tjd_types = Types::new();
        let tjd = TJD::new(tjd_types);
        
        // give table a name
        let happy_tbl_name = "Happy test table";
        // set up table fields
        let mut happy_tbl_fields: HashMap<String, &'static str> = HashMap::new();
        
        // loop default fields to make a test
        for field in tjd.fields.keys() {
            // style an example table field name
            let tbl_field_name = "Table test field for ".to_owned() + field;
            // add example table field to test table
            happy_tbl_fields.insert(tbl_field_name, field);
        }
        
        // create new table
        let happy_tbl_create_res = tjd.create_table(happy_tbl_name, happy_tbl_fields);
        assert_eq!(happy_tbl_create_res.success, true);
        
        // try making a second table with nonexistent fields
        let conflict_tbl_name = "South pole test table";
        // set up table fields
        let mut conflict_tbl_fields: HashMap<String, &'static str> = HashMap::new();
        
        // loop default fields to make a test
        for field in tjd.fields.keys() {
            // style an example table field name
            let tbl_field_name = "Table test field for ".to_owned() + field;
            
            // add example table field to test table with bad field ref name
            conflict_tbl_fields.insert(tbl_field_name, "fake_field");
        }
        
        // create new table
        let conflict_tbl_create_res = tjd.create_table(conflict_tbl_name, conflict_tbl_fields);
        assert_eq!(conflict_tbl_create_res.success, false);
        
        /* note to self: tables need ability to archive table fields
            possibly removing a table field, then creating a new table field with
            the same name.
            
            Add test for accidentally allowing the same field name twice */
    }
    
    // add field to table
    fn add_field_to_table() {
        // init
        let tjd_types = Types::new();
        let tjd = TJD::new(tjd_types);
        
        // create new table with no fields
        let tbl_res = tjd.create_table("Test table".to_owned(), None);
        
        match tbl_res.value {
            Some(tbl) => {
                // add field to table
                tjd.add_table_field(tbl, "i32", "Test table field 1")
            },
            None => {
                // not supposed to happen. Return error message
                match tbl_res.message {
                    Some(message) => println!("{}", message),
                    None => println!("No specific failure message to create a table")
                }
            }
        }        
    }
}
