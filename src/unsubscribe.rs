use bytes::{BytesMut, Buf};

use super::error::{DecodeError};
use super::types::DecodedPacket;
use super::property::Property;

#[derive(Clone, Debug)]
pub struct UnsubscribePacket {
  pub packet_id: u16,
  pub topic_names: Vec<String>,
  pub properties: Vec<Property>
}

impl UnsubscribePacket {
  pub fn decode(buffer: &mut BytesMut, remaining_length: usize) -> Result<DecodedPacket, DecodeError> {
    let starting_length = buffer.remaining();
    let packet_id = buffer.get_u16();

    let properties = Property::decode(buffer)?;

    let mut topic_names = Vec::new();

    while super::utils::get_remaining_length(&buffer, starting_length, remaining_length) > 0 {
      let topic_name = super::utils::decode_utf8(buffer)?;
      topic_names.push(topic_name);
    }

    let packet = UnsubscribePacket {
      packet_id: packet_id,
      topic_names: topic_names,
      properties: properties
    };

    Ok(DecodedPacket::Unsubscribe(packet))
  }
}
