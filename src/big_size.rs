use crate::big_size::EncodingDetails::{Length8, Length64, Length16, Length32};
use std::convert::TryInto;

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

	fn detect(prefix: u8) -> EncodingDetails {
		match prefix {
			0xfd => Length16,
			0xfe => Length32,
			0xff => Length64,
			_ => Length8
		}
	}
}

pub struct BigSize {
	value: u64
}

impl BigSize {
	pub fn new(value: u64) -> Self {
		BigSize { value }
	}

	pub fn length(&self) -> u8 {
		self.encoding_details().length()
	}

	pub fn value(&self) -> u64 {
		self.value
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

		serialization.to_owned()
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
		let encoding_details = EncodingDetails::detect(first_byte);

		let length = encoding_details.length() as usize;
		let mut offset: usize = 0;
		if encoding_details.prefix().is_some() {
			offset = 1;
		}

		let complement_length = 8 - length + offset;

		let relevant_bytes = undelimited_buffer[offset..length].to_vec();

		// create 8-byte-vector to parse values up to u64
		let mut big_endian_bytes: Vec<u8> = vec![0; complement_length];
		big_endian_bytes.extend_from_slice(&relevant_bytes);

		// create compile-time-known-length slice
		let mut big_endian_slice: [u8; 8] = Default::default();
		big_endian_slice.copy_from_slice(&big_endian_bytes[..]);

		// parse u64
		let value: u64 = u64::from_be_bytes(big_endian_slice);
		BigSize::new(value)
	}
}
