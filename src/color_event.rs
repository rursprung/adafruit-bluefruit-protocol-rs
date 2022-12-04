use super::ProtocolParseError;

/// Represents a color event from the protocol.
#[derive(PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ColorEvent {
    red: u8,
    green: u8,
    blue: u8,
}

impl TryFrom<&[u8]> for ColorEvent {
    type Error = ProtocolParseError;

    /// Parse the data section of a color event.
    ///
    /// The full command is not validated here, identifying the command as a color event and CRC validation is the responsibility of the caller!
    fn try_from(input: &[u8]) -> Result<Self, Self::Error> {
        let expected_len = 3;
        if input.len() != expected_len {
            Err(ProtocolParseError::InvalidLength(expected_len, input.len()))
        } else {
            Ok(ColorEvent {
                red: input[0],
                green: input[1],
                blue: input[2],
            })
        }
    }
}

impl ColorEvent {
    pub fn red(&self) -> u8 {
        self.red
    }

    pub fn green(&self) -> u8 {
        self.green
    }

    pub fn blue(&self) -> u8 {
        self.blue
    }
}