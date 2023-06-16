#[cfg(test)]
mod tests {
		// get context one layer above current
		use super::*;
		
		// create one thing of each type
		#[test]
		fn thing_create(){
			// init
			let tjd_types = Types::new();
			
			// following needs to be abstracted to one-liner:
			// get int type from available types
			let type_int = tjd_types.type_list.get("i32");
			// create new thing of type i32
			let thing_int = type_int.create();
			// next give init value
			thing_int = 10;
		}
}