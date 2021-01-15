use tokio_util::codec::{Decoder, Encoder};
use bytes::{BytesMut, Buf, BufMut};

use super::types::{DecodedPacket, PacketType, ConnectPacket, PublishPacket, PubackPacket, SubscribePacket, 
  SubackPacket, UnsubscribePacket, UnsubackPacket, PingReqPacket, PingRespPacket};
use super::error::{DecodeError, EncodeError};

pub struct MQTTCodec {}

impl Decoder for MQTTCodec {
  type Item = DecodedPacket;
  type Error = DecodeError;

  fn decode(&mut self, buffer: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
    if buffer.remaining() < 2 {
      return Ok(None);
    }

    let (packet_type, publish_config, remaining_length) = super::header::decode_fixed_header(buffer)?;
    println!("packet_type: {}", packet_type);
    if buffer.remaining() < remaining_length {
      return Ok(None);
    }

    let decoded_packet = match packet_type {
      PacketType::CONNECT => ConnectPacket::decode(buffer)?,
      PacketType::PUBLISH => {
        match publish_config {
          Some(config) => PublishPacket::decode(buffer, config, remaining_length)?,
          _ => return Err(DecodeError::FormatError) 
        }
      },
      PacketType::PUBACK => PubackPacket::decode(buffer, remaining_length)?,
      PacketType::SUBSCRIBE => SubscribePacket::decode(buffer, remaining_length)?,
      PacketType::SUBACK => SubackPacket::decode(buffer, remaining_length)?,
      PacketType::UNSUBSCRIBE => UnsubscribePacket::decode(buffer, remaining_length)?,
      PacketType::UNSUBACK => UnsubackPacket::decode(buffer, remaining_length)?,
      PacketType::PINGREQ => PingReqPacket::decode(),
      PacketType::PINGRESP => PingRespPacket::decode(),
      _ => return Err(DecodeError::FormatError)
    };

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
    super::header::encode_fixed_header(buffer, packet_type, &content, config)?;

    buffer.put(content);
    Ok(())
  }
}