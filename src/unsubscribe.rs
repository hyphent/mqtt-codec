use bytes::{BytesMut, Buf, BufMut};

use crate::{
  error::{EncodeError, DecodeError},
  types::DecodedPacket,
  property::Property,
  utils::{decode_utf8, encode_utf8, get_remaining_length}
};


#[derive(Clone, Debug, PartialEq)]
pub struct UnsubscribePacket {
  pub packet_id: u16,
  pub topics: Vec<String>,
  pub properties: Vec<Property>
}

impl super::types::Encode for UnsubscribePacket {
  fn encode(&self, buffer: &mut BytesMut) -> Result<(), EncodeError> {
    buffer.put_u16(self.packet_id);

    Property::encode(buffer, &self.properties)?;

    for topic in &self.topics {
      encode_utf8(buffer, topic);
    }

    Ok(())
  }
}

impl UnsubscribePacket {
  pub fn decode(buffer: &mut BytesMut, remaining_length: usize) -> Result<DecodedPacket, DecodeError> {
    let starting_length = buffer.remaining();
    let packet_id = buffer.get_u16();

    let properties = Property::decode(buffer)?;

    let mut topics = Vec::new();

    while get_remaining_length(&buffer, starting_length, remaining_length) > 0 {
      let topic = decode_utf8(buffer)?;
      topics.push(topic);
    }

    let packet = UnsubscribePacket {
      packet_id: packet_id,
      topics: topics,
      properties: properties
    };

    Ok(DecodedPacket::Unsubscribe(packet))
  }
}

#[cfg(test)]
mod tests {
  use bytes::BytesMut;
  use crate::types::{Encode, DecodedPacket};
  use super::*;

  #[test]
  fn codec_test() {
    let packet = UnsubscribePacket {
      packet_id: 32,
      topics: vec!["test".to_owned(), "test1".to_owned()],
      properties: vec![]
    };

    let packet2 = packet.clone();
    let mut buffer = BytesMut::new();
    packet.encode(&mut buffer).unwrap();

    let remaining_length = buffer.remaining();
    let packet = UnsubscribePacket::decode(&mut buffer, remaining_length).unwrap();

    assert_eq!(DecodedPacket::Unsubscribe(packet2), packet);
  }
}
