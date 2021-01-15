use bytes::BytesMut;

use super::error::EncodeError;
use super::types::DecodedPacket;

pub struct PingRespPacket {}

impl super::types::Encode for PingRespPacket {
  fn encode(&self, _buffer: &mut BytesMut) -> Result<(), EncodeError> {
    Ok(())
  }
}

impl PingRespPacket {
  pub fn decode() -> DecodedPacket {
    DecodedPacket::PingResp(PingRespPacket {})
  }
}