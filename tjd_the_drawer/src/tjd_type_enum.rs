use strum::{EnumMessage, IntoEnumIterator};
use strum_macros::{EnumIter, EnumMessage};

#[derive(EnumIter, EnumMessage, Debug)]
#[allow(dead_code)]
pub enum TjdTypes {
    #[strum(message = "Non-negative Integer", detailed_message = "Positive integer.")]
    u32(u32),
    #[strum(message = "Text", detailed_message = "Standard text.")]
    String(String),
    #[strum(message = "True/False", detailed_message = "True/false toggle.")]
    bool(bool),
    #[strum(message = "Decimal", detailed_message = "Number with decimals.")]
    f64(f64),
    #[strum(message = "Integer", detailed_message = "Standard integer.")]
    i32(i32),
}