fn encode_decode_string() -> windows::core::Result<()> {
    use win_base64::decode;
    use win_base64::encode;
    let text = "What's up?😊🥘🍲🚩🏁🚀👌😁👍😍❤️💕🎶";
    let base64_string = encode(&text)?;
    let decoded_text = decode(&base64_string)?;
    // If it worked, text and decoded_text would be the same
    println!("Original: {}", text);
    println!("Decoded : {}", decoded_text);
    Ok(())
}

fn encode_decode_mut_u8() -> windows::core::Result<()> {
    use win_base64::encode;
    let text = "What's up?";
    let base64_string = encode(&text)?;
    let mut decoded_arr = vec![0u8; 10];
    use win_base64::decode_as_mut8;
    decode_as_mut8(&base64_string, &mut decoded_arr)?;
    // If it worked, text and decoded_text would be the same
    println!("Original: {}", text);
    println!("Decoded : {:?}", decoded_arr);
    Ok(())
}

fn encode_decode_vecu8() -> windows::core::Result<()> {
    use win_base64::encode;
    let text = "What's up?";
    let base64_string = encode(&text)?;
    use win_base64::decode_as_vecu8;
    let decoded_vec = decode_as_vecu8(&base64_string)?;
    // If it worked, text and decoded_text would be the same
    println!("Original: {}", text);
    println!("Decoded : {:?}", decoded_vec);
    Ok(())
}

fn main() -> windows::core::Result<()> {
    println!("String Encode Decode");
    encode_decode_string()?;
    println!("Encode Decode as mut u8");
    encode_decode_mut_u8()?;
    println!("Encode Decode as Vec u8");
    encode_decode_vecu8()?;
    Ok(())
}
