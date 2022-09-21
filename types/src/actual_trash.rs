// use quote::quote;

// extern crate quote;
// extern crate tjd_builder;
// #[macro_use]
// extern crate tjd_builder_derive;
/*
		use tjd_builder::TjdBuilder;
		
		//#[derive(TjdBuilder)]
		#[derive(Debug)]
		pub struct Types{
				type_list: HashMap<&'static str, TjdType>
		}
		
		impl Types {
				// initializes with the following atomic types:
				// 16 bit unsigned integer
				// 32 bit signed integer
				// 32 bit float
				// 256 byte string
				// String
				// bool
				fn new() -> Types {
						// make an empty TJD instance
						let mut tjd_types = Types{ type_list: HashMap::new() };
						
						// fill in starter types
						tjd_types.create_type(Mutex::new("Integer".to_string()), "i32", Some("Standard integer."));
						tjd_types.create_type(Mutex::new("Non-negative Integer".to_string()), "u32", Some("Positive integer."));
						tjd_types.create_type(Mutex::new("Decimal".to_string()), "f64", Some("Number with decimals."));
						tjd_types.create_type(Mutex::new("Text".to_string()), "String", Some("Standard text."));
						tjd_types.create_type(Mutex::new("True/False".to_string()), "bool", Some("True/false toggle."));
						
						// return new tjd
						tjd_types
				}
				
				// register custom type for frontend
				fn create_type(&mut self, display_name: Mutex<String>,
												type_name: &'static str,
												description: Option<&'static str>)
										-> TjdApiResponse<TjdType> {
						// is name available
						match self.type_list.get(type_name){
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
										self.type_list.insert(type_name, TjdType{ display_name, type_name,
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
						// build TJD using types
						fn build(&mut self) -> TJD {
								// write macro building enum from types
								!quote {
										enum TestEnum {
												#(#self.type_list),*
										}
								}
						}
				
		}
		
		tests
		
		// create types to build foundation of TJD. check against dupe types and names.
		#[test]
		fn create_type(){
				// create instance of core. just a place to put stuff
				let mut tjd_types = Types::new();
								
				// make new type. expect success.
				let type_int = tjd_types.create_type(Mutex::new("Test Type i8".to_string()), "i8", None);
				assert_eq!(type_int.success, true);
				
				// make another type using the same rust type. expext failure to dupe type.
				let type_int_two = tjd_types.create_type(Mutex::new("Test Type i8 redux".to_string()), "i8", None);
				assert_eq!(type_int_two.success, false);
				
				// add fourth atomic type with dupe name and different type. expect success.
				let type_int_three = tjd_types.create_type(Mutex::new("Test Type i8".to_string()), "i16", None);
				assert_eq!(type_int_three.success, true);
		}
		
		
		// test building TJD Types and then check prerolled types match expected
		#[test]
		fn build_tjd(){
				let tjd_types = Types::new();
				tjd_builder!(tjd_types);
								
				
				// enum_builder!("f64" , "i32" , "u32" , "String" , "bool");
				// enum_builder!(quote!{#(#type_list_keys),*});
				
				// build from standard
				// let tjd = Types::tjd_builder_macro();
				// let tjd = Types::build();
				
				// tjd_builder_macro!{type_list_keys};
				// mixed_rules!(trace testing = tjd_types.type_list.keys();)
				/*
						// check each type matches expected type
						match tjd.types.get("i32"){
								Some(default_type) => assert_eq!(default_type.tjd_type, TjdTypes::i32),
								None => println!("Failed to get Integer from Atomic Types")
						}
						match tjd.types.get("u16"){
								Some(default_type) => assert_eq!(default_type.tjd_type, TjdTypes::u16),
								None => println!("Failed to get Non-negative Integer from Atomic Types")
						}
						match tjd.types.get("f32"){
								Some(default_type) => assert_eq!(default_type.tjd_type, TjdTypes::f32),
								None => println!("Failed to get Decimal from Atomic Types")
						}
						match tjd.types.get("String"){
								Some(default_type) => assert_eq!(default_type.tjd_type, TjdTypes::String),
								None => println!("Failed to get Text from Atomic Types")
						}
						match tjd.types.get("bool"){
								Some(default_type) => assert_eq!(default_type.tjd_type, TjdTypes::bool),
								None => println!("Failed to get True/False from Atomic Types")
						}
				
		}
		
		
		// check prepackaged tjd types are installed 
		#[test]
		fn create_tjds_default_types(){
				// init
				let tjd_types = Types::new();
				
				// check initial types
				match tjd_types.type_list.get("i32"){
						Some(default_type) => assert_eq!(default_type.type_name, "i32"),
						None => println!("Failed to get Integer from Atomic Types")
				}
				match tjd_types.type_list.get("u16"){
						Some(default_type) => assert_eq!(default_type.type_name, "u16"),
						None => println!("Failed to get Non-negative Integer from Atomic Types")
				}
				match tjd_types.type_list.get("f32"){
						Some(default_type) => assert_eq!(default_type.type_name, "f32"),
						None => println!("Failed to get Decimal from Atomic Types")
				}
				match tjd_types.type_list.get("String"){
						Some(default_type) => assert_eq!(default_type.type_name, "String"),
						None => println!("Failed to get Text from Atomic Types")
				}
				match tjd_types.type_list.get("bool"){
						Some(default_type) => assert_eq!(default_type.type_name, "bool"),
						None => println!("Failed to get True/False from Atomic Types")
				}
		}
		
*/