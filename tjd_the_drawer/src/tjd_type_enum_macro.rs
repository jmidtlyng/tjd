// https://stackoverflow.com/questions/34953711/unwrap-inner-type-when-enum-variant-is-known
macro_rules! enum_thing {
		(
				enum $Name:ident {
						$($Variant:ident($f:ident)),* $(,)?
				}
		) => {
				#[derive(Debug)]
				pub enum $Name {
						$($Variant($Variant),)*
				}

				$(
						impl TryFrom<$Name> for $Variant {
								type Error = $Name;

								fn try_from(other: $Name) -> Result<Self, Self::Error> {
										match other {
												$Name::$Variant(v) => Ok(v),
												o => Err(o),
										}
								}
						}
				)*
		};
}

enum_thing! {
		enum Thing {
        i32(i32),
        u32(u32),
        f64(f64),
        String(String),
        bool(bool),
	  }
}

// look up input and output a type
pub fn thing_create(name: &str) -> Option<Thing> {
		match name {
        "Integer" => Some(Thing::i32(0)),
        "Non-negative Integer" => Some(Thing::u32(0)),
        "Decimal" => Some(Thing::f64(0.0)),
        "Text" => Some(Thing::String(String::from(""))),
        "True/False" => Some(Thing::bool(false)),
				_ => None,
		}
}