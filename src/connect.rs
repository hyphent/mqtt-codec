use bytes::{BytesMut, Buf, BufMut};

use crate::{
  error::{EncodeError, DecodeError},
  types::DecodedPacket,
  property::Property,
  utils::{decode_utf8, encode_utf8}
};

#[derive(Clone, Debug, PartialEq)]
pub struct ConnectPacket {
  pub client_id: String,
  pub clean_start: bool,

  pub will_config: Option<WillConfig>,

  pub keep_alive: u16,

  pub username: Option<String>,
  pub password: Option<String>,

  pub properties: Vec<Property>
}

#[derive(Clone, Debug, PartialEq)]
pub struct WillConfig {
  pub topic: String,
  pub payload: String,
  pub retain: bool,
  pub qos: u8,
  pub properties: Vec<Property>
}

const PROTOCOL_NAME: &'static str = "MQTT";
const PROTOCOL_VERSION: u8 = 5;

impl super::types::Encode for ConnectPacket {
  fn encode(&self, buffer: &mut BytesMut) -> Result<(), EncodeError> {
    encode_utf8(buffer, PROTOCOL_NAME);
    buffer.put_u8(PROTOCOL_VERSION);

    self.encode_connect(buffer);

    buffer.put_u16(self.keep_alive);

    Property::encode(buffer, &self.properties)?;

    encode_utf8(buffer, &self.client_id);

    if let Some(will_config) = &self.will_config {
      Property::encode(buffer, &will_config.properties)?;
      encode_utf8(buffer, &will_config.topic);
      encode_utf8(buffer, &will_config.payload);
    }

    if let Some(username) = &self.username {
      encode_utf8(buffer, &username);
    }

    if let Some(password) = &self.password {
      encode_utf8(buffer, &password);
    }

    Ok(())
  }
}


impl ConnectPacket {
  fn encode_connect(&self, buffer: &mut BytesMut) {
    let mut connect_flags = 0;
    if let Some(_) = self.username {
      connect_flags += 0b10000000; 
    }
    if let Some(_) = self.password {
      connect_flags += 0b1000000;
    }

    if let Some(will_config) = &self.will_config {
      connect_flags += 0b100;

      if will_config.retain {
        connect_flags += 0b100000;
      }

      connect_flags += will_config.qos << 3;
    }

    if self.clean_start {
      connect_flags += 0b10;
    }

    buffer.put_u8(connect_flags);
  }

  fn check_protocol(buffer: &mut BytesMut) -> Result<(), DecodeError> {
    let protocol_name = decode_utf8(buffer).map_err(|_| DecodeError::ProtocolNotSupportedError)?;

    if protocol_name != PROTOCOL_NAME {
      return Err(DecodeError::ProtocolNotSupportedError);
    }
    
    let protocol_version = buffer.get_u8();
    if protocol_version != PROTOCOL_VERSION {
      return Err(DecodeError::ProtocolNotSupportedError);
    }
    Ok(())
  }

  fn decode_connect_flags(connect_flags: u8) -> Result<(bool, bool, bool, u8, bool, bool), DecodeError> {
    if connect_flags & 0b1 != 0 {
      return Err(DecodeError::FormatError);
    }

    let username_flag = ((connect_flags & 0b10000000) >> 7) == 1;
    let password_flag = ((connect_flags & 0b1000000) >> 6) == 1;
    let will_retain = ((connect_flags & 0b100000) >> 5) == 1;
    let will_qos = (connect_flags & 0b11000) >> 3;
    let will_flag = ((connect_flags & 0b100) >> 2) == 1;
    let clean_start = ((connect_flags & 0b10) >> 1) == 1;

    Ok((username_flag, password_flag, will_retain, will_qos, will_flag, clean_start))
  }

  pub fn decode(buffer: &mut BytesMut) -> Result<DecodedPacket, DecodeError> {
    Self::check_protocol(buffer)?;

    let (username_flag, password_flag, will_retain, will_qos, will_flag, clean_start) = Self::decode_connect_flags(buffer.get_u8())?;

    let keep_alive = buffer.get_u16();

    let properties = Property::decode(buffer)?;

    let client_id = decode_utf8(buffer)?;

    let will_config = match will_flag {
      true => {
        // TODO pass will properties
        let will_properties = Property::decode(buffer)?;

        let topic = decode_utf8(buffer)?;
        let payload = decode_utf8(buffer)?;

        Some(WillConfig {
          topic,
          payload,
          retain: will_retain,
          qos: will_qos,
          properties: will_properties
        })
      }
      false => None
    };

    let mut username: Option<String> = None;
    if username_flag {
      username = Some(decode_utf8(buffer)?);
    }

    let mut password: Option<String> = None;
    if password_flag {
      password = Some(decode_utf8(buffer)?);
    }

    let packet = ConnectPacket {
      client_id,
      clean_start,
      will_config,
      keep_alive,
      username,
      password,
      properties
    };
    
    Ok(DecodedPacket::Connect(packet))
  }
}


#[cfg(test)]
mod tests {
  use bytes::BytesMut;
  use crate::types::{Encode, DecodedPacket};
  use super::*;

  #[test]
  fn codec_test() {
    let packet = ConnectPacket {
      client_id: "test".to_owned(),
      clean_start: true,
      will_config: Some(WillConfig {
        topic: "topic".to_owned(),
        payload: "payload".to_owned(),
        retain: false,
        qos: 1,
        properties: vec![]
      }),
      keep_alive: 20,
      username: Some("username".to_owned()),
      password: Some("password".to_owned()),
      properties: vec![]
    };

    let packet2 = packet.clone();
    let mut buffer = BytesMut::new();
    packet.encode(&mut buffer).unwrap();

    let packet = ConnectPacket::decode(&mut buffer).unwrap();

    assert_eq!(DecodedPacket::Connect(packet2), packet);
  }
}
