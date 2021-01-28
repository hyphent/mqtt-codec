use bytes::{BytesMut, Buf, BufMut};

use crate::{
  error::{EncodeError, DecodeError},
  types::DecodedPacket,
  property::Property,
  reason_code::ReasonCode
};

#[derive(Clone, Debug, PartialEq)]pub struct PubackPacket {
  pub packet_id: u16,
  pub reason_code: ReasonCode,
  pub properties: Vec<Property>
}

impl super::types::Encode for PubackPacket {
  fn encode(&self, buffer: &mut BytesMut) -> Result<(), EncodeError> {
    buffer.put_u16(self.packet_id);
    buffer.put_u8(self.reason_code as u8);
  
    if self.properties.len() > 0 {
      Property::encode(buffer, &self.properties)?;
    }
    
    Ok(())
  }
}

impl PubackPacket {
  pub fn decode(buffer: &mut BytesMut, remaining_length: usize) -> Result<DecodedPacket, DecodeError> {
    let packet_id = buffer.get_u16();
    let reason_code = ReasonCode::decode(buffer)?;

    let properties = match remaining_length - 3 {
      x if x < 4 => Vec::new(),
      _ => Property::decode(buffer)?
    };

    let packet = PubackPacket {
      packet_id: packet_id,
      reason_code: reason_code,
      properties: properties
    };

    Ok(DecodedPacket::Puback(packet))
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
    let packet = PubackPacket {
      packet_id: 35,
      reason_code: ReasonCode::Success,
      properties: vec![]
    };

    let packet2 = packet.clone();
    let mut buffer = BytesMut::new();
    packet.encode(&mut buffer).unwrap();

    let remaining_length = buffer.remaining();
    let packet = PubackPacket::decode(&mut buffer, remaining_length).unwrap();

    assert_eq!(DecodedPacket::Puback(packet2), packet);
  }
}