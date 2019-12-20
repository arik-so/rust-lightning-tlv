pub mod big_size;

#[cfg(test)]
mod tests {
	use crate::big_size::BigSize;

	#[test]
	fn it_works() {
		let big_size = BigSize::new(34);

		println!("length: {}", &big_size.length());
		assert_eq!(2 + 2, 4);
	}

	#[test]
	fn it_parses() {
		let value = 235312314;
		let big_size_3 = BigSize::new(value);
		let serialization = big_size_3.serialize();
		let deserialization = BigSize::parse(serialization.as_slice());
		assert_eq!(deserialization.value(), value);
	}
}
