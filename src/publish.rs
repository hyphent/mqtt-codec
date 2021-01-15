use bytes::{BytesMut, Buf, BufMut};

use super::error::{EncodeError, DecodeError};
use super::types::DecodedPacket;
use super::property::Property;

#[derive(Clone, Debug)]
pub struct PublishPacket {
  pub topic_name: String,
  pub packet_id: Option<u16>,
  pub payload: String,
  pub config: PublishConfig,
  pub properties: Vec<Property>
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct PublishConfig {
  pub dup: bool,
  pub qos: u8,
  pub retain: bool
}

impl super::types::Encode for PublishPacket {
  fn encode(&self, buffer: &mut BytesMut) -> Result<(), EncodeError> {
    super::utils::encode_utf8(buffer, &self.topic_name);
    
    match self.packet_id {
      Some(identifier) => buffer.put_u16(identifier),
      None => {}
    }

    Property::encode(buffer, &self.properties)?;

    buffer.put_slice(self.payload.as_bytes());

    Ok(())
  }
}

impl PublishPacket {
  pub fn decode(buffer: &mut BytesMut, publish_config: PublishConfig, remaining_length: usize) -> Result<DecodedPacket, DecodeError> {
    let starting_length = buffer.remaining();
    let topic_name = super::utils::decode_utf8(buffer)?;

    let packet_id = match publish_config.qos {
      0 => None,
      _ => Some(buffer.get_u16())
    };

    let properties = Property::decode(buffer)?;

    let payload = super::utils::decode_utf8_with_length(buffer, super::utils::get_remaining_length(&buffer, starting_length, remaining_length))?;

    let packet = PublishPacket {
      topic_name: topic_name,
      packet_id: packet_id,
      payload: payload,
      config: publish_config,
      properties: properties
    };

    Ok(DecodedPacket::Publish(packet))
  }
}
