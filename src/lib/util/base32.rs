use data_encoding::{DecodeError, BASE32_NOPAD};

pub fn encode(input: &str) -> String {
    let encoded = BASE32_NOPAD.encode(input.as_bytes()).to_lowercase();

    encoded
}

pub fn decode(input: &str) -> Result<String, DecodeError> {
    let input = input.to_uppercase();

    let decoded = BASE32_NOPAD.decode(input.as_bytes())?;

    Ok(String::from_utf8(decoded).unwrap())
}
