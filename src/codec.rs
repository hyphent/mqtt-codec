use tokio_util::codec::{Decoder, Encoder};
use bytes::{BytesMut, Buf, BufMut};

use crate::{
  types::*,
  error::{DecodeError, EncodeError},
  header::*
};

pub struct MQTTCodec {}

impl Decoder for MQTTCodec {
  type Item = DecodedPacket;
  type Error = DecodeError;

  fn decode(&mut self, buffer: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
    let mut read_buffer = buffer.clone();
    if read_buffer.remaining() < 2 {
      return Ok(None);
    }

    let (packet_type, publish_config, remaining_length) = decode_fixed_header(&mut read_buffer)?;
    if read_buffer.remaining() < remaining_length {
      return Ok(None);
    }

    let decoded_packet = match packet_type {
      PacketType::CONNECT => ConnectPacket::decode(&mut read_buffer)?,
      PacketType::CONNACK => ConnackPacket::decode(&mut read_buffer)?,
      PacketType::PUBLISH => {
        match publish_config {
          Some(config) => PublishPacket::decode(&mut read_buffer, config, remaining_length)?,
          _ => return Err(DecodeError::FormatError) 
        }
      },
      PacketType::PUBACK => PubackPacket::decode(&mut read_buffer, remaining_length)?,
      PacketType::SUBSCRIBE => SubscribePacket::decode(&mut read_buffer, remaining_length)?,
      PacketType::SUBACK => SubackPacket::decode(&mut read_buffer, remaining_length)?,
      PacketType::UNSUBSCRIBE => UnsubscribePacket::decode(&mut read_buffer, remaining_length)?,
      PacketType::UNSUBACK => UnsubackPacket::decode(&mut read_buffer, remaining_length)?,
      PacketType::PINGREQ => PingReqPacket::decode(&mut read_buffer)?,
      PacketType::PINGRESP => PingRespPacket::decode(&mut read_buffer)?,
      _ => return Err(DecodeError::FormatError)
    };

    buffer.advance(buffer.remaining() - read_buffer.remaining());
    Ok(Some(decoded_packet))
  }
}

impl Encoder<DecodedPacket> for MQTTCodec {
  type Error = EncodeError;
  fn encode(&mut self, item: DecodedPacket, buffer: &mut BytesMut) -> Result<(), Self::Error> {
    let mut content = bytes::BytesMut::new();
    item.encode(&mut content)?;

    let packet_type = item.get_type();
    buffer.reserve(content.len());
    let config = match item {
      DecodedPacket::Publish(packet) => Some(packet.config),
      _ => None
    };
    encode_fixed_header(buffer, packet_type, &content, config)?;

    buffer.put(content);
    Ok(())
  }
}