use std::error::Error;
use std::io::{Error as IoError, ErrorKind};

use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use bytes::{Bytes, BytesMut};
use futures::TryStreamExt;
use futures_codec::{Decoder, Encoder, Framed, LengthCodec};

use wireboard::{ClipboardData, Command, WireError};

pub struct WireCodec(LengthCodec);

impl Encoder for WireCodec {
    type Item = Command;
    type Error = WireError;

    fn encode(&mut self, src: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let bytes = Bytes::from(wireboard::encode(&src)?);
        self.0
            .encode(bytes, dst)
            .map_err(|_| WireError::FailedToEncode)
    }
}

impl Decoder for WireCodec {
    type Item = Command;
    type Error = WireError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        match self.0.decode(src)? {
            Some(bytes) => match wireboard::decode(&bytes) {
                Ok(cmd) => Ok(Some(cmd)),
                Err(e) => Err(WireError::FailedToEncode),
            },
            None => Ok(None),
        }
    }
}

async fn process(stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut framed = Framed::new(stream, WireCodec(LengthCodec));
    while let Some(packet) = framed.try_next().await? {
        dbg!(packet);
    }

    Ok(())
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut listener = TcpListener::bind("0.0.0.0:5555").await?;
    println!("Sever started");
    loop {
        let (stream, _) = listener.accept().await?;
        async_std::task::spawn({
            async {
                if let Err(er) = process(stream).await {
                    eprintln!("{}", er.to_string());
                }
            }
        });
    }

    Ok(())
}
