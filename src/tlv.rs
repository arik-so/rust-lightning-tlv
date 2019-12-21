use crate::big_size::BigSize;

pub struct TLV {
	type_id: u64,
	value: Vec<u8>,
}

impl TLV {
	pub fn new(type_id: u64, value: &[u8]) -> Self {
		TLV { type_id, value: value.to_owned() }
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

	pub fn parse(undelimited_buffer: &[u8]) -> TLV {
		let type_id = BigSize::parse(undelimited_buffer);

		let length_buffer = undelimited_buffer[type_id.length()..];
		let length = BigSize::parse(length_buffer);

		let data_buffer = length_buffer[length.length()..];
		let data = data_buffer[..length.value()];

		TLV::new(type_id.value(), data)
	}
}


