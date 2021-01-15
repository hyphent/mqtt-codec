use bytes::BytesMut;

use super::error::EncodeError;
use super::types::DecodedPacket;

pub struct PingReqPacket {}

impl super::types::Encode for PingReqPacket {
  fn encode(&self, _buffer: &mut BytesMut) -> Result<(), EncodeError> {
    Ok(())
  }
}

impl PingReqPacket {
  pub fn decode() -> DecodedPacket {
    DecodedPacket::PingReq(PingReqPacket {})
  }
}