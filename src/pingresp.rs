use bytes::BytesMut;

use crate::{
  error::{EncodeError, DecodeError},
  types::DecodedPacket
};

#[derive(Clone, Debug, PartialEq)]
pub struct PingRespPacket {}

impl super::types::Encode for PingRespPacket {
  fn encode(&self, _buffer: &mut BytesMut) -> Result<(), EncodeError> {
    Ok(())
  }
}

impl PingRespPacket {
  pub fn decode(_buffer: &mut BytesMut) -> Result<DecodedPacket, DecodeError> {
    Ok(DecodedPacket::PingResp(PingRespPacket {}))
  }
}

#[cfg(test)]
mod tests {
  use bytes::BytesMut;
  use crate::types::{Encode, DecodedPacket};
  use super::*;

  #[test]
  fn codec_test() {
    let packet = PingRespPacket {};

    let packet2 = packet.clone();
    let mut buffer = BytesMut::new();
    packet.encode(&mut buffer).unwrap();

    let packet = PingRespPacket::decode(&mut buffer).unwrap();

    assert_eq!(DecodedPacket::PingResp(packet2), packet);
  }
}