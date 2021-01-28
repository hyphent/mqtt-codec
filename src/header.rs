use bytes::{BytesMut, Buf, BufMut};

use crate::{
  error::{EncodeError, DecodeError},
  variable_integer,
  types::PacketType,
  publish::PublishConfig
};

pub fn encode_fixed_header(buffer: &mut BytesMut, packet_type: PacketType, payload: &BytesMut, 
  config: Option<PublishConfig>) -> Result<(), EncodeError> {

  let header = (packet_type as u8) << 4;
  let first_byte: u8 = match packet_type {
    PacketType::PUBLISH => {
      match config {
        Some(config) => header + ((config.dup as u8) << 3) + (config.qos << 1) + (config.retain as u8),
        None => return Err(EncodeError::FormatError)
      }
    },
    PacketType::PUBREL | PacketType::SUBSCRIBE | PacketType::UNSUBSCRIBE => header + 0b0010,
    _ => header
  };

  buffer.put_u8(first_byte);

  variable_integer::encode(buffer, payload.len() as u64)?;

  Ok(())
}

pub fn decode_fixed_header(buffer: &mut BytesMut) -> Result<(PacketType, Option<PublishConfig>, usize), DecodeError> {
  let first_byte = buffer.get_u8();
  let type_bits = (first_byte & 0b11110000) >> 4;
  let packet_type: PacketType = match type_bits {
    0..=15 => type_bits.into(),
    _ => return Err(DecodeError::FormatError)
  };
  let remaining_length = variable_integer::decode(buffer)?;

  let flags = first_byte & 0b1111;
  let publish_config = match packet_type {
    PacketType::PUBLISH => Some(PublishConfig {
      dup: (flags & 0b1000) == 0b1000,
      qos: (flags & 0b0110) >> 1,
      retain: (flags & 0b0001) == 0b0001
    }),
    PacketType::PUBREL | PacketType::SUBSCRIBE | PacketType::UNSUBSCRIBE => {
      if flags != 0b0010 {
        return Err(DecodeError::FormatError);
      }
      None
    },
    _ => None
  };

  Ok((packet_type, publish_config, remaining_length as usize))
}

#[cfg(test)]
mod tests {
  use bytes::BytesMut;

  use crate::types::{PacketType::*, PublishConfig};
  use super::*;

  const ENCODE_PAYLOAD: [u8; 2] = [0x01, 0x02];

  #[test]
  fn encode_fixed_header_general_test() {
    let payload = BytesMut::from(&ENCODE_PAYLOAD[..]);
    for packet_type in [RESERVED, CONNECT, CONNACK, PUBACK, PUBREC, PUBCOMP, SUBACK, UNSUBACK, PINGREQ, PINGRESP, DISCONNECT, AUTH].iter() {
      let mut buffer = BytesMut::new();
      encode_fixed_header(&mut buffer, *packet_type, &payload, None).unwrap();
      assert_eq!(&buffer[..], [(*packet_type as u8) << 4, 0x02]);
    }
  }

  #[test]
  fn encode_fixed_header_special_flags_test() {
    let payload = BytesMut::from(&ENCODE_PAYLOAD[..]);
    for packet_type in [PUBREL, SUBSCRIBE, UNSUBSCRIBE].iter() {
      let mut buffer = BytesMut::new();
      encode_fixed_header(&mut buffer, *packet_type, &payload, None).unwrap();
      assert_eq!(&buffer[..], [((*packet_type as u8) << 4) + 0b0010, 0x02]);
    }
  }

  #[test]
  fn encode_fixed_header_publish_test() {
    let payload = BytesMut::from(&ENCODE_PAYLOAD[..]);
    for i in 0..12 {
      let mut buffer = BytesMut::new();
      let config = PublishConfig { dup: (i % 2) != 0, qos: ((i / 2) % 3), retain: ((i / 6) % 2) != 0 };
      let first_byte = (3 << 4) + ((config.dup as u8) << 3) + (config.qos << 1) + (config.retain as u8);
      encode_fixed_header(&mut buffer, PUBLISH, &payload, Some(config)).unwrap();
      assert_eq!(&buffer[..], [first_byte, 0x02]);
    }
  }

  #[test]
  fn decode_fixed_header_general_test() {
    for expected_packet_type in [RESERVED, CONNECT, CONNACK, PUBACK, PUBREC, PUBCOMP, SUBACK, UNSUBACK, PINGREQ, PINGRESP, DISCONNECT, AUTH].iter() {
      let mut buffer = BytesMut::from(&[(*expected_packet_type as u8) << 4, 0x02][..]);
      let (packet_type, publish_config, remaining_length) = decode_fixed_header(&mut buffer).unwrap();
      assert_eq!(packet_type, *expected_packet_type);
      assert_eq!(publish_config.is_none(), true);
      assert_eq!(remaining_length, 2);
    }
  }

  #[test]
  fn decode_fixed_header_special_flags_test() {
    for expected_packet_type in [PUBREL, SUBSCRIBE, UNSUBSCRIBE].iter() {
      let mut buffer = BytesMut::from(&[((*expected_packet_type as u8) << 4) + 0b0010, 0x02][..]);
      let (packet_type, publish_config, remaining_length) = decode_fixed_header(&mut buffer).unwrap();
      assert_eq!(packet_type, *expected_packet_type);
      assert_eq!(publish_config.is_none(), true);
      assert_eq!(remaining_length, 2);
    }
  }

  #[test]
  fn decode_fixed_header_publish_test() {
    for i in 0..12 {
      let config = PublishConfig { dup: (i % 2) != 0, qos: ((i / 2) % 3), retain: ((i / 6) % 2) != 0 };
      let first_byte = (3 << 4) + ((config.dup as u8) << 3) + (config.qos << 1) + (config.retain as u8);

      let mut buffer = BytesMut::from(&[first_byte, 0x02][..]);
      let (packet_type, publish_config, remaining_length) = decode_fixed_header(&mut buffer).unwrap();
      assert_eq!(packet_type, PUBLISH);
      assert_eq!(publish_config, Some(config));
      assert_eq!(remaining_length, 2);
    }
  }
}
