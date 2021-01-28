use bytes::{BytesMut, Buf, BufMut};

use crate::{
  error::{EncodeError, DecodeError},
  types::DecodedPacket,
  property::Property,
  utils::{decode_utf8, encode_utf8, get_remaining_length}
};

#[derive(Clone, Debug, PartialEq)]
pub struct SubscribePacket {
  pub packet_id: u16,
  pub subscriptions: Vec<SubscriptionConfig>,
  pub properties: Vec<Property>
}

#[derive(Clone, Debug, PartialEq)]
pub struct SubscriptionConfig {
  pub topic: String,
  pub retain_handling: u8,
  pub rap: bool,
  pub nl: bool,
  pub qos: u8
}

impl super::types::Encode for SubscribePacket {
  fn encode(&self, buffer: &mut BytesMut) -> Result<(), EncodeError> {
    buffer.put_u16(self.packet_id);

    Property::encode(buffer, &self.properties)?;

    for subscription in &self.subscriptions {
      encode_utf8(buffer, &subscription.topic);
      Self::encode_subscription_options(subscription, buffer);
    }

    Ok(())
  }
}

impl SubscribePacket {
  fn encode_subscription_options(subscription: &SubscriptionConfig, buffer: &mut BytesMut) {
    let mut subscription_options = 0;
    subscription_options += subscription.retain_handling << 4;
    
    if subscription.rap {
      subscription_options += 0b1000;
    }

    if subscription.nl {
      subscription_options += 0b100
    }

    subscription_options += subscription.qos;
    buffer.put_u8(subscription_options);
  }

  fn decode_subscription_options(subscription_options: u8) -> Result<(u8, bool, bool, u8), DecodeError> {
    if (subscription_options & 0b11000000) >> 6 != 0 {
      return Err(DecodeError::FormatError);
    }

    let retain_handling = (subscription_options & 0b110000) >> 4;
    let rap = ((subscription_options & 0b1000) >> 3) == 1;
    let nl = ((subscription_options & 0b100) >> 2) == 1;
    let qos = subscription_options & 0b11;

    Ok((retain_handling, rap, nl, qos))
  }

  pub fn decode(buffer: &mut BytesMut, remaining_length: usize) -> Result<DecodedPacket, DecodeError> {
    let starting_length = buffer.remaining();
    let packet_id = buffer.get_u16();

    let properties = Property::decode(buffer)?;

    let mut subscriptions = Vec::new();

    while get_remaining_length(&buffer, starting_length, remaining_length) > 0 {
      let topic = decode_utf8(buffer)?;
      let (retain_handling, rap, nl, qos) = SubscribePacket::decode_subscription_options(buffer.get_u8())?;
      subscriptions.push(SubscriptionConfig{
        topic,
        retain_handling,
        rap,
        nl,
        qos
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

#[cfg(test)]
mod tests {
  use bytes::BytesMut;
  use crate::types::{Encode, DecodedPacket};
  use super::*;

  #[test]
  fn codec_test() {
    let subscription = SubscriptionConfig {
      topic: "test".to_owned(),
      retain_handling: 1,
      rap: false,
      nl: false,
      qos: 1
    };

    let packet = SubscribePacket {
      packet_id: 32,
      subscriptions: vec![subscription],
      properties: vec![]
    };

    let packet2 = packet.clone();
    let mut buffer = BytesMut::new();
    packet.encode(&mut buffer).unwrap();

    let remaining_length = buffer.remaining();
    let packet = SubscribePacket::decode(&mut buffer, remaining_length).unwrap();

    assert_eq!(DecodedPacket::Subscribe(packet2), packet);
  }
}