use bytes::{BytesMut, Buf, BufMut};

pub fn decode_utf8(buffer: &mut BytesMut) -> Result<String, std::str::Utf8Error> {
  let string_length = buffer.get_u16();
  decode_utf8_with_length(buffer, string_length as usize)
}

pub fn decode_utf8_with_length(buffer: &mut BytesMut, string_length: usize) -> Result<String, std::str::Utf8Error> {
  let mut read_buffer = vec![0; string_length];
  buffer.copy_to_slice(&mut read_buffer);
  let ret = std::str::from_utf8(&read_buffer)?.to_string();
  Ok(ret)
}

pub fn encode_utf8(buffer: &mut BytesMut, string: &str) {
  buffer.put_u16(string.len() as u16);
  buffer.put_slice(string.as_bytes());
}

pub fn get_remaining_length(buffer: &BytesMut, starting_length: usize, remaining_length: usize) -> usize {
  let byte_written = starting_length - buffer.remaining();
  return remaining_length - byte_written;
}

#[cfg(test)]
mod tests {
  use bytes::{BytesMut, Buf};
  const TEST_BYTES: [u8; 7] = [0x00, 0x05, 0x41, 0xF0, 0xAA, 0x9B, 0x94];
  const TEST_STRING: &str = "A𪛔";

  #[test]
  fn decode_utf8_test() {
    let mut buffer = BytesMut::from(&TEST_BYTES[..]);
    assert_eq!(super::decode_utf8(&mut buffer).unwrap(), TEST_STRING);
  }

  #[test]
  fn decode_utf8_with_length_test() {
    let mut buffer = BytesMut::from(&TEST_BYTES[2..]);
    assert_eq!(super::decode_utf8_with_length(&mut buffer, 5).unwrap(), TEST_STRING);
  }

  #[test]
  fn encode_utf8_test() {
    let mut buffer = BytesMut::new();
    super::encode_utf8(&mut buffer, TEST_STRING);
    assert_eq!(&buffer[..], TEST_BYTES);
  }

  #[test]
  fn get_remaining_length_test() {
    let mut buffer = BytesMut::from(&TEST_BYTES[..]);
    let starting_length = buffer.remaining();
    buffer.get_u16();
    assert_eq!(super::get_remaining_length(&buffer, starting_length, 3), 1);
  }
}
