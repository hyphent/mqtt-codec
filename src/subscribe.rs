use bytes::{BytesMut, Buf};

use super::error::{DecodeError};
use super::types::DecodedPacket;
use super::property::Property;

#[derive(Clone, Debug)]
pub struct SubscribePacket {
  pub packet_id: u16,
  pub subscriptions: Vec<SubscriptionConfig>,
  pub properties: Vec<Property>
}

#[derive(Clone, Debug)]
pub struct SubscriptionConfig {
  pub topic_name: String,
  pub retain: bool,
  pub rap: bool,
  pub nl: bool,
  pub qos: u8
}

impl SubscribePacket {
  fn parse_subscription_options(subscription_options: u8) -> Result<(bool, bool, bool, u8), DecodeError> {
    if (subscription_options & 0b11000000) >> 6 != 0 {
      return Err(DecodeError::FormatError);
    }

    let retain = ((subscription_options & 0b110000) >> 4) == 1;
    let rap = ((subscription_options & 0b1000) >> 3) == 1;
    let nl = ((subscription_options & 0b100) >> 2) == 1;
    let qos = subscription_options & 0b11;

    Ok((retain, rap, nl, qos))
  }

  pub fn decode(buffer: &mut BytesMut, remaining_length: usize) -> Result<DecodedPacket, DecodeError> {
    let starting_length = buffer.remaining();
    let packet_id = buffer.get_u16();

    let properties = Property::decode(buffer)?;

    let mut subscriptions = Vec::new();

    while super::utils::get_remaining_length(&buffer, starting_length, remaining_length) > 0 {
      let topic_name = super::utils::decode_utf8(buffer)?;
      let (retain, rap, nl, qos) = SubscribePacket::parse_subscription_options(buffer.get_u8())?;
      subscriptions.push(SubscriptionConfig{
        topic_name: topic_name,
        retain: retain,
        rap: rap,
        nl: nl,
        qos: qos
      });
    }

    let packet = SubscribePacket {
      packet_id: packet_id,
      subscriptions: subscriptions,
      properties:properties
    };

    Ok(DecodedPacket::Subscribe(packet))
  }
}
