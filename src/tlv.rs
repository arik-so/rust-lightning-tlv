use crate::big_size::BigSize;

pub struct TLV {
	type_id: u64,
	value: Vec<u8>,
}

impl TLV {
	pub fn new(type_id: u64, value: Vec<u8>) -> Self {
		TLV { type_id, value }
	}

	pub fn tlv_length(&self) -> usize {
		let type_length = BigSize::new(self.type_id).length() as usize;
		let data_length = self.data_length();
		let size_length = BigSize::new(data_length as u64).length() as usize;
		type_length + size_length + data_length
	}

	pub fn data_length(&self) -> usize {
		self.value.len()
	}

	pub fn type_id(&self) -> u64 {
		self.type_id
	}

	pub fn serialize(&self) -> Vec<u8> {
		let mut serialization = BigSize::new(self.type_id).serialize();

		let data_length = self.data_length();
		let length_serialization = BigSize::new(data_length as u64).serialize();
		serialization.extend_from_slice(length_serialization.as_slice());

		serialization.extend_from_slice(self.value.as_slice());
		serialization
	}
}


