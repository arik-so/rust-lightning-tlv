pub mod big_size;
pub mod tlv;

#[cfg(test)]
mod tests {
	use crate::big_size::BigSize;
	use crate::tlv::TLV;

	#[test]
	fn it_works() {
		let big_size = BigSize::new(34);

		println!("length: {}", &big_size.length());
		assert_eq!(2 + 2, 4);
	}

	#[test]
	fn test_big_size_serialization_parsing() {
		let value = 235312314;
		let big_size_3 = BigSize::new(value);
		let serialization = big_size_3.serialize();
		let deserialization = BigSize::parse(serialization.as_slice());
		assert_eq!(deserialization.value(), value);
	}

	#[test]
	fn test_tlv_serialization_parsing() {
		let type_id = 235312314;
		let data = vec![110, 111, 112];
		let tlv = TLV::new(type_id, data.as_slice());
		let serialization = tlv.serialize();
		let restored_tlv = TLV::parse(serialization.as_slice());
		assert_eq!(restored_tlv.type_id(), type_id);
		assert_eq!(restored_tlv.tlv_length(), serialization.len());
		assert_eq!(restored_tlv.data().len(), data.len());
	}
}
