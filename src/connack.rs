use bytes::{BytesMut, Buf, BufMut};

use super::error::{EncodeError, DecodeError};
use super::types::DecodedPacket;
use super::property::Property;
use super::reason_code::ReasonCode;

#[derive(Clone, Debug)]
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

