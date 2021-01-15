use bytes::{BytesMut, Buf, BufMut};

use super::error::{EncodeError, DecodeError};
use super::types::DecodedPacket;
use super::property::Property;
use super::reason_code::ReasonCode;

#[derive(Clone, Debug)]
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
