use windows::{
    core::HSTRING,
    Security::Cryptography::CryptographicBuffer,
    Storage::Streams::{DataReader, IBuffer},
};

// dest is out parameter
fn mut8_buffer(buffer: &IBuffer, out: &mut [u8]) -> windows::core::Result<()> {
    let reader: DataReader = DataReader::FromBuffer(buffer)?;
    // Read bytes into the array
    // This now contains our buffer
    return reader.ReadBytes(out);
}

fn vec8_buffer(buffer: &IBuffer) -> windows::core::Result<Vec<u8>> {
    let buffer_length = buffer.Length()?;

    // Create an array of the correct size
    let mut temp: Vec<u8> = vec![0; buffer_length as usize];

    mut8_buffer(buffer, temp.as_mut())?;
    Ok(temp)
}

fn stringify_buffer(buffer: &IBuffer) -> windows::core::Result<String> {
    // Create an array of the correct size
    let temp = vec8_buffer(buffer)?;

    use windows::core::Error;
    use windows::Win32::Foundation::E_FAIL;
    // Convert to UTF-8 String
    return String::from_utf8(temp)
        // Map Error to windows core Error
        // Which we depend on
        .map_err(|_| -> Error { Error::new(E_FAIL, "Unable to convert".into()) });
}

pub fn decode(base64_string: &str) -> windows::core::Result<String> {
    // Decoded the string from Base64 to binary.
    let base64_string: HSTRING = base64_string.into();
    let buffer = CryptographicBuffer::DecodeFromBase64String(base64_string)?;
    return stringify_buffer(&buffer);
}

pub fn decode_as_mut8(base64_string: &str, dest: &mut [u8]) -> windows::core::Result<()> {
    // Decoded the string from Base64 to binary.
    let base64_string: HSTRING = base64_string.into();
    let buffer = CryptographicBuffer::DecodeFromBase64String(base64_string)?;
    return mut8_buffer(&buffer, dest);
}

pub fn decode_as_vecu8(base64_string: &str) -> windows::core::Result<Vec<u8>> {
    // Decoded the string from Base64 to binary.
    let base64_string: HSTRING = base64_string.into();
    let buffer = CryptographicBuffer::DecodeFromBase64String(base64_string)?;
    return vec8_buffer(&buffer);
}

pub fn encode(text: &str) -> windows::core::Result<String> {
    let text = text.as_bytes();
    let buffer: IBuffer = CryptographicBuffer::CreateFromByteArray(text)?;
    let base64_string: HSTRING = CryptographicBuffer::EncodeToBase64String(&buffer)?;
    let base64_string: String = base64_string.to_string();
    Ok(base64_string)
}

#[cfg(test)]
mod tests {
    use crate::decode;
    use crate::encode;

    #[test]
    fn test_base64_encode_decode() -> windows::core::Result<()> {
        let text = "What's up?😊🥘🍲🚩🏁🚀👌😁👍😍❤️💕🎶";
        let encoded_base64 = encode(&text)?;
        let decoded_text = decode(&encoded_base64)?;
        // If it worked, text and decoded_text would be the same
        println!("Original: {}\nDecoded: {}", text, decoded_text);
        assert_eq!(text, decoded_text);

        Ok(())
    }

    #[test]
    fn test_base64_encode_check_decode() -> windows::core::Result<()> {
        let a = "hello world";
        let b = "aGVsbG8gd29ybGQ=";

        assert_eq!(encode(a)?, b);
        assert_eq!(a, &decode(b)?[..]);
        Ok(())
    }
}
