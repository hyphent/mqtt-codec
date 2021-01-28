use bytes::{BytesMut, Buf, BufMut};

use crate::{
  error::{EncodeError, DecodeError},
  types::DecodedPacket,
  property::Property,
  reason_code::ReasonCode,
  utils::get_remaining_length
};

#[derive(Clone, Debug, PartialEq)]
pub struct SubackPacket {
  pub packet_id: u16,
  pub reason_codes: Vec<ReasonCode>,
  pub properties: Vec<Property>
}

impl super::types::Encode for SubackPacket {
  fn encode(&self, buffer: &mut BytesMut) -> Result<(), EncodeError> {
    buffer.put_u16(self.packet_id);
    
    Property::encode(buffer, &self.properties)?;

    for reason_code in &self.reason_codes {
      buffer.put_u8(*reason_code as u8);
    }

    Ok(())
  }
}

impl SubackPacket {
  pub fn decode(buffer: &mut BytesMut, remaining_length: usize) -> Result<DecodedPacket, DecodeError> {
    let starting_length = buffer.remaining();
    let packet_id = buffer.get_u16();

    let properties = Property::decode(buffer)?;

    let mut reason_codes = Vec::new();
    while get_remaining_length(&buffer, starting_length, remaining_length) > 0 {
      reason_codes.push(ReasonCode::decode(buffer)?);
    }

    let packet = SubackPacket {
      packet_id: packet_id,
      reason_codes: reason_codes,
      properties: properties
    };

    Ok(DecodedPacket::Suback(packet))
  }
}

#[cfg(test)]
mod tests {
  use bytes::BytesMut;
  use crate::{
    types::{Encode, DecodedPacket},
    reason_code::ReasonCode
  };
  use super::*;

  #[test]
  fn codec_test() {
    let packet = SubackPacket {
      packet_id: 32,
      reason_codes: vec![ReasonCode::Success],
      properties: vec![]
    };

    let packet2 = packet.clone();
    let mut buffer = BytesMut::new();
    packet.encode(&mut buffer).unwrap();

    let remaining_length = buffer.remaining();
    let packet = SubackPacket::decode(&mut buffer, remaining_length).unwrap();

    assert_eq!(DecodedPacket::Suback(packet2), packet);
  }
}