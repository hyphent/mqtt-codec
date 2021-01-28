use bytes::BytesMut;

use crate::{
  error::{EncodeError, DecodeError},
  types::DecodedPacket
};

#[derive(Clone, Debug, PartialEq)]
pub struct PingReqPacket {}

impl super::types::Encode for PingReqPacket {
  fn encode(&self, _buffer: &mut BytesMut) -> Result<(), EncodeError> {
    Ok(())
  }
}

impl PingReqPacket {
  pub fn decode(_buffer: &mut BytesMut) -> Result<DecodedPacket, DecodeError> {
    Ok(DecodedPacket::PingReq(PingReqPacket {}))
  }
}


#[cfg(test)]
mod tests {
  use bytes::BytesMut;
  use crate::types::{Encode, DecodedPacket};
  use super::*;

  #[test]
  fn codec_test() {
    let packet = PingReqPacket {};

    let packet2 = packet.clone();
    let mut buffer = BytesMut::new();
    packet.encode(&mut buffer).unwrap();

    let packet = PingReqPacket::decode(&mut buffer).unwrap();

    assert_eq!(DecodedPacket::PingReq(packet2), packet);
  }
}