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
		let vector: Vec<u8> = vec![2, 3, 4];
		let slice = vector.as_slice();
		let big_size = BigSize::parse(slice);
		let big_size_2 = BigSize::parse(slice);
		let big_size_3 = BigSize::new(235312314);
		big_size_3.serialize();
	}
}
