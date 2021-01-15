use bytes::{BytesMut, Buf, BufMut};

use super::error::{EncodeError, DecodeError};
use super::types::DecodedPacket;
use super::property::Property;
use super::reason_code::ReasonCode;

#[derive(Clone, Debug)]
pub struct UnsubackPacket {
  pub packet_id: u16,
  pub reason_codes: Vec<ReasonCode>,
  pub properties: Vec<Property>
}

impl super::types::Encode for UnsubackPacket {
  fn encode(&self, buffer: &mut BytesMut) -> Result<(), EncodeError> {
    buffer.put_u16(self.packet_id);

    Property::encode(buffer, &self.properties)?;

    for reason_code in &self.reason_codes {
      buffer.put_u8(*reason_code as u8);
    }

    Ok(())
  }
}

impl UnsubackPacket {
  pub fn decode(buffer: &mut BytesMut, remaining_length: usize) -> Result<DecodedPacket, DecodeError> {
    let starting_length = buffer.remaining();
    let packet_id = buffer.get_u16();

    let properties = Property::decode(buffer)?;

    let mut reason_codes = Vec::new();
    while super::utils::get_remaining_length(&buffer, starting_length, remaining_length) > 0 {
      reason_codes.push(ReasonCode::decode(buffer)?);
    }

    let packet = UnsubackPacket {
      packet_id: packet_id,
      reason_codes: reason_codes,
      properties: properties
    };

    Ok(DecodedPacket::Unsuback(packet))
  }
}
