use bincode::Error as BinCodeError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Command {
    Set(ClipboardData),
    Sync,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClipboardData {
    pub value: String,
}

#[derive(Error, Debug)]
pub enum WireError {
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    BinCodeError(#[from] BinCodeError),
    #[error("Failed to encode data")]
    FailedToEncode,
    #[error("Failed to decode data")]
    FailedToDecode,
}

pub fn decode(data: &[u8]) -> Result<Command, WireError> {
    let cmd = bincode::deserialize(data)?;
    Ok(cmd)
}

pub fn encode(cmd: &Command) -> Result<Vec<u8>, WireError> {
    let vec = bincode::serialize(cmd)?;
    Ok(vec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let cmd = Command::Set(ClipboardData {
            value: "hi ".to_string(),
        });
    }
}
