use bytes::{BytesMut, Buf, BufMut};
use super::error::{EncodeError, DecodeError};

pub fn decode(buffer: &mut BytesMut) -> Result<u64, DecodeError>  {
  let mut multiplier = 1;
  let mut value = 0;
  while {
    let encoded_byte = buffer.get_u8();
    value += ((encoded_byte & 0x7F) as u64) * multiplier;
    if multiplier > 128 * 128 * 128 {
      return Err(DecodeError::FormatError);
    }
    multiplier *= 128;

    encoded_byte & 0x80 != 0
  } {}
  Ok(value)
}

pub fn encode(buffer: &mut BytesMut, mut val: u64) -> Result<(), EncodeError> {
  if val > 268435455 {
    return Err(EncodeError::VariableIntegerOutOfRangeError);
  }
  while {
    let mut encoded_byte = (val % 128) as u8;
    val /= 128;
    if val > 0 {
      encoded_byte |= 128;
    }
    buffer.put_u8(encoded_byte);

    val > 0
  } {}
  Ok(())
}
 
#[cfg(test)]
mod tests {
  use bytes::BytesMut;
  use super::super::error::{EncodeError, DecodeError};

  fn decode_util(slice: &[u8]) -> Result<u64, DecodeError> {
    let mut buffer = BytesMut::from(slice);
    let value = super::decode(&mut buffer)?;
    Ok(value)
  }

  #[test]
  fn decode_value() {
    assert_eq!(decode_util(&[0x00]).unwrap(), 0);
    assert_eq!(decode_util(&[0x7F]).unwrap(), 127);
    assert_eq!(decode_util(&[0x80, 0x01]).unwrap(), 128);
    assert_eq!(decode_util(&[0xFF, 0x7F]).unwrap(), 16383);
    assert_eq!(decode_util(&[0x80, 0x80, 0x01]).unwrap(), 16384);
    assert_eq!(decode_util(&[0xFF, 0xFF, 0x7F]).unwrap(), 2097151);
    assert_eq!(decode_util(&[0x80, 0x80, 0x80, 0x01]).unwrap(), 2097152);
    assert_eq!(decode_util(&[0xFF, 0xFF, 0xFF, 0x7F]).unwrap(), 268435455);
  }

  #[test]
  #[should_panic]
  fn decode_err() {
    decode_util(&[0x80, 0x80, 0x80, 0x80, 0x01]).unwrap(); 
  }

  fn encode_util(value: u64) -> Result<BytesMut, EncodeError> {
    let mut buffer = BytesMut::new();
    super::encode(&mut buffer, value)?;
    Ok(buffer)
  }

  #[test]
  fn encode_value() {
    assert_eq!(&encode_util(0).unwrap()[..], [0]);
    assert_eq!(&encode_util(127).unwrap()[..], &[0x7F]);
    assert_eq!(&encode_util(128).unwrap()[..], &[0x80, 0x01]);
    assert_eq!(&encode_util(16383).unwrap()[..], &[0xFF, 0x7F]);
    assert_eq!(&encode_util(16384).unwrap()[..], &[0x80, 0x80, 0x01]);
    assert_eq!(&encode_util(2097151).unwrap()[..], &[0xFF, 0xFF, 0x7F]);
    assert_eq!(&encode_util(2097152).unwrap()[..], &[0x80, 0x80, 0x80, 0x01]);
    assert_eq!(&encode_util(268435455).unwrap()[..], &[0xFF, 0xFF, 0xFF, 0x7F]);
  }

  #[test]
  #[should_panic]
  fn encode_err() {
    encode_util(268435456).unwrap();
  }
}