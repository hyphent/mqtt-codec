use bytes::{BytesMut, Buf, BufMut};

use crate::{
  error::{EncodeError, DecodeError},
  variable_integer,
  utils::{decode_utf8, encode_utf8, get_remaining_length}
};

#[derive(Clone, Debug, PartialEq)]
pub enum Property {
  PayloadFormatIndicator(u8),
  MessageExpiryInterval(u32),
  ContentType(String),
  ResponseTopic(String),
  CorrelationData(Vec<u8>),
  SubscriptionIdentifier(u64),
  SessionExpiryInterval(u32),
  AssignedClientIdentifier(String),
  ServerKeepAlive(u16),
  AuthenticationMethod(String),
  AuthenticationData(Vec<u8>),
  RequestProblemInformation(u8),
  WillDelayInterval(u32),
  RequestResponseInformation(u8),
  ResponseInformation(String),
  ServerReference(String),
  ReasonString(String),
  ReceiveMaximum(u16),
  TopicAliasMaximum(u16),
  TopicAlias(u16),
  MaximumQoS(u8),
  RetainAvailable(u8),
  UserProperty((String, String)),
  MaximumPacketSize(u32),
  WildcardSubscriptionAvailable(bool),
  SubscriptionIdentifierAvailable(bool),
  SharedSubscriptionAvailable(bool)
}

impl Property {
  pub fn decode(buffer: &mut BytesMut) -> Result<Vec<Property>, DecodeError>  {
    let remaining_length = variable_integer::decode(buffer)? as usize;

    let starting_length = buffer.remaining();
    let mut properties = Vec::new();
    while get_remaining_length(&buffer, starting_length, remaining_length) > 0 {
      let property = match buffer.get_u8() {
        0x01 => Property::PayloadFormatIndicator(buffer.get_u8()),
        0x02 => Property::MessageExpiryInterval(buffer.get_u32()),
        0x08 => Property::ContentType(decode_utf8(buffer)?),
        0x09 => Property::ResponseTopic(decode_utf8(buffer)?),
        0x0B => return Err(DecodeError::FormatError),
        0x11 => Property::SubscriptionIdentifier(variable_integer::decode(buffer)?),
        0x12 => Property::SessionExpiryInterval(buffer.get_u32()),
        0x13 => Property::AssignedClientIdentifier(decode_utf8(buffer)?),
        0x14 => Property::ServerKeepAlive(buffer.get_u16()),
        0x15 => Property::AuthenticationMethod(decode_utf8(buffer)?),
        0x16 => return Err(DecodeError::FormatError),
        0x17 => Property::RequestProblemInformation(buffer.get_u8()),
        0x18 => Property::WillDelayInterval(buffer.get_u32()),
        0x19 => Property::RequestResponseInformation(buffer.get_u8()),
        0x1A => Property::ResponseInformation(decode_utf8(buffer)?),
        0x1C => Property::ServerReference(decode_utf8(buffer)?),
        0x1F => Property::ReasonString(decode_utf8(buffer)?),
        0x21 => Property::ReceiveMaximum(buffer.get_u16()),
        0x22 => Property::TopicAliasMaximum(buffer.get_u16()),
        0x23 => Property::TopicAlias(buffer.get_u16()),
        0x24 => Property::MaximumQoS(buffer.get_u8()),
        0x25 => Property::RetainAvailable(buffer.get_u8()),
        0x26 => Property::UserProperty((decode_utf8(buffer)?, decode_utf8(buffer)?)),
        0x27 => Property::MaximumPacketSize(buffer.get_u32()),
        0x28 => Property::WildcardSubscriptionAvailable(buffer.get_u8() > 0),
        0x29 => Property::SubscriptionIdentifierAvailable(buffer.get_u8() > 0),
        0x2A => Property::SharedSubscriptionAvailable(buffer.get_u8() > 0),
        _ => return Err(DecodeError::FormatError)
      };
      properties.push(property);
    }
    Ok(properties)
  }
  pub fn encode(buffer: &mut BytesMut, properties: &Vec<Property>) -> Result<(), EncodeError> {
    let mut content = bytes::BytesMut::new();
    for property in properties.clone().iter() {
      match property {
        Property::PayloadFormatIndicator(val) => content.put_u8(*val),
        Property::MessageExpiryInterval(val) => content.put_u32(*val),
        Property::ContentType(val) => encode_utf8(&mut content, val),
        Property::ResponseTopic(val) => encode_utf8(&mut content, val),
        Property::CorrelationData(_) => return Err(EncodeError::FormatError),
        Property::SubscriptionIdentifier(val) => variable_integer::encode(&mut content, *val)?,
        Property::SessionExpiryInterval(val) => content.put_u32(*val),
        Property::AssignedClientIdentifier(val) => encode_utf8(&mut content, val),
        Property::ServerKeepAlive(val) => content.put_u16(*val),
        Property::AuthenticationMethod(val) => encode_utf8(&mut content, val),
        Property::AuthenticationData(_) => return Err(EncodeError::FormatError),
        Property::RequestProblemInformation(val) => content.put_u8(*val),
        Property::WillDelayInterval(val) => content.put_u32(*val),
        Property::RequestResponseInformation(val) => content.put_u8(*val),
        Property::ResponseInformation(val) => encode_utf8(&mut content, val),
        Property::ServerReference(val) => encode_utf8(&mut content, val),
        Property::ReasonString(val) => encode_utf8(&mut content, val),
        Property::ReceiveMaximum(val) => content.put_u16(*val),
        Property::TopicAliasMaximum(val) => content.put_u16(*val),
        Property::TopicAlias(val) => content.put_u16(*val),
        Property::MaximumQoS(val) => content.put_u8(*val),
        Property::RetainAvailable(val) => content.put_u8(*val),
        Property::UserProperty((val1, val2)) => { 
          encode_utf8(&mut content, val1);
          encode_utf8(&mut content, val2);
        },
        Property::MaximumPacketSize(val) => content.put_u32(*val),
        Property::WildcardSubscriptionAvailable(val) => content.put_u8(*val as u8),
        Property::SubscriptionIdentifierAvailable(val) => content.put_u8(*val as u8),
        Property::SharedSubscriptionAvailable(val) => content.put_u8(*val as u8)
      };
    }

    let content_length = content.len();
    buffer.reserve(content_length);
    variable_integer::encode(buffer, content_length as u64)?;
    buffer.put(content);
    Ok(())
  }
}