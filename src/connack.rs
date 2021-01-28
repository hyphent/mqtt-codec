use bytes::{BytesMut, Buf, BufMut};

use crate::{
  error::{EncodeError, DecodeError},
  types::DecodedPacket,
  property::Property,
  reason_code::ReasonCode
};

#[derive(Clone, Debug, PartialEq)]
pub struct ConnackPacket {
  pub session_present: bool,
  pub reason_code: ReasonCode,
  pub properties: Vec<Property>
}

impl super::types::Encode for ConnackPacket {
  fn encode(&self, buffer: &mut BytesMut) -> Result<(), EncodeError> {
    if self.session_present {
      buffer.put_u8(0b10000000);
    } else {
      buffer.put_u8(0b00000000);
    }
    
    buffer.put_u8(self.reason_code as u8);
    
    Property::encode(buffer, &self.properties)?;

    Ok(())
  }
}

impl ConnackPacket {
  pub fn decode(buffer: &mut BytesMut) -> Result<DecodedPacket, DecodeError> {
    let session_present = match buffer.get_u8() {
      0b10000000 => true,
      0b00000000 => false,
      _ => return Err(DecodeError::FormatError)
    };

    let reason_code = ReasonCode::decode(buffer)?;
    let properties = Property::decode(buffer)?;

    let packet = ConnackPacket {
      session_present: session_present,
      reason_code: reason_code,
      properties: properties
    };

    Ok(DecodedPacket::Connack(packet))
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
    let packet = ConnackPacket {
      session_present: true,
      reason_code: ReasonCode::Success,
      properties: vec![]
    };

    let packet2 = packet.clone();
    let mut buffer = BytesMut::new();
    packet.encode(&mut buffer).unwrap();

    let packet = ConnackPacket::decode(&mut buffer).unwrap();

    assert_eq!(DecodedPacket::Connack(packet2), packet);
  }
}