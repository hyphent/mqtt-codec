use bytes::{BytesMut, Buf, BufMut};

use crate::{
  error::{EncodeError, DecodeError},
  types::DecodedPacket,
  property::Property,
  utils::{decode_utf8, decode_utf8_with_length, encode_utf8, get_remaining_length}
};

#[derive(Clone, Debug, PartialEq)]
pub struct PublishPacket {
  pub topic: String,
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
    encode_utf8(buffer, &self.topic);
    
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
    let topic = decode_utf8(buffer)?;

    let packet_id = match publish_config.qos {
      0 => None,
      _ => Some(buffer.get_u16())
    };

    let properties = Property::decode(buffer)?;

    let payload = decode_utf8_with_length(buffer, get_remaining_length(&buffer, starting_length, remaining_length))?;

    let packet = PublishPacket {
      topic,
      packet_id,
      payload,
      config: publish_config,
      properties
    };

    Ok(DecodedPacket::Publish(packet))
  }
}

#[cfg(test)]
mod tests {
  use bytes::BytesMut;
  use crate::types::{Encode, DecodedPacket};
  use super::*;

  #[test]
  fn codec_test() {
    let publish_config = PublishConfig {
      dup: false,
      qos: 1,
      retain: true
    };

    let packet = PublishPacket {
      topic: "test".to_owned(),
      packet_id: Some(1234),
      payload: "hello".to_owned(),
      config: publish_config.clone(),
      properties: vec![]
    };

    let packet2 = packet.clone();
    let mut buffer = BytesMut::new();
    packet.encode(&mut buffer).unwrap();

    let remaining_length = buffer.remaining();
    let packet = PublishPacket::decode(&mut buffer, publish_config, remaining_length).unwrap();

    assert_eq!(DecodedPacket::Publish(packet2), packet);
  }
}