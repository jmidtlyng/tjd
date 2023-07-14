use strum::{EnumMessage, IntoEnumIterator};
use strum_macros::{EnumIter, EnumMessage};

// all available types for making things
#[derive(EnumIter, EnumMessage, Debug)]
#[allow(dead_code)]
pub enum TjdTypes {    #[strum(message = "Decimal", detailed_message = "Number with decimals.")]
    f64(f64),
    #[strum(message = "True/False", detailed_message = "True/false toggle.")]
    bool(bool),
    #[strum(message = "Non-negative Integer", detailed_message = "Positive integer.")]
    u32(u32),
    #[strum(message = "Text", detailed_message = "Standard text.")]
    String(String),
    #[strum(message = "Integer", detailed_message = "Standard integer.")]
    i32(i32),
}

// look up input and output a type
pub fn thing_create(type_name: &str) -> Option<TjdTypes> {
		match type_name {
        "Decimal" => Some(TjdTypes::f64(0.0)),
        "True/False" => Some(TjdTypes::bool(false)),
        "Non-negative Integer" => Some(TjdTypes::u32(0)),
        "Text" => Some(TjdTypes::String(String::from(""))),
        "Integer" => Some(TjdTypes::i32(0)),
				_ => None,
		}
}