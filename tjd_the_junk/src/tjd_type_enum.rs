use strum::EnumMessage;
use strum_macros;

#[derive(strum_macros::EnumMessage, Debug)]
#[allow(dead_code)]
enum TjdTypes {
    #[strum(message = "Non-negative Integer", detailed_message = "Positive integer.")]
    u32(u32),
    #[strum(message = "Decimal", detailed_message = "Number with decimals.")]
    f64(f64),
    #[strum(message = "Text", detailed_message = "Standard text.")]
    String(String),
    #[strum(message = "Integer", detailed_message = "Standard integer.")]
    i32(i32),
    #[strum(message = "True/False", detailed_message = "True/false toggle.")]
    bool(bool),
}