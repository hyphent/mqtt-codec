use bytes::BytesMut;

use crate::error::EncodeError;

// import all the types
pub use crate::connect::{ConnectPacket, WillConfig};
pub use crate::connack::ConnackPacket;
pub use crate::publish::{PublishPacket, PublishConfig};
pub use crate::puback::PubackPacket;
pub use crate::subscribe::{SubscribePacket, SubscriptionConfig};
pub use crate::suback::SubackPacket;
pub use crate::unsubscribe::UnsubscribePacket;
pub use crate::unsuback::UnsubackPacket;
pub use crate::pingreq::PingReqPacket;
pub use crate::pingresp::PingRespPacket;
pub use crate::disconnect::DisconnectPacket;

pub use crate::property::Property;
pub use crate::reason_code::ReasonCode;

pub trait Encode {
  fn encode(&self, buffer: &mut BytesMut) -> Result<(), EncodeError>;
}

#[derive(Clone, Debug, PartialEq)]
pub enum DecodedPacket {
  Connect(ConnectPacket),
  Connack(ConnackPacket),
  Publish(PublishPacket),
  Puback(PubackPacket),
  Subscribe(SubscribePacket),
  Suback(SubackPacket),
  Unsubscribe(UnsubscribePacket),
  Unsuback(UnsubackPacket),
  PingReq(PingReqPacket),
  PingResp(PingRespPacket),
  Disconnect(DisconnectPacket)
}

impl DecodedPacket {
  pub fn encode(&self, buffer: &mut BytesMut) -> Result<(), EncodeError>{
    match self {
      DecodedPacket::Connack(item) => item.encode(buffer)?,
      DecodedPacket::Publish(item) => item.encode(buffer)?,
      DecodedPacket::Puback(item) => item.encode(buffer)?,
      DecodedPacket::Suback(item) => item.encode(buffer)?,
      DecodedPacket::Unsuback(item) => item.encode(buffer)?,
      DecodedPacket::PingReq(item) => item.encode(buffer)?,
      DecodedPacket::PingResp(item) => item.encode(buffer)?,
      DecodedPacket::Disconnect(item) => item.encode(buffer)?,
      _ => {}
    };
    Ok(())
  }

  pub fn get_type(&self) -> PacketType {
    match self {
      DecodedPacket::Connect(_) => PacketType::CONNECT,
      DecodedPacket::Connack(_) => PacketType::CONNACK,
      DecodedPacket::Publish(_) => PacketType::PUBLISH,
      DecodedPacket::Puback(_) => PacketType::PUBACK,
      DecodedPacket::Subscribe(_) => PacketType::SUBSCRIBE,
      DecodedPacket::Suback(_) => PacketType::SUBACK,
      DecodedPacket::Unsubscribe(_) => PacketType::UNSUBSCRIBE,
      DecodedPacket::Unsuback(_) => PacketType::UNSUBACK,
      DecodedPacket::PingReq(_) => PacketType::PINGREQ,
      DecodedPacket::PingResp(_) => PacketType::PINGRESP,
      DecodedPacket::Disconnect(_) => PacketType::DISCONNECT
    }
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PacketType {
  RESERVED,
  CONNECT,
  CONNACK,
  PUBLISH,
  PUBACK,
  PUBREC,
  PUBREL,
  PUBCOMP,
  SUBSCRIBE,
  SUBACK,
  UNSUBSCRIBE,
  UNSUBACK,
  PINGREQ,
  PINGRESP,
  DISCONNECT,
  AUTH
}

impl From<u8> for PacketType {
  fn from(num: u8) -> Self {
    match num {
      0 => PacketType::RESERVED,
      1 => PacketType::CONNECT,
      2 => PacketType::CONNACK,
      3 => PacketType::PUBLISH,
      4 => PacketType::PUBACK,
      5 => PacketType::PUBREC,
      6 => PacketType::PUBREL,
      7 => PacketType::PUBCOMP,
      8 => PacketType::SUBSCRIBE,
      9 => PacketType::SUBACK,
      10 => PacketType::UNSUBSCRIBE,
      11 => PacketType::UNSUBACK,
      12 => PacketType::PINGREQ,
      13 => PacketType::PINGRESP,
      14 => PacketType::DISCONNECT,
      15 => PacketType::AUTH,
      _ => panic!("{} is out of range", num)
    }
  }
}

impl std::fmt::Display for PacketType {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", match self {
      PacketType::RESERVED => "RESERVED",
      PacketType::CONNECT => "CONNECT",
      PacketType::CONNACK => "CONNACK",
      PacketType::PUBLISH => "PUBLISH",
      PacketType::PUBACK => "PUBACK",
      PacketType::PUBREC => "PUBREC",
      PacketType::PUBREL => "PUBREL",
      PacketType::PUBCOMP => "PUBCOMP",
      PacketType::SUBSCRIBE => "SUBSCRIBE",
      PacketType::SUBACK => "SUBACK",
      PacketType::UNSUBSCRIBE => "UNSUBSCRIBE",
      PacketType::UNSUBACK => "UNSUBACK",
      PacketType::PINGREQ => "PINGREQ",
      PacketType::PINGRESP => "PINGRESP",
      PacketType::DISCONNECT => "DISCONNECT",
      PacketType::AUTH => "AUTH"
    })
  }
}