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
    fields: HashMap<&'static str, Field>,
    tables: HashMap<&'static str, Table>,
    // record data is grouped by type like table
    // records: HashMap<&'static str, RecordType<T>>
}

impl TJD {
    fn new(types: Types) -> TJD {
        // TJD instance with new types and empty field list
        let mut tjd = TJD{ types: types.type_list,
                            fields: HashMap::new(),
                            tables: HashMap::new() };
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
            
    // create a new table. Can include fields or not. Register with TJD.
    fn create_table(&mut self,
                    table_name: &'static str,
                    display_name: String,
                    _fields: Option<HashMap<&'static str, (String, &'static str)>>)
            -> TjdApiResponse<Table> {
        // check for table with same name
        match self.tables.get(&table_name){   
            Some(tbl) => {
                // field name taken. notify client
                TjdApiResponse {
                    success: false,
                    message: Some(format!("Table name '{}' already taken", table_name)),
                    value: None
                }
            },
            None => {
                // make new field ref
                let new_table = Table::new(display_name);
                // ref to trigger failure if bad field inputs
                let mut fields_error = None;
                
                // use entry or insert approach to grab ref for response
                self.tables.insert(table_name, new_table);
                
                // if there are fields, add them to the table
                match _fields {
                    Some(fields) => {
                        for (tbl_field_name, (tbl_field_display_name, field_name)) in fields {
                            let tbl_field_response =  self.create_table_field(
                                                            table_name,
                                                            tbl_field_name,
                                                            field_name,
                                                            tbl_field_display_name);
                                                            
                            if tbl_field_response.success == false {
                                fields_error = tbl_field_response.message;
                                break;
                            }
                        }
                    },
                    None => {}
                }
                
                match fields_error {
                    None => {
                        // respond in positive
                        TjdApiResponse {
                            success: true,
                            message: Some(format!("Created table called {}", table_name)),
                            value: None
                        }
                    },
                    Some(msg) => {
                        // remove table
                        self.tables.remove(table_name);
                        
                        // respond in negative and dont register the table to TJD
                        TjdApiResponse {
                            success: false,
                            message: Some(format!("Found conflict while adding initial \
                                                    fields.  Rolled back table. Field \
                                                    Error: {}", msg)),
                            value: None
                        }
                    }
                }
            }
        }
    }
            
    // create new table field from available field types
    fn create_table_field(&mut self,
                        tbl_name: &'static str,
                        tbl_field_name: &'static str,
                        field_name: &'static str,
                        display_name: String)
            -> TjdApiResponse<TableField> {
        // first validate field reference
        match self.fields.get(field_name){
            Some(_field) => {
                // get table from name
                match self.tables.get_mut(tbl_name){
                    Some(tbl) => {
                        // validate table doesnt already have this field
                        match tbl.fields.get(tbl_field_name) {
                            Some(_tbl_field) => {
                                // should not have had match. fail and warn
                                // field name taken. notify client
                                TjdApiResponse {
                                    success: false,
                                    message: Some(format!("Table already has field called '{}'.", tbl_field_name)),
                                    value: None
                                }
                            },
                            None => {
                                // no conflict so create and add to table
                                let new_field = TableField::new(display_name, field_name);
                                
                                // attach table field to tjd table
                                tbl.fields.insert(tbl_field_name, new_field);
                                
                                // notify client
                                TjdApiResponse {
                                    success: true,
                                    message: Some(format!("Added table field '{}' to table '{}'.",
                                                            tbl_field_name, tbl_name)),
                                    value: None
                                }
                            }
                        }
                    },
                    None => {
                        // no table matching this name
                        TjdApiResponse {
                            success: false,
                            message: Some(format!("No table called called '{}'.", tbl_name)),
                            value: None
                        }
                    }
                }
            },
            None => {
                // no table matching this name
                TjdApiResponse {
                    success: false,
                    message: Some(format!("Invalid field reference '{}'.", field_name)),
                    value: None
                }
            }
        }
    }
}

// client reference to native and custom Rust types.
// no types or display names
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
    fn new(display_name: String) -> Table{
        // just give a display name and default fields empty and archived false        
        Table { display_name: Mutex::new(display_name),
                fields: HashMap::new(),
                archived: false }
    }
}

#[derive(Debug)]
struct TableField {
    display_name: Mutex<String>,
    field: &'static str,
    archived: bool
}

impl TableField {
    fn new(display_name: String, field: &'static str) -> TableField {
        // give a display name and field reference   
        TableField { display_name: Mutex::new(display_name),
                    field,
                    archived: false }
    }
}

#[cfg(test)]
mod tests {
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
    #[test]    
    fn create_simple_table(){
        // init
        let tjd_types = Types::new();
        let mut tjd = TJD::new(tjd_types);
        
        // set up table fields
        let happy_tbl_fields: HashMap<&'static str, (String, &'static str)>
            = HashMap::from([
                ("test_field_i32", ("Test table field for integers".to_owned(), "i32")),
                ("test_field_u32", ("Test table field for positive integers".to_owned(), "u32")),
                ("test_field_String", ("Test table field for strings".to_owned(), "String")),
                ("test_field_bool", ("Test table field for booleans".to_owned(), "bool"))
            ]);
        
        // create new working table
        let happy_tbl_create_res = tjd.create_table(
                                            "happy_test_table",
                                            "Happy test table".to_owned(),
                                            Some(happy_tbl_fields));
                                 
        // log any issues if test didnt meet expectations
        if happy_tbl_create_res.success == false {
            match happy_tbl_create_res.message {
                Some(msg) => {
                    println!("{}", msg);
                }, None => {}
            }   
        }
                   
        assert_eq!(happy_tbl_create_res.success, true);
        
        // set up table fields where the field doesnt exist to make sure adding bad fields fails
        let fake_field_tbl_fields: HashMap<&'static str, (String, &'static str)>
            = HashMap::from([
                ("test_field_fake", ("Test table field for fake types".to_owned(), "fake_field")),
                ("test_field_u32", ("Test table field for positive integers".to_owned(), "u32")),
                ("test_field_String", ("Test table field for strings".to_owned(), "String")),
                ("test_field_bool", ("Test table field for booleans".to_owned(), "bool"))
            ]);
        
        // create new table
        let fake_field_tbl_create_res = tjd.create_table(
                                                "south_pole_test_table",
                                                "South pole test table".to_owned(),
                                                Some(fake_field_tbl_fields));
                                            
        assert_eq!(fake_field_tbl_create_res.success, false);
        
        // create new table with name of exiting table
        let repeat_tbl_create_res = tjd.create_table(
                                            "happy_test_table",
                                            "Repeat table name test table".to_owned(),
                                            None);
                                            
        assert_eq!(repeat_tbl_create_res.success, false);
        
        /*  note to self: tables need ability to archive table fields
            possibly removing a table field, then creating a new table field with
            the same name.
            
            Add test for accidentally allowing the same field name twice */
    }
    
    // add field to table
    #[test]
    fn add_field_to_table() {
        // init
        let tjd_types = Types::new();
        let mut tjd = TJD::new(tjd_types);
        
        // create new table with no fields
        let tbl_res = tjd.create_table(
                            "test_table",
                            "Test table".to_owned(),
                            None);
        
        // make sure that worked
        assert_eq!(tbl_res.success, true);
        
        match tbl_res.value {
            Some(_tbl) => {
                // add field to table
                let tjd_table_field_standard = tjd.create_table_field(
                                                        "test_table",
                                                        "i32",
                                                        "test_tbl_field",
                                                        "Test table field 1".to_owned());
                
                // should succeed
                assert_eq!(tjd_table_field_standard.success, true);
                
                // add second table field with the same name to the same table
                let tjd_table_field_dupe_name = tjd.create_table_field(
                                                        "test_table",
                                                        "bool",
                                                        "test_tbl_field",
                                                        "Test table field 1".to_owned());
                
                // should fail
                assert_eq!(tjd_table_field_dupe_name.success, false);
                
                // add third table field referencing an invalid field
                let tjd_table_field_nonexistent = tjd.create_table_field(
                                                        "test_table",
                                                        "abcdefghijdklmnopqrstuvwxyz",
                                                        "test_tbl_field_1",
                                                        "Test table field nonexistent".to_owned());
                
                // should fail
                assert_eq!(tjd_table_field_nonexistent.success, false);
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
