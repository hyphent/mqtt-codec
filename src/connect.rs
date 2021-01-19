use bytes::{BytesMut, Buf};

use super::types::DecodedPacket;
use super::utils::decode_utf8;
use super::error::DecodeError;
use super::property::Property;

#[derive(Clone, Debug)]
pub struct ConnectPacket {
  pub client_id: String,
  pub clean_start: bool,

  pub will_config: Option<WillConfig>,

  pub keep_alive: u16,

  pub username: Option<String>,
  pub password: Option<String>,

  pub properties: Vec<Property>
}

#[derive(Clone, Debug)]
pub struct WillConfig {
  pub topic_name: String,
  pub payload: String,
  pub retain: bool,
  pub qos: u8
}

impl ConnectPacket {
  fn check_protocol(buffer: &mut BytesMut) -> Result<(), DecodeError> {
    let protocol_name = decode_utf8(buffer).map_err(|_| DecodeError::ProtocolNotSupportedError)?;

    if protocol_name != "MQTT" {
      return Err(DecodeError::ProtocolNotSupportedError);
    }
    
    let protocol_version = buffer.get_u8();
    if protocol_version != 5 {
      return Err(DecodeError::ProtocolNotSupportedError);
    }
    Ok(())
  }

  fn parse_connect_flags(connect_flags: u8) -> Result<(bool, bool, bool, u8, bool, bool), DecodeError> {
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
    ConnectPacket::check_protocol(buffer)?;

    let (username_flag, password_flag, will_retain, will_qos, will_flag, clean_start) = ConnectPacket::parse_connect_flags(buffer.get_u8())?;

    let keep_alive = buffer.get_u16();

    let properties = Property::decode(buffer)?;

    let client_id = decode_utf8(buffer)?;

    let will_config = match will_flag {
      true => {
        // TODO pass will properties
        let _will_properties = Property::decode(buffer)?;

        let topic_name = decode_utf8(buffer)?;
        let payload = decode_utf8(buffer)?;

        Some(WillConfig {
          topic_name: topic_name,
          payload: payload,
          retain: will_retain,
          qos: will_qos
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
      client_id: client_id,
      clean_start: clean_start,

      will_config: will_config,
  
      keep_alive: keep_alive,

      username: username,
      password: password,

      properties: properties
    };
    
    Ok(DecodedPacket::Connect(packet))
  }
}
