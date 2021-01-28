use bytes::{BytesMut, BufMut};

use crate::{
  error::{EncodeError, DecodeError},
  types::DecodedPacket,
  property::Property,
  reason_code::ReasonCode
};


#[derive(Clone, Debug, PartialEq)]
pub struct DisconnectPacket {
  pub reason_code: ReasonCode,
  pub properties: Vec<Property>
}

impl super::types::Encode for DisconnectPacket {
  fn encode(&self, buffer: &mut BytesMut) -> Result<(), EncodeError> {
    buffer.put_u8(self.reason_code as u8);
    Property::encode(buffer, &self.properties)?;
    Ok(())
  }
}

impl DisconnectPacket {
  pub fn decode(buffer: &mut BytesMut) -> Result<DecodedPacket, DecodeError> {
    let reason_code = ReasonCode::decode(buffer)?;
    let properties = Property::decode(buffer)?;
    let packet = DisconnectPacket {
      reason_code: reason_code,
      properties: properties
    };

    Ok(DecodedPacket::Disconnect(packet))
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
    let packet = DisconnectPacket {
      reason_code: ReasonCode::Success,
      properties: vec![]
    };

    let packet2 = packet.clone();
    let mut buffer = BytesMut::new();
    packet.encode(&mut buffer).unwrap();

    let packet = DisconnectPacket::decode(&mut buffer).unwrap();

    assert_eq!(DecodedPacket::Disconnect(packet2), packet);
  }
}