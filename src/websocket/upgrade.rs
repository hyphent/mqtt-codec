use std::collections::HashMap;
use tokio_util::codec::{Decoder, Encoder};
use bytes::{BytesMut, Buf, BufMut};
use sha1::{Sha1, Digest};

use crate::{
  error::{DecodeError, EncodeError},
  utils::decode_utf8_with_length
};

fn convert_headers_to_hashmap<'a>(headers: &'a str) -> Result<HashMap<&'a str, &'a str>, DecodeError> {
  let mut hashmap = HashMap::new();

  let mut lines = headers.split("\r\n");
  while let Some(line) = lines.next() {
    let mut items = line.splitn(2, ": ");
    let key = match items.next() {
      Some(key) => key,
      None => continue
    };
    let value = match items.next() {
      Some(value) => value,
      None => continue
    };
    hashmap.insert(key, value);
  }
  Ok(hashmap)
}

pub struct WebsocketUpgradeCodec {}

impl Decoder for WebsocketUpgradeCodec {
  type Item = String;
  type Error = DecodeError;

  fn decode(&mut self, buffer: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
    let headers = decode_utf8_with_length(buffer, buffer.remaining())?;
    let headers = convert_headers_to_hashmap(&headers)?;

    if headers.get("Upgrade") != Some(&"websocket") {
      return Err(DecodeError::FormatError);
    }

    if headers.get("Connection") != Some(&"Upgrade") {
      return Err(DecodeError::FormatError);
    }

    if headers.get("Sec-WebSocket-Protocol") != Some(&"mqtt") {
      return Err(DecodeError::FormatError);
    }

    let websocket_key = match headers.get("Sec-WebSocket-Key") {
      Some(key) => key,
      None => return Err(DecodeError::FormatError)
    };

    let mut hasher = Sha1::new();
    hasher.update(websocket_key);
    hasher.update("258EAFA5-E914-47DA-95CA-C5AB0DC85B11");
    
    Ok(Some(base64::encode(hasher.finalize())))
  }
}

impl Encoder<String> for WebsocketUpgradeCodec {
  type Error = EncodeError;
  fn encode(&mut self, websocket_key: String, buffer: &mut BytesMut) -> Result<(), Self::Error> {
    let response = format!(
      "HTTP/1.1 101 Switching Protocols\r\n\
       Upgrade: websocket\r\n\
       Connection: Upgrade\r\n\
       Sec-WebSocket-Protocol: mqtt\r\n\
       Sec-WebSocket-Accept: {}\r\n\r\n",
      websocket_key
    );

    buffer.put_slice(response.as_bytes());
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use bytes::BytesMut;
  use super::*;

  #[test]
  fn codec_upgrader() {
    let mut buffer = BytesMut::new();
    let request = "GET /chat HTTP/1.1\r\n\
      Host: server.example.com\r\n\
      Upgrade: websocket\r\n\
      Connection: Upgrade\r\n\
      Sec-WebSocket-Key: x3JJHMbDL1EzLkh9GBhXDw==\r\n\
      Sec-WebSocket-Protocol: mqtt\r\n\
      Sec-WebSocket-Version: 13\r\n\
      Origin: http://example.com\r\n\r\n";
    buffer.put_slice(request.as_bytes());

    let mut codec = WebsocketUpgradeCodec {};
    let websocket_key = codec.decode(&mut buffer).unwrap().unwrap();
    assert_eq!("HSmrc0sMlYUkAGmm5OPpG2HaGWk=", websocket_key);

    let mut buffer = BytesMut::new();
    codec.encode(websocket_key, &mut buffer).unwrap();

    let response = "HTTP/1.1 101 Switching Protocols\r\n\
      Upgrade: websocket\r\n\
      Connection: Upgrade\r\n\
      Sec-WebSocket-Protocol: mqtt\r\n\
      Sec-WebSocket-Accept: HSmrc0sMlYUkAGmm5OPpG2HaGWk=\r\n\r\n";

    assert_eq!(&buffer[..], response.as_bytes());
  }
}
