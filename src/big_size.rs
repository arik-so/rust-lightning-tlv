use crate::big_size::EncodingDetails::{Length8, Length64, Length16, Length32};

const MIN_16_BIT_VALUE: u64 = 0xfd;
const MIN_32_BIT_VALUE: u64 = 0x10000;
const MIN_64_BIT_VALUE: u64 = 0x100000000;
const MAX_64_BIT_VALUE: u64 = 0xffffffffffffffff;

enum EncodingDetails {
	Length8,
	Length16,
	Length32,
	Length64,
}

impl EncodingDetails {
	fn length(&self) -> u8 {
		return match self {
			&Length8 => 1,
			&Length16 => 3,
			&Length32 => 5,
			&Length64 => 9
		};
	}

	fn prefix(&self) -> Option<u8> {
		return match self {
			&Length8 => None,
			&Length16 => Some(0xfd),
			&Length32 => Some(0xfe),
			&Length64 => Some(0xff)
		};
	}
}

pub struct BigSize {
	pub value: u64
}

impl BigSize {
	pub fn new(value: u64) -> Self {
		BigSize { value }
	}

	pub fn length(&self) -> u8 {
		self.encoding_details().length()
	}

	pub fn serialize(&self) -> Vec<u8> {
		let encoding_details = self.encoding_details();
		let prefix = encoding_details.prefix();

		if let None = prefix {
			return vec![self.value as u8];
		}

		let prefix = prefix.unwrap();
		let length = encoding_details.length();

		let bytes: [u8; 8] = self.value.to_be_bytes();
		let start_index = (8 - length + 1) as usize;
		let relevant_bytes = &bytes[start_index..8];

		let mut serialization = vec![prefix];
		serialization.extend_from_slice(relevant_bytes);

		serialization
	}

	fn encoding_details(&self) -> EncodingDetails {
		if self.value < MIN_16_BIT_VALUE {
			return Length8;
		}
		if self.value < MIN_32_BIT_VALUE {
			return Length16;
		}
		if self.value < MIN_64_BIT_VALUE {
			return Length32;
		}
		Length64
	}

	pub fn parse(undelimited_buffer: &[u8]) -> BigSize {
		let first_byte = undelimited_buffer[0];
		println!("first_byte: {}", first_byte);
		BigSize::new(23)
	}
}
