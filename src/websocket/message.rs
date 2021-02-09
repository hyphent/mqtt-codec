use tokio_util::codec::{Decoder, Encoder};
use bytes::{BytesMut, Buf, BufMut};

use crate::{
  error::{DecodeError, EncodeError},
  types::DecodedPacket,
  codec::MQTTCodec
};

pub struct WebsocketCodec {}

impl Decoder for WebsocketCodec {
  type Item = DecodedPacket;
  type Error = DecodeError;

  fn decode(&mut self, buffer: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
    let mut read_buffer = buffer.clone();

    let mut mqtt_buffer = BytesMut::new();
    let mut fin = false;
    
    while read_buffer.remaining() > 0 {
      while !fin {
        if read_buffer.remaining() < 2 {
          return Ok(None);
        }
    
        let first_byte = read_buffer.get_u8();
        fin = (first_byte & 0b10000000) == 0b10000000;
        let opcode = first_byte & 0b1111;
  
        if first_byte & 0b01110000 != 0 || opcode != 2 {
          return Err(DecodeError::FormatError);
        }
  
        let second_byte = read_buffer.get_u8();
        let mask = (second_byte & 0b10000000) == 0b10000000;
        let mut payload_length = (second_byte & 0b1111111) as usize;
        
        if payload_length == 126 {
          payload_length = read_buffer.get_u16() as usize;
        } else if payload_length > 126 {
          payload_length = read_buffer.get_u64() as usize;
        }
  
        let mask_key = match mask {
          true => {
            Some([read_buffer.get_u8(), read_buffer.get_u8(), read_buffer.get_u8(), read_buffer.get_u8()])
          },
          false => None
        };

    
        if read_buffer.remaining() < payload_length {
          return Ok(None);
        }
    
        let mut message = vec![0; payload_length];
        
        if let Some(mask_key) = mask_key {
          for i in 0..payload_length {
            message[i] = read_buffer[i] ^ mask_key[i % 4]
          }
        }

        read_buffer.advance(payload_length);

        mqtt_buffer.reserve(payload_length);
        mqtt_buffer.put_slice(&message);
      }
      
      let mut codec = MQTTCodec {};
      if let Ok(Some(packet)) = codec.decode(&mut mqtt_buffer) {
        buffer.advance(buffer.remaining() - read_buffer.remaining());
        return Ok(Some(packet));
      }
      fin = false;
    }
    Ok(None)
  }
}

impl Encoder<DecodedPacket> for WebsocketCodec {
  type Error = EncodeError;
  fn encode(&mut self, packet: DecodedPacket, buffer: &mut BytesMut) -> Result<(), Self::Error> {
    let mut codec = MQTTCodec {};
    let mut mqtt_buffer = BytesMut::new();
    codec.encode(packet, &mut mqtt_buffer)?;
    
    buffer.put_u8(0b10000010);
    let payload_length = mqtt_buffer.len();
    if payload_length < 126 {
      buffer.put_u8(payload_length as u8);
    } else if payload_length < 65536 {
      buffer.put_u8(126);
      buffer.put_u16(payload_length as u16);
    } else {
      buffer.put_u64(payload_length as u64);
    }

    buffer.put_slice(&mqtt_buffer[..]);
    Ok(())
  }
}
